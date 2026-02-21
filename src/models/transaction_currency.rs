use crate::models::Tags;
use crate::write_tlv;
use std::fmt::{Display, Write};
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub enum TransactionCurrency {
    KHR,
    USD,
}

#[derive(Debug)]
pub enum TransactionAmount {
    KHR(u32),
    USD(f32),
}
impl TransactionCurrency {
    pub fn write_currency(&self, buffer: &mut String) -> Result<(), Error> {
        let code = self.currency_code();

        Tags::TransactionCurrency
            .validate_length(code)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        write_tlv!(buffer, (Tags::TransactionCurrency.code(), code)).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("Failed to write currency TLV: {}", e),
            )
        })?;

        Ok(())
    }

    pub fn from_string(value: &str) -> Result<TransactionCurrency, Error> {
        match value {
            "116" => Ok(TransactionCurrency::KHR),
            "840" => Ok(TransactionCurrency::USD),
            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid currency code")),
        }
    }

    pub fn currency_code(&self) -> &'static str {
        match self {
            Self::KHR => "116",
            Self::USD => "840",
        }
    }
}

impl TransactionAmount {
    pub fn write_amount(&self, temp_value: &mut String, value: &mut String) -> Result<(), Error> {
        match self {
            Self::KHR(amount) => {
                if *amount < 100 {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("KHR amount must be at least 100, got {}", amount),
                    ));
                }
                write!(temp_value, "{}", amount).map_err(|e| {
                    Error::new(
                        ErrorKind::Other,
                        format!("Failed to format KHR amount: {}", e),
                    )
                })?;
            }
            Self::USD(amount) => {
                if *amount < 0.1 {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("USD amount must be at least 0.1, got {}", amount),
                    ));
                }
                write!(temp_value, "{:.2}", amount).map_err(|e| {
                    Error::new(
                        ErrorKind::Other,
                        format!("Failed to format USD amount: {}", e),
                    )
                })?;
            }
        };

        Tags::TransactionAmount
            .validate_length(temp_value)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        write_tlv!(value, (Tags::TransactionAmount.code(), temp_value)).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("Failed to write TransactionAmount TLV: {}", e),
            )
        })?;

        temp_value.clear();
        Ok(())
    }

    pub fn write_currency(&self, buffer: &mut String) -> Result<(), Error> {
        let code = self.currency_code();

        Tags::TransactionCurrency
            .validate_length(code)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        write_tlv!(buffer, (Tags::TransactionCurrency.code(), code)).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("Failed to write currency TLV: {}", e),
            )
        })?;

        Ok(())
    }

    pub fn from_string(
        currency: &TransactionCurrency,
        value: &str,
    ) -> Result<TransactionAmount, Error> {
        match currency {
            TransactionCurrency::KHR => {
                let parsed = value
                    .parse::<u32>()
                    .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid KHR amount"))?;

                if parsed < 100 {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("KHR amount must be at least 100, got {}", parsed),
                    ));
                }

                Ok(TransactionAmount::KHR(parsed))
            }
            TransactionCurrency::USD => {
                let parsed = value
                    .parse::<f32>()
                    .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid USD amount"))?;

                if parsed < 0.1 {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("USD amount must be at least 0.1, got {}", parsed),
                    ));
                }

                Ok(TransactionAmount::USD(parsed))
            }
        }
    }

    #[inline]
    pub fn currency_code(&self) -> &'static str {
        match self {
            Self::KHR(_) => TransactionCurrency::KHR.currency_code(),
            Self::USD(_) => TransactionCurrency::USD.currency_code(),
        }
    }
}

impl Display for TransactionAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KHR(amount) => write!(f, "{}", amount),
            Self::USD(amount) => write!(f, "{:.2}", amount),
        }
    }
}
