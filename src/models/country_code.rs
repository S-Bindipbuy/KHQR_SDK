pub enum CountryCode {
    KH,
}

impl CountryCode {
    pub fn code(&self) -> &'static str {
        match self {
            CountryCode::KH => "KH",
        }
    }
}
