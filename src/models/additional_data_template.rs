use crate::{
    TLV,
    models::{SubTags, Tags},
};
use std::fmt::Write;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct AdditionalDataTemplate {
    pub bill_number: Option<String>,
    pub store_label: Option<String>,
    pub terminal_label: Option<String>,
    pub mobile_number: Option<String>,
    pub purpose_of_transaction: Option<String>,
}

impl AdditionalDataTemplate {
    #[inline]
    pub fn apply_sub_tag(&mut self, sub_tag: &SubTags, value: &str) {
        if value.len() > sub_tag.max_length() {
            return;
        }
        match sub_tag {
            SubTags::BillNumber => self.bill_number = Some(value.to_string()),
            SubTags::MobileNumber => self.mobile_number = Some(value.to_string()),
            SubTags::StoreLabel => self.store_label = Some(value.to_string()),
            SubTags::TerminalLabel => self.terminal_label = Some(value.to_string()),
            SubTags::PurposeOfTransaction => self.purpose_of_transaction = Some(value.to_string()),
            _ => {}
        }
    }

    pub fn from_string(value: &str) -> std::io::Result<Self> {
        let mut additional_data_template = AdditionalDataTemplate {
            bill_number: None,
            store_label: None,
            terminal_label: None,
            mobile_number: None,
            purpose_of_transaction: None,
        };

        let mut slice = value;

        while !slice.is_empty() {
            let tag: u8 = slice[0..2].parse().map_err(|_| {
                Error::new(
                    ErrorKind::InvalidData,
                    format!("Tag {} can't be parsed", &slice[0..2]),
                )
            })?;

            let length: usize = slice[2..4].parse().map_err(|_| {
                Error::new(
                    ErrorKind::InvalidData,
                    format!("Error parsing length for tag {}", tag),
                )
            })?;

            let value_end = 4 + length;

            if value_end > slice.len() {
                return Err(Error::new(ErrorKind::InvalidData, "Length is invalid"));
            }

            let data_value: &str = &slice[4..value_end];

            let sub_tag =
                SubTags::from_code(Tags::AdditionalDataTemplate, tag).ok_or_else(|| {
                    Error::new(
                        ErrorKind::InvalidData,
                        format!("Unknown AdditionalDataTemplate sub-tag {}", tag),
                    )
                })?;

            sub_tag
                .validate_length(data_value)
                .map_err(|msg| Error::new(ErrorKind::InvalidData, msg))?;

            additional_data_template.apply_sub_tag(&sub_tag, data_value);

            slice = &slice[value_end..];
        }

        Ok(additional_data_template)
    }
}

impl TLV for AdditionalDataTemplate {
    fn to_tlv(&self, temp: &mut String, buffer: &mut String) -> Result<(), std::io::Error> {
        #[inline]
        fn write_field(
            temp: &mut String,
            sub_tag: SubTags,
            value: &Option<String>,
        ) -> Result<(), std::io::Error> {
            if let Some(val) = value {
                sub_tag
                    .validate_length(val)
                    .map_err(|msg| Error::new(ErrorKind::InvalidData, msg))?;
                write!(temp, "{:02}{:02}{}", sub_tag.code(), val.len(), val)
                    .map_err(|e| Error::new(ErrorKind::Other, e))?;
            }
            Ok(())
        }

        write_field(temp, SubTags::BillNumber, &self.bill_number)?;
        write_field(temp, SubTags::MobileNumber, &self.mobile_number)?;
        write_field(temp, SubTags::StoreLabel, &self.store_label)?;
        write_field(temp, SubTags::TerminalLabel, &self.terminal_label)?;
        write_field(
            temp,
            SubTags::PurposeOfTransaction,
            &self.purpose_of_transaction,
        )?;

        write!(
            buffer,
            "{:02}{:02}{}",
            Tags::AdditionalDataTemplate.code(),
            temp.len(),
            temp
        )
        .map_err(|e| Error::new(ErrorKind::Other, e))?;
        temp.clear();
        Ok(())
    }
}
