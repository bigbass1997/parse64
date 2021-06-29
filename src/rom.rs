use std::convert::TryInto;

#[derive(Debug)]
pub struct Header {
    pub pi_regs: u32,
    pub clockrate: u32,
    pub pc: u32,
    pub release: u32,
    pub crc1: u32,
    pub crc2: u32,
    pub unknown0: u64,
    pub image_name: [u8; 20],
    pub unknown1: u32,
    pub manu_id: u32,
    pub cart_id: u16,
    pub country: u16,
}

trait ByteUtil2 { fn to_u8_tuple(&self) -> (u8, u8); }
impl ByteUtil2 for u16 {
    fn to_u8_tuple(&self) -> (u8, u8) {
        let val = *self;
        ((val >> 8) as u8, val as u8)
    }
}

trait ByteUtil4 { fn to_u8_tuple(&self) -> (u8, u8, u8, u8); }
impl ByteUtil4 for u32 {
    fn to_u8_tuple(&self) -> (u8, u8, u8, u8) {
        let val = *self;
        ((val >> 24) as u8, (val >> 16) as u8, (val >> 8) as u8, val as u8)
    }
}

trait ByteUtil8 { fn to_u8_tuple(&self) -> (u8, u8, u8, u8, u8, u8, u8, u8); }
impl ByteUtil8 for u64 {
    fn to_u8_tuple(&self) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
        let val = *self;
        ((val >> 56) as u8, (val >> 48) as u8, (val >> 40) as u8, (val >> 32) as u8, (val >> 24) as u8, (val >> 16) as u8, (val >> 8) as u8, val as u8)
    }
}

impl Into<[u8; 0x40]> for Header {
    fn into(self) -> [u8; 0x40] {
        let pi_regs = self.pi_regs.to_u8_tuple();
        let clockrate = self.clockrate.to_u8_tuple();
        let pc = self.pc.to_u8_tuple();
        let release = self.release.to_u8_tuple();
        let crc1 = self.crc1.to_u8_tuple();
        let crc2 = self.crc2.to_u8_tuple();
        let unknown0 = self.unknown0.to_u8_tuple();
        let img = self.image_name;
        let unknown1 = self.unknown1.to_u8_tuple();
        let manu_id = self.manu_id.to_u8_tuple();
        let cart_id = self.cart_id.to_u8_tuple();
        let country = self.country.to_u8_tuple();
        
        [
            pi_regs.0, pi_regs.1, pi_regs.2, pi_regs.3,
            clockrate.0, clockrate.1, clockrate.2, clockrate.3,
            pc.0, pc.1, pc.2, pc.3,
            release.0, release.1, release.2, release.3,
            crc1.0, crc1.1, crc1.2, crc1.3,
            crc2.0, crc2.1, crc2.2, crc2.3,
            unknown0.0, unknown0.1, unknown0.2, unknown0.3, unknown0.4, unknown0.5, unknown0.6, unknown0.7,
            img[0], img[1], img[2], img[3], img[4], img[5], img[6], img[7], img[8], img[9], img[10], img[11], img[12], img[13], img[14], img[15], img[16], img[17], img[18], img[19],
            unknown1.0, unknown1.1, unknown1.2, unknown1.3,
            manu_id.0, manu_id.1, manu_id.2, manu_id.3,
            cart_id.0, cart_id.1,
            country.0, country.1
        ]
    }
}

#[derive(Debug)]
pub struct Rom {
    pub header: Header,
    pub bootcode: [u32; 1008],
    pub data: Vec<u8>,
}

impl Rom {
    pub fn new(bytes: Vec<u8>) -> Rom {
        let header = Header {
            pi_regs: to_u32(&bytes[0..4]),
            clockrate: to_u32(&bytes[4..8]),
            pc: to_u32(&bytes[8..12]),
            release: to_u32(&bytes[12..16]),
            crc1: to_u32(&bytes[16..20]),
            crc2: to_u32(&bytes[20..24]),
            unknown0: to_u64(&bytes[24..32]),
            image_name: bytes[32..52].try_into().unwrap(),
            unknown1: to_u32(&bytes[52..56]),
            manu_id: to_u32(&bytes[56..60]),
            cart_id: to_u16(&bytes[60..62]),
            country: to_u16(&bytes[62..64]),
        };
        
        let mut bootcode = [0u32; 1008];
        for i in 0..bootcode.len() {
            bootcode[i] = to_u32(&[
                bytes[64 + (i * 4) + 0],
                bytes[64 + (i * 4) + 1],
                bytes[64 + (i * 4) + 2],
                bytes[64 + (i * 4) + 3]
            ]).try_into().unwrap();
        }
        
        Rom {
            header: header,
            bootcode: bootcode,
            data: bytes,
        }
    }
}

fn to_u16(bytes: &[u8]) -> u16 {
    ((bytes[0] as u16) << 8) | (bytes[1] as u16)
}

fn to_u32(bytes: &[u8]) -> u32 {
    ((bytes[0] as u32) << 24) | ((bytes[1] as u32) << 16) | ((bytes[2] as u32) << 8) | (bytes[3] as u32)
}

fn to_u64(bytes: &[u8]) -> u64 {
    ((to_u32(&bytes[0..4]) as u64) << 32) | (to_u32(&bytes[4..8]) as u64)
}