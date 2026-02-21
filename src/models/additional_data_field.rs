use crate::models::{SubTags, Tags};
use crate::tlv::TLV;
use chrono::{DateTime, TimeZone, Utc};
use std::fmt::Write;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct AdditionalDataField {
    pub creation_timestamp: DateTime<Utc>,
    pub expiration_timestamp: DateTime<Utc>,
}

impl AdditionalDataField {
    pub fn new(expiration: DateTime<Utc>) -> Result<Self, Error> {
        let creation = Utc::now();

        if expiration < creation {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Expiration timestamp is before creation timestamp",
            ));
        }

        let creation_str = creation.timestamp_millis().to_string();
        SubTags::CreationTimestamp
            .validate_length(&creation_str)
            .map_err(|msg| Error::new(ErrorKind::InvalidData, msg))?;

        let expiration_str = expiration.timestamp_millis().to_string();
        SubTags::ExpirationTimestamp
            .validate_length(&expiration_str)
            .map_err(|msg| Error::new(ErrorKind::InvalidData, msg))?;

        Ok(Self {
            creation_timestamp: creation,
            expiration_timestamp: expiration,
        })
    }

    pub fn from_string(value: &str) -> Result<Self, Error> {
        let mut slice = value;
        let mut creation: Option<DateTime<Utc>> = None;
        let mut expiration: Option<DateTime<Utc>> = None;

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
            let sub_tag = SubTags::from_code(Tags::AdditionalDataField, tag).ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidData,
                    format!("Unknown AdditionalDataField sub-tag {}", tag),
                )
            })?;

            sub_tag
                .validate_length(&data)
                .map_err(|msg| Error::new(ErrorKind::InvalidData, msg))?;

            match sub_tag {
                SubTags::CreationTimestamp => {
                    let millis: i64 = data.parse().map_err(|_| {
                        Error::new(ErrorKind::InvalidData, "Invalid creation timestamp")
                    })?;
                    creation =
                        Some(Utc.timestamp_millis_opt(millis).single().ok_or_else(|| {
                            Error::new(ErrorKind::InvalidData, "Invalid creation timestamp value")
                        })?);
                }
                SubTags::ExpirationTimestamp => {
                    let millis: i64 = data.parse().map_err(|_| {
                        Error::new(ErrorKind::InvalidData, "Invalid expiration timestamp")
                    })?;
                    expiration =
                        Some(Utc.timestamp_millis_opt(millis).single().ok_or_else(|| {
                            Error::new(ErrorKind::InvalidData, "Invalid expiration timestamp value")
                        })?);
                }
                _ => {}
            }

            slice = &slice[end..];
        }

        let creation = creation
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Missing creation timestamp"))?;
        let expiration = expiration
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Missing expiration timestamp"))?;

        if expiration < creation {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Expiration timestamp is before creation timestamp",
            ));
        }

        if expiration < Utc::now() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Expiration timestamp is already in the past",
            ));
        }

        Ok(Self {
            creation_timestamp: creation,
            expiration_timestamp: expiration,
        })
    }
}

impl TLV for AdditionalDataField {
    fn to_tlv(&self, temp: &mut String, buffer: &mut String) -> Result<(), Error> {
        let creation_str = self.creation_timestamp.timestamp_millis().to_string();
        SubTags::CreationTimestamp
            .validate_length(&creation_str)
            .map_err(|msg| Error::new(ErrorKind::InvalidData, msg))?;
        write!(
            temp,
            "{:02}{:02}{}",
            SubTags::CreationTimestamp.code(),
            creation_str.len(),
            creation_str
        )
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

        let expiration_str = self.expiration_timestamp.timestamp_millis().to_string();
        SubTags::ExpirationTimestamp
            .validate_length(&expiration_str)
            .map_err(|msg| Error::new(ErrorKind::InvalidData, msg))?;
        write!(
            temp,
            "{:02}{:02}{}",
            SubTags::ExpirationTimestamp.code(),
            expiration_str.len(),
            expiration_str
        )
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

        write!(
            buffer,
            "{:02}{:02}{}",
            Tags::AdditionalDataField.code(),
            temp.len(),
            temp
        )
        .map_err(|e| Error::new(ErrorKind::Other, e))?;
        temp.clear();
        Ok(())
    }
}
