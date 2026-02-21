use super::{AdditionalDataField, TransactionAmount, TransactionCurrency};
use std::io::Error;

#[derive(Debug)]
pub enum PointOfInitialMethod {
    Static(TransactionCurrency),
    Dynamic {
        additional_data_field: Option<AdditionalDataField>,
        amount: TransactionAmount,
    },
}

impl PointOfInitialMethod {
    pub fn write_currency(&self, buffer: &mut String) -> Result<(), Error> {
        match self {
            PointOfInitialMethod::Static(transaction_currency) => {
                transaction_currency.write_currency(buffer)
            }
            PointOfInitialMethod::Dynamic {
                additional_data_field: _,
                amount,
            } => amount.write_currency(buffer),
        }
    }

    pub fn from_string(
        is_static: bool,
        currency: &TransactionCurrency,
        value: &str,
    ) -> Result<Self, Error> {
        match is_static {
            true => {
                let currency = TransactionCurrency::from_string(value)?;
                Ok(PointOfInitialMethod::Static(currency))
            }
            false => {
                let amount = TransactionAmount::from_string(&currency, value)?;
                let additional_data_field = if value.len() > 0 {
                    Some(AdditionalDataField::from_string(value)?)
                } else {
                    None
                };

                Ok(PointOfInitialMethod::Dynamic {
                    additional_data_field,
                    amount,
                })
            }
        }
    }

    pub fn value(&self) -> &'static str {
        match self {
            Self::Static(_) => "11",
            Self::Dynamic { .. } => "12",
        }
    }

    pub fn is_static(&self) -> bool {
        match self {
            Self::Static(_) => true,
            Self::Dynamic { .. } => false,
        }
    }

    pub fn is_static_from_string(value: &str) -> Option<bool> {
        match value {
            "11" => Some(true),
            "12" => Some(false),
            _ => None,
        }
    }
}
