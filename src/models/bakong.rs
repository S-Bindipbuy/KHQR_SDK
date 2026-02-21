use crate::TLV;
use crate::models::{
    AdditionalDataField, AdditionalDataTemplate, CountryCode, IndivualInformation, MerchantCity,
    MerchantInformation, MerchantInformationLanguageTemplate, MerchantType, PointOfInitialMethod,
    Tags, TransactionAmount, TransactionCurrency, crc16_ccitt,
};
use crate::write_tlv;
use std::fmt::Write;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct Bakong {
    pub qr_type: PointOfInitialMethod,
    pub merchant_type: MerchantType,
    pub merchant_category_code: Option<String>,
    pub merchant_name: String,
    pub merchant_city: Option<MerchantCity>,
    pub additional_data_template: Option<AdditionalDataTemplate>,
    pub unionpay_merchant: Option<String>,
    pub merchant_information_language_template: Option<MerchantInformationLanguageTemplate>,
}

impl Bakong {
    pub fn decode_qr(qr: &str) -> Result<Bakong, std::io::Error> {
        if qr.len() > 256 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "QR string length exceeds maximum allowed",
            ));
        }

        let mut slice = qr;

        let mut is_static: Option<bool> = None;
        let mut currency: Option<TransactionCurrency> = None;
        let mut amount_raw: Option<String> = None;
        let mut merchant_category_code = None;
        let mut merchant_name = String::new();
        let mut merchant_city: Option<MerchantCity> = None;
        let mut additional_data_template: Option<AdditionalDataTemplate> = None;
        let mut additional_data_field: Option<AdditionalDataField> = None;
        let mut unionpay_merchant: Option<String> = None;
        let mut merchant_information_language_template: Option<
            MerchantInformationLanguageTemplate,
        > = None;
        let mut merchant_type: Option<MerchantType> = None;

        while slice.len() >= 4 {
            let raw_tag: u8 = slice[0..2].parse().map_err(|_| {
                Error::new(
                    ErrorKind::InvalidData,
                    format!("Invalid tag code '{}'", &slice[0..2]),
                )
            })?;

            let tag = Tags::from_code(raw_tag).ok_or_else(|| {
                Error::new(ErrorKind::InvalidData, format!("Unknown tag '{}'", raw_tag))
            })?;

            let length: usize = slice[2..4].parse().map_err(|_| {
                Error::new(
                    ErrorKind::InvalidData,
                    format!("Invalid length '{}'", &slice[2..4]),
                )
            })?;

            let end = 4 + length;
            if end > slice.len() {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Declared length exceeds remaining slice",
                ));
            }

            let value = &slice[4..end];

            match tag {
                Tags::PointOfInitialMethod => {
                    is_static = PointOfInitialMethod::is_static_from_string(value);
                }
                Tags::TransactionCurrency => {
                    currency = Some(TransactionCurrency::from_string(value)?);
                }
                Tags::TransactionAmount => {
                    amount_raw = Some(value.to_string());
                }
                Tags::MerchantName => {
                    merchant_name = value.to_string();
                }
                Tags::MerchantCategoryCode => {
                    merchant_category_code = Some(value.to_string());
                }
                Tags::MerchantCity => {
                    merchant_city = MerchantCity::from_string(value);
                }
                Tags::AdditionalDataTemplate => {
                    additional_data_template = Some(AdditionalDataTemplate::from_string(value)?);
                }
                Tags::AdditionalDataField => {
                    additional_data_field = Some(AdditionalDataField::from_string(value)?);
                }
                Tags::UnionPayMerchant => {
                    unionpay_merchant = Some(value.to_string());
                }
                Tags::MerchantInformationLanguageTemplate => {
                    merchant_information_language_template =
                        Some(MerchantInformationLanguageTemplate::from_string(value)?);
                }
                Tags::MerchantAccountInfoIndividual => {
                    if merchant_type.is_some() {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            "Multiple merchant account types found (Individual + Merchant)",
                        ));
                    }
                    let individual = IndivualInformation::from_string(value)?;
                    merchant_type = Some(MerchantType::Indivual(individual));
                }
                Tags::MerchantAccountInfoMerchant => {
                    if merchant_type.is_some() {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            "Multiple merchant account types found (Merchant + Individual)",
                        ));
                    }
                    let merchant = MerchantInformation::from_string(value)?;
                    merchant_type = Some(MerchantType::Merchant(merchant));
                }
                _ => {}
            }

            slice = &slice[end..];
        }

        let qr_type = match is_static {
            Some(true) => {
                let currency = currency.ok_or_else(|| {
                    Error::new(ErrorKind::InvalidData, "Missing currency for static QR")
                })?;
                PointOfInitialMethod::Static(currency)
            }
            Some(false) => {
                let currency = currency.ok_or_else(|| {
                    Error::new(ErrorKind::InvalidData, "Missing currency for dynamic QR")
                })?;
                let amount_value = amount_raw.ok_or_else(|| {
                    Error::new(ErrorKind::InvalidData, "Missing amount for dynamic QR")
                })?;
                let amount = TransactionAmount::from_string(&currency, &amount_value)?;
                PointOfInitialMethod::Dynamic {
                    additional_data_field,
                    amount,
                }
            }
            None => PointOfInitialMethod::Static(TransactionCurrency::KHR),
        };

        let merchant_type = merchant_type.ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                "Missing merchant account information",
            )
        })?;

        Ok(Bakong {
            qr_type,
            merchant_type,
            merchant_category_code,
            merchant_name,
            merchant_city,
            additional_data_template,
            unionpay_merchant,
            merchant_information_language_template,
        })
    }

    pub fn generate_qr(&self) -> Result<String, std::io::Error> {
        let mut qr_code = String::with_capacity(255);
        let mut temp_value = String::with_capacity(99);
        let mut dynamic_additional: Option<&AdditionalDataField> = None;

        Tags::PayloadFormatIndicator
            .validate_length("01")
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        write_tlv!(&mut qr_code, (Tags::PayloadFormatIndicator.code(), "01"))
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        let poi_value = self.qr_type.value();
        Tags::PointOfInitialMethod
            .validate_length(poi_value)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        write_tlv!(&mut qr_code, (Tags::PointOfInitialMethod.code(), poi_value))
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        self.merchant_type.to_tlv(&mut temp_value, &mut qr_code)?;

        if let Some(merchant_category_code) = &self.merchant_category_code {
            Tags::MerchantCategoryCode
                .validate_length(merchant_category_code)
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
            write_tlv!(
                &mut qr_code,
                (Tags::MerchantCategoryCode.code(), merchant_category_code)
            )
            .map_err(|e| Error::new(ErrorKind::Other, e))?;
        }

        self.qr_type.write_currency(&mut qr_code)?;

        if let Some(unionpay) = &self.unionpay_merchant {
            Tags::UnionPayMerchant
                .validate_length(unionpay)
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
            write_tlv!(&mut qr_code, (Tags::UnionPayMerchant.code(), unionpay))
                .map_err(|e| Error::new(ErrorKind::Other, e))?;
        }

        if let PointOfInitialMethod::Dynamic {
            additional_data_field,
            amount,
        } = &self.qr_type
        {
            dynamic_additional = additional_data_field.as_ref();
            amount.write_amount(&mut temp_value, &mut qr_code)?;
        }

        let country_code = CountryCode::KH.code();
        Tags::CountryCode
            .validate_length(country_code)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        write_tlv!(&mut qr_code, (Tags::CountryCode.code(), country_code))
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        Tags::MerchantName
            .validate_length(&self.merchant_name)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        write_tlv!(
            &mut qr_code,
            (Tags::MerchantName.code(), &self.merchant_name)
        )
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

        let merchant_city = self
            .merchant_city
            .as_ref()
            .unwrap_or(&MerchantCity::PhnomPenh);
        let city_value = merchant_city.city();
        Tags::MerchantCity
            .validate_length(city_value)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        write_tlv!(&mut qr_code, (Tags::MerchantCity.code(), city_value))
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        if let Some(additional_data_template) = &self.additional_data_template {
            additional_data_template.to_tlv(&mut temp_value, &mut qr_code)?;
        }

        if let Some(merchant_information) = &self.merchant_information_language_template {
            merchant_information.to_tlv(&mut temp_value, &mut qr_code)?;
        }

        if let Some(additional_data) = dynamic_additional {
            additional_data.to_tlv(&mut temp_value, &mut qr_code)?;
        }

        write!(&mut qr_code, "{:02}{:02}", Tags::Crc.code(), 4)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;
        let crc = crc16_ccitt(&qr_code);
        write!(&mut qr_code, "{:04X}", crc).map_err(|e| Error::new(ErrorKind::Other, e))?;

        Ok(qr_code)
    }
}
