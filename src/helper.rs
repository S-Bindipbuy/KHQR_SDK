#[macro_export]
macro_rules! write_tlv {
    ($buffer:expr, ($tag:expr, $value:expr)) => {
        write!($buffer, "{:02}{:02}{}", $tag, $value.len(), $value)
    };
}
