use crate::{
    models::{IndivualInformation, MerchantInformation, SubTags, Tags},
    tlv::TLV,
};
use std::fmt::Write;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub enum MerchantType {
    Indivual(IndivualInformation),
    Merchant(MerchantInformation),
}

impl TLV for MerchantType {
    fn to_tlv(&self, temp: &mut String, buffer: &mut String) -> Result<(), std::io::Error> {
        let tag = match &self {
            MerchantType::Indivual(indivual_information) => {
                SubTags::BakongAccountIdentifier
                    .validate_length(&indivual_information.bakong_account_identifier)
                    .map_err(|msg| Error::new(ErrorKind::InvalidData, msg))?;
                let at_count = indivual_information
                    .bakong_account_identifier
                    .matches('@')
                    .count();
                if at_count != 1 {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Bakong account identifier must contain exactly one '@'",
                    ));
                }
                write!(
                    temp,
                    "{:02}{:02}{}",
                    SubTags::BakongAccountIdentifier.code(),
                    indivual_information.bakong_account_identifier.len(),
                    indivual_information.bakong_account_identifier
                )
                .map_err(|e| Error::new(ErrorKind::Other, e))?;

                if let Some(info) = &indivual_information.individual_account_information {
                    SubTags::IndividualAccountInformation
                        .validate_length(info)
                        .map_err(|msg| Error::new(ErrorKind::InvalidData, msg))?;
                    write!(
                        temp,
                        "{:02}{:02}{}",
                        SubTags::IndividualAccountInformation.code(),
                        info.len(),
                        info
                    )
                    .map_err(|e| Error::new(ErrorKind::Other, e))?;
                }

                if let Some(bank) = &indivual_information.acquiring_bank {
                    SubTags::AcquiringBank
                        .validate_length(bank)
                        .map_err(|msg| Error::new(ErrorKind::InvalidData, msg))?;
                    write!(
                        temp,
                        "{:02}{:02}{}",
                        SubTags::AcquiringBank.code(),
                        bank.len(),
                        bank
                    )
                    .map_err(|e| Error::new(ErrorKind::Other, e))?;
                }

                Tags::MerchantAccountInfoIndividual
            }
            MerchantType::Merchant(merchant_information) => {
                SubTags::BakongAccountIdentifier
                    .validate_length(&merchant_information.bakong_account_identifier)
                    .map_err(|msg| Error::new(ErrorKind::InvalidData, msg))?;
                let merchant_identifier = &merchant_information.bakong_account_identifier;
                let at_count = merchant_identifier.matches('@').count();
                if at_count != 1 {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Bakong account identifier must contain exactly one '@'",
                    ));
                }
                write!(
                    temp,
                    "{:02}{:02}{}",
                    SubTags::BakongAccountIdentifier.code(),
                    merchant_information.bakong_account_identifier.len(),
                    merchant_information.bakong_account_identifier
                )
                .map_err(|e| Error::new(ErrorKind::Other, e))?;

                SubTags::MerchantIdentifier
                    .validate_length(&merchant_information.merchant_identifier)
                    .map_err(|msg| Error::new(ErrorKind::InvalidData, msg))?;
                write!(
                    temp,
                    "{:02}{:02}{}",
                    SubTags::MerchantIdentifier.code(),
                    merchant_information.merchant_identifier.len(),
                    merchant_information.merchant_identifier
                )
                .map_err(|e| Error::new(ErrorKind::Other, e))?;

                SubTags::AcquiringBank
                    .validate_length(&merchant_information.acquiring_bank)
                    .map_err(|msg| Error::new(ErrorKind::InvalidData, msg))?;
                write!(
                    temp,
                    "{:02}{:02}{}",
                    SubTags::AcquiringBank.code(),
                    merchant_information.acquiring_bank.len(),
                    merchant_information.acquiring_bank
                )
                .map_err(|e| Error::new(ErrorKind::Other, e))?;

                Tags::MerchantAccountInfoMerchant
            }
        };

        write!(buffer, "{:02}{:02}{}", tag.code(), temp.len(), temp)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;
        temp.clear();
        Ok(())
    }
}
