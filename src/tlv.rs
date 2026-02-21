use std::io::Error;
pub trait TLV {
    fn to_tlv(&self, temp: &mut String, buffer: &mut String) -> Result<(), Error>;
}
