
fn crc32(bytes: &[u8]) -> u32 {
    let crc32_poly: u32 = 0xEDB88320;
    let mut crc: u32 = 0xFFFFFFFF;

    for byte in bytes {
        crc ^= *byte as u32; // XOR
        for _ in 0..8 {
            if (crc & 1) == 1 {
                crc >>= 1;
                crc ^= crc32_poly;
            } else {
                crc >>=1
            }
        }
    }
    crc ^ 0xFFFFFFFF
}


fn main() {
    let hello_bin = b"hello";
    
    let crc = crc32(hello_bin);

    println!("CRC-32: {:08X}", crc);
}
