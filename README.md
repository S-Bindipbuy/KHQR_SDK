## Bakong KHQR code
#### Indivual QR code
```Rust
use khqr_sdk::{
    Bakong, IndivualInformation, MerchantCity, MerchantType, PointOfInitialMethod,
    TransactionCurrency,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bakong_qr = Bakong {
        qr_type: PointOfInitialMethod::Static(TransactionCurrency::KHR),
        merchant_type: MerchantType::Indivual(IndivualInformation {
            bakong_account_identifier: "abc@dev".to_string(),
            individual_account_information: None,
            acquiring_bank: None,
        }),

        merchant_category_code: None,
        merchant_name: "Example".to_string(),
        merchant_city: Some(MerchantCity::PhnomPenh),

        additional_data_template: None,
        unionpay_merchant: None,

        merchant_information_language_template: None,
    };

    let qr_string = bakong_qr.generate_qr()?;
    println!("Generated QR TLV: {}", qr_string);

    Ok(())
}
```
#### Merchant QR code
```Rust
use chrono::Utc;
use std::time::Duration;

use khqr_sdk::{
    AdditionalDataField, AdditionalDataTemplate, Bakong, MerchantCity, MerchantInformation,
    MerchantInformationLanguageTemplate, MerchantType, PointOfInitialMethod, TransactionAmount,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bakong_qr = Bakong {
        qr_type: PointOfInitialMethod::Dynamic {
            additional_data_field: Some(AdditionalDataField::new(
                Utc::now() + Duration::from_mins(2),
            )?),
            amount: TransactionAmount::KHR(100),
        },

        merchant_type: MerchantType::Merchant(MerchantInformation {
            bakong_account_identifier: "abc@dev".to_string(),
            merchant_identifier: "123456789".to_string(),
            acquiring_bank: "abc".to_string(),
        }),

        merchant_category_code: Some("1234".to_string()),
        merchant_name: "Example".to_string(),
        merchant_city: Some(MerchantCity::PhnomPenh),

        additional_data_template: Some(AdditionalDataTemplate {
            bill_number: Some("1234".to_string()),
            store_label: Some("Hello".to_string()),
            terminal_label: Some("abc".to_string()),
            mobile_number: Some("12345789".to_string()),
            purpose_of_transaction: Some("Order Somethings to eat.".to_string()),
        }),

        unionpay_merchant: Some("1234".to_string()),

        merchant_information_language_template: Some(MerchantInformationLanguageTemplate {
            language_preference: "kh".to_string(),
            merchant_name_alternate_language: "en".to_string(),
            merchant_city_alternate_language: "Siem Reap".to_string(),
        }),
    };

    let qr_string = bakong_qr.generate_qr()?;
    println!("Generated QR TLV: {}", qr_string);

    Ok(())
}
```
