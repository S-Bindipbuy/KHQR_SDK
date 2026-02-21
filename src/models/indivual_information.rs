use crate::models::{SubTags, Tags};
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct IndivualInformation {
    pub bakong_account_identifier: String,
    pub individual_account_information: Option<String>,
    pub acquiring_bank: Option<String>,
}

impl IndivualInformation {
    #[inline]
    pub fn apply_sub_tag(&mut self, sub_tag: &SubTags, value: &str) {
        if value.len() > sub_tag.max_length() {
            return;
        }
        match sub_tag {
            SubTags::BakongAccountIdentifier => self.bakong_account_identifier = value.to_string(),
            SubTags::IndividualAccountInformation => {
                self.individual_account_information = Some(value.to_string())
            }
            SubTags::AcquiringBank => self.acquiring_bank = Some(value.to_string()),
            _ => {}
        }
    }

    pub fn from_string(value: &str) -> std::io::Result<Self> {
        let mut indiv_info = IndivualInformation {
            bakong_account_identifier: String::new(),
            individual_account_information: None,
            acquiring_bank: None,
        };

        let mut slice = value;
        while !slice.is_empty() {
            let tag: u8 = slice[0..2].parse().map_err(|_| {
                Error::new(
                    ErrorKind::InvalidData,
                    format!("Tag {} can't be parsed!!!", &slice[0..2]),
                )
            })?;

            let length: usize = slice[2..4].parse().map_err(|_| {
                Error::new(
                    ErrorKind::InvalidData,
                    format!("Error length parsing in tag : {}", tag),
                )
            })?;

            let value_end = 4 + length;
            if value_end > slice.len() {
                return Err(Error::new(ErrorKind::InvalidData, "Length is invalid!!!"));
            }

            let data_value: &str = &slice[4..value_end];

            let sub_tag =
                SubTags::from_code(Tags::MerchantAccountInfoIndividual, tag).ok_or_else(|| {
                    Error::new(
                        ErrorKind::InvalidData,
                        format!("Unknown IndivualInformation sub-tag {}", tag),
                    )
                })?;

            sub_tag
                .validate_length(data_value)
                .map_err(|msg| Error::new(ErrorKind::InvalidData, msg))?;

            indiv_info.apply_sub_tag(&sub_tag, data_value);

            slice = &slice[value_end..];
        }

        if indiv_info.bakong_account_identifier.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Bakong account identifier is required but missing",
            ));
        }

        Ok(indiv_info)
    }
}
