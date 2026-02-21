use crate::{
    models::{SubTags, Tags},
    tlv::TLV,
    write_tlv,
};
use std::fmt::Write;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct MerchantInformationLanguageTemplate {
    pub language_preference: String,
    pub merchant_name_alternate_language: String,
    pub merchant_city_alternate_language: String,
}

impl TLV for MerchantInformationLanguageTemplate {
    fn to_tlv(&self, temp: &mut String, buffer: &mut String) -> Result<(), std::io::Error> {
        SubTags::LanguagePreference
            .validate_length(&self.language_preference)
            .map_err(|msg| Error::new(ErrorKind::InvalidData, msg))?;

        SubTags::MerchantNameAlternateLanguage
            .validate_length(&self.merchant_name_alternate_language)
            .map_err(|msg| Error::new(ErrorKind::InvalidData, msg))?;

        SubTags::MerchantCityAlternateLanguage
            .validate_length(&self.merchant_city_alternate_language)
            .map_err(|msg| Error::new(ErrorKind::InvalidData, msg))?;

        write_tlv!(
            temp,
            (
                SubTags::LanguagePreference.code(),
                &self.language_preference
            )
        )
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

        write_tlv!(
            temp,
            (
                SubTags::MerchantNameAlternateLanguage.code(),
                &self.merchant_name_alternate_language
            )
        )
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

        write_tlv!(
            temp,
            (
                SubTags::MerchantCityAlternateLanguage.code(),
                &self.merchant_city_alternate_language
            )
        )
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

        write_tlv!(
            buffer,
            (Tags::MerchantInformationLanguageTemplate.code(), temp)
        )
        .map_err(|e| Error::new(ErrorKind::Other, e))?;
        temp.clear();
        Ok(())
    }
}

impl MerchantInformationLanguageTemplate {
    pub fn from_string(value: &str) -> std::io::Result<Self> {
        let mut template = MerchantInformationLanguageTemplate {
            language_preference: String::new(),
            merchant_name_alternate_language: String::new(),
            merchant_city_alternate_language: String::new(),
        };

        let mut slice = value;

        while !slice.is_empty() {
            let tag: u8 = slice[0..2]
                .parse()
                .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid sub-tag"))?;

            let length: usize = slice[2..4]
                .parse()
                .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid length"))?;

            let end = 4 + length;
            if end > slice.len() {
                return Err(Error::new(ErrorKind::InvalidData, "Length error"));
            }

            let data = &slice[4..end];
            let sub_tag = SubTags::from_code(Tags::MerchantInformationLanguageTemplate, tag)
                .ok_or_else(|| {
                    Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "Unknown MerchantInformationLanguageTemplate sub-tag {}",
                            tag
                        ),
                    )
                })?;

            sub_tag
                .validate_length(data)
                .map_err(|msg| Error::new(ErrorKind::InvalidData, msg))?;

            match sub_tag {
                SubTags::LanguagePreference => template.language_preference = data.to_string(),
                SubTags::MerchantNameAlternateLanguage => {
                    template.merchant_name_alternate_language = data.to_string()
                }
                SubTags::MerchantCityAlternateLanguage => {
                    template.merchant_city_alternate_language = data.to_string()
                }
                _ => {}
            }

            slice = &slice[end..];
        }

        if template.language_preference.is_empty()
            || template.merchant_name_alternate_language.is_empty()
            || template.merchant_city_alternate_language.is_empty()
        {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "All fields are required but missing",
            ));
        }

        Ok(template)
    }
}
