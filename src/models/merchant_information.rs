use crate::models::{SubTags, Tags};
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct MerchantInformation {
    pub bakong_account_identifier: String,
    pub merchant_identifier: String,
    pub acquiring_bank: String,
}

impl MerchantInformation {
    #[inline]
    pub fn apply_sub_tag(&mut self, sub_tag: &SubTags, value: &str) {
        if value.len() > sub_tag.max_length() {
            return;
        }
        match sub_tag {
            SubTags::BakongAccountIdentifier => self.bakong_account_identifier = value.to_string(),
            SubTags::MerchantIdentifier => self.merchant_identifier = value.to_string(),
            SubTags::AcquiringBank => self.acquiring_bank = value.to_string(),
            _ => {}
        }
    }

    pub fn from_string(value: &str) -> std::io::Result<Self> {
        let mut merchant_info = MerchantInformation {
            bakong_account_identifier: String::new(),
            merchant_identifier: String::new(),
            acquiring_bank: String::new(),
        };

        let mut slice = value;
        while !slice.is_empty() {
            let tag: u8 = slice[0..2].parse().map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Tag {} can't be parsed!!!", &slice[0..2]),
                )
            })?;

            let length: usize = slice[2..4].parse().map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Error length parsing in tag : {}", tag),
                )
            })?;

            let value_end = 4 + length;
            if value_end > slice.len() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Length is invalid!!!",
                ));
            }

            let data_value: &str = &slice[4..value_end];
            let sub_tag =
                SubTags::from_code(Tags::MerchantAccountInfoMerchant, tag).ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Unknown MerchantInformation sub-tag {}", tag),
                    )
                })?;

            sub_tag
                .validate_length(data_value)
                .map_err(|msg| Error::new(ErrorKind::InvalidData, msg))?;

            merchant_info.apply_sub_tag(&sub_tag, data_value);
            slice = &slice[value_end..];
        }

        if merchant_info.bakong_account_identifier.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Bakong account identifier is required but missing",
            ));
        }

        if merchant_info.merchant_identifier.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Merchant identifier is required but missing",
            ));
        }

        if merchant_info.acquiring_bank.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Acquiring bank is required but missing",
            ));
        }

        Ok(merchant_info)
    }
}
