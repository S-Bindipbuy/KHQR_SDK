#[derive(Debug)]
pub enum Tags {
    PayloadFormatIndicator,
    PointOfInitialMethod,
    MerchantAccountInfoIndividual,
    MerchantAccountInfoMerchant,
    MerchantCategoryCode,
    TransactionCurrency,
    TransactionAmount,
    CountryCode,
    MerchantName,
    MerchantCity,
    AdditionalDataTemplate,
    AdditionalDataField,
    Crc,
    UnionPayMerchant,
    MerchantInformationLanguageTemplate,
}

impl Tags {
    pub fn code(&self) -> u8 {
        match self {
            Self::PayloadFormatIndicator => 0,
            Self::PointOfInitialMethod => 1,
            Self::MerchantAccountInfoIndividual => 29,
            Self::MerchantAccountInfoMerchant => 30,
            Self::MerchantCategoryCode => 52,
            Self::TransactionCurrency => 53,
            Self::TransactionAmount => 54,
            Self::CountryCode => 58,
            Self::MerchantName => 59,
            Self::MerchantCity => 60,
            Self::AdditionalDataTemplate => 62,
            Self::AdditionalDataField => 99,
            Self::Crc => 63,
            Self::UnionPayMerchant => 15,
            Self::MerchantInformationLanguageTemplate => 64,
        }
    }

    pub fn from_code(number: u8) -> Option<Tags> {
        match number {
            0 => Some(Self::PayloadFormatIndicator),
            1 => Some(Self::PointOfInitialMethod),
            29 => Some(Self::MerchantAccountInfoIndividual),
            30 => Some(Self::MerchantAccountInfoMerchant),
            52 => Some(Self::MerchantCategoryCode),
            53 => Some(Self::TransactionCurrency),
            54 => Some(Self::TransactionAmount),
            58 => Some(Self::CountryCode),
            59 => Some(Self::MerchantName),
            60 => Some(Self::MerchantCity),
            62 => Some(Self::AdditionalDataTemplate),
            99 => Some(Self::AdditionalDataField),
            63 => Some(Self::Crc),
            15 => Some(Self::UnionPayMerchant),
            64 => Some(Self::MerchantInformationLanguageTemplate),
            _ => None,
        }
    }

    pub fn max_length(&self) -> usize {
        match self {
            Self::PayloadFormatIndicator => 2,
            Self::PointOfInitialMethod => 2,
            Self::TransactionCurrency => 3,
            Self::CountryCode => 3,
            Self::Crc => 4,
            Self::MerchantAccountInfoIndividual => 99,
            Self::MerchantAccountInfoMerchant => 99,
            Self::MerchantCategoryCode => 4,
            Self::TransactionAmount => 14,
            Self::MerchantName => 25,
            Self::MerchantCity => 15,
            Self::AdditionalDataTemplate => 99,
            Self::AdditionalDataField => 99,
            Self::UnionPayMerchant => 99,
            Self::MerchantInformationLanguageTemplate => 99,
        }
    }

    pub fn validate_length(&self, value: &str) -> Result<(), String> {
        let len = value.len();
        let max = self.max_length();

        match self {
            Self::MerchantCategoryCode => {
                if len != 4 {
                    return Err("MerchantCategoryCode must be exactly 4 digits".to_string());
                }
                if !value.chars().all(|c| c.is_ascii_digit()) {
                    return Err("MerchantCategoryCode must contain only digits".to_string());
                }
            }
            _ => {
                if len > max {
                    return Err(format!("{:?} exceeds max length {}", self, max));
                }
            }
        }

        Ok(())
    }
}
