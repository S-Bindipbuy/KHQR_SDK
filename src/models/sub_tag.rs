use super::Tags;

#[derive(Debug)]
pub enum SubTags {
    BakongAccountIdentifier,
    IndividualAccountInformation,
    MerchantIdentifier,
    AcquiringBank,
    BillNumber,
    MobileNumber,
    StoreLabel,
    TerminalLabel,
    PurposeOfTransaction,
    LanguagePreference,
    MerchantNameAlternateLanguage,
    MerchantCityAlternateLanguage,
    CreationTimestamp,
    ExpirationTimestamp,
}

impl SubTags {
    pub fn code(&self) -> u8 {
        match self {
            Self::BakongAccountIdentifier => 0,
            Self::IndividualAccountInformation => 1,
            Self::MerchantIdentifier => 1,
            Self::AcquiringBank => 2,

            Self::BillNumber => 1,
            Self::MobileNumber => 2,
            Self::StoreLabel => 3,
            Self::TerminalLabel => 7,
            Self::PurposeOfTransaction => 8,

            Self::LanguagePreference => 0,
            Self::MerchantNameAlternateLanguage => 1,
            Self::MerchantCityAlternateLanguage => 2,

            Self::CreationTimestamp => 0,
            Self::ExpirationTimestamp => 1,
        }
    }

    pub fn from_code(tag: Tags, code: u8) -> Option<Self> {
        match (tag, code) {
            (Tags::MerchantAccountInfoIndividual, 0) => Some(Self::BakongAccountIdentifier),
            (Tags::MerchantAccountInfoIndividual, 1) => Some(Self::IndividualAccountInformation),

            (Tags::MerchantAccountInfoMerchant, 1) => Some(Self::MerchantIdentifier),
            (Tags::MerchantAccountInfoMerchant, 2) => Some(Self::AcquiringBank),

            (Tags::AdditionalDataTemplate, 1) => Some(Self::BillNumber),
            (Tags::AdditionalDataTemplate, 2) => Some(Self::MobileNumber),
            (Tags::AdditionalDataTemplate, 3) => Some(Self::StoreLabel),
            (Tags::AdditionalDataTemplate, 7) => Some(Self::TerminalLabel),
            (Tags::AdditionalDataTemplate, 8) => Some(Self::PurposeOfTransaction),

            (Tags::MerchantInformationLanguageTemplate, 0) => Some(Self::LanguagePreference),
            (Tags::MerchantInformationLanguageTemplate, 1) => {
                Some(Self::MerchantNameAlternateLanguage)
            }
            (Tags::MerchantInformationLanguageTemplate, 2) => {
                Some(Self::MerchantCityAlternateLanguage)
            }

            (Tags::AdditionalDataField, 0) => Some(Self::CreationTimestamp),
            (Tags::AdditionalDataField, 1) => Some(Self::ExpirationTimestamp),

            _ => None,
        }
    }

    pub fn max_length(&self) -> usize {
        match self {
            Self::BakongAccountIdentifier => 32,
            Self::IndividualAccountInformation => 32,
            Self::MerchantIdentifier => 32,
            Self::AcquiringBank => 32,
            Self::BillNumber => 25,
            Self::MobileNumber => 25,
            Self::StoreLabel => 25,
            Self::TerminalLabel => 25,
            Self::PurposeOfTransaction => 25,
            Self::LanguagePreference => 2,
            Self::MerchantNameAlternateLanguage => 25,
            Self::MerchantCityAlternateLanguage => 15,
            Self::CreationTimestamp => 13,
            Self::ExpirationTimestamp => 13,
        }
    }

    pub fn validate_length(&self, value: &str) -> Result<(), String> {
        let len = value.len();
        let max = self.max_length();

        match self {
            Self::LanguagePreference => {
                if len != 2 {
                    return Err("LanguagePreference must be exactly 2 characters".to_string());
                }
                if !value.chars().all(|c| c.is_ascii_alphabetic()) {
                    return Err(
                        "LanguagePreference must contain only alphabetic characters".to_string()
                    );
                }
            }

            Self::CreationTimestamp | Self::ExpirationTimestamp => {
                if len != 13 {
                    return Err(format!("{:?} must be exactly 13 digits", self));
                }
                if !value.chars().all(|c| c.is_ascii_digit()) {
                    return Err(format!("{:?} must contain only digits", self));
                }
            }

            Self::MobileNumber => {
                if len > max {
                    return Err(format!("{:?} exceeds max length {}", self, max));
                }
                if !value.chars().all(|c| c.is_ascii_digit()) {
                    return Err("MobileNumber must contain only digits".to_string());
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
