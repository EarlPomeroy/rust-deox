#[derive(Debug)]
pub(crate) struct Header {
    magic_number: u32,
    minor_version: u16,
    major_version: u16,
}

pub enum MajorVersion {
    Java19,
    Java18,
    Java17,
    Java16,
    Java15,
    Java14,
    Java13,
    Java12,
    Java11,
    Java10,
    Java9,
    Java8,
    Java7,
    Java6,
    Java5,
    Java1_4,
    Java1_3,
    Java1_2,
    Java1_1,
}

impl Header {
    pub fn new(bytes: [u8; 8]) -> Result<Header, String> {
        let h = Header {
            magic_number: (bytes[0] as u32) << 24
                | (bytes[1] as u32) << 16
                | (bytes[2] as u32) << 8
                | bytes[3] as u32,
            minor_version: (bytes[4] as u16) << 8 | bytes[5] as u16,
            major_version: (bytes[6] as u16) << 8 | bytes[7] as u16,
        };

        if h.magic_number != 0xCAFEBABE {
            return Err(format!(
                "Magic number does not match 0xCAFEBABE. Found {:?}",
                h.magic_number
            ));
        }

        Ok(h)
    }

    pub fn get_major_version(self) -> Result<MajorVersion, String> {
        let ver = match self.major_version {
            0x3F => MajorVersion::Java19,
            0x3E => MajorVersion::Java18,
            0x3D => MajorVersion::Java17,
            0x3C => MajorVersion::Java16,
            0x3B => MajorVersion::Java15,
            0x3A => MajorVersion::Java14,
            0x39 => MajorVersion::Java13,
            0x38 => MajorVersion::Java12,
            0x37 => MajorVersion::Java11,
            0x36 => MajorVersion::Java10,
            0x35 => MajorVersion::Java9,
            0x34 => MajorVersion::Java8,
            0x33 => MajorVersion::Java7,
            0x32 => MajorVersion::Java6,
            0x31 => MajorVersion::Java5,
            0x30 => MajorVersion::Java1_4,
            0x2F => MajorVersion::Java1_3,
            0x2E => MajorVersion::Java1_2,
            0x2D => MajorVersion::Java1_1,
            _ => return Err(format!("Major version not found: {}", self.major_version)),
        };

        Ok(ver)
    }

    fn get_minor_version(self) -> u16 {
        self.minor_version
    }
}
