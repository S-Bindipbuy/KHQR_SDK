const fn generate_crc16_table() -> [u16; 256] {
    const POLY: u16 = 0x1021;
    let mut table = [0u16; 256];
    let mut i = 0;
    let mut b = 0;
    while i < 256 {
        let mut crc = (i as u16) << 8;
        while b < 8 {
            if (crc & 0x8000) != 0 {
                crc = (crc << 1) ^ POLY;
            } else {
                crc <<= 1;
            }
            b += 1;
        }
        table[i as usize] = crc;
        i += 1;
        b = 0;
    }
    table
}

pub fn crc16_ccitt(data: &str) -> u16 {
    let data_bytes = data.as_bytes();
    let table = generate_crc16_table();
    let mut crc: u16 = 0xFFFF;
    for &byte in data_bytes {
        let index = ((crc >> 8) ^ byte as u16) & 0xFF;
        crc = (crc << 8) ^ table[index as usize];
    }
    crc
}
