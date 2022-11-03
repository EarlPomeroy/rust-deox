use std::fmt;

#[derive(Clone, Copy)]
pub struct Header {
    magic_number: u32,
    minor_version: u16,
    major_version: MajorVersion,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MajorVersion {
    Unknown,
    Java19 = 0x3F,
    Java18 = 0x3E,
    Java17 = 0x3D,
    Java16 = 0x3C,
    Java15 = 0x3B,
    Java14 = 0x3A,
    Java13 = 0x39,
    Java12 = 0x38,
    Java11 = 0x37,
    Java10 = 0x36,
    Java9 = 0x35,
    Java8 = 0x34,
    Java7 = 0x33,
    Java6 = 0x32,
    Java5 = 0x31,
    Java1_4 = 0x30,
    Java1_3 = 0x2F,
    Java1_2 = 0x2E,
    Java1_1 = 0x2D,
}

impl MajorVersion {
    fn from_u16(value: u16) -> MajorVersion {
        match value {
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
            _ => MajorVersion::Unknown,
        }
    }
}

impl fmt::Display for MajorVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MajorVersion::Java19 => write!(f, "Java SE 19"),
            MajorVersion::Java18 => write!(f, "Java SE 18"),
            MajorVersion::Java17 => write!(f, "Java SE 17"),
            MajorVersion::Java16 => write!(f, "Java SE 16"),
            MajorVersion::Java15 => write!(f, "Java SE 15"),
            MajorVersion::Java14 => write!(f, "Java SE 14"),
            MajorVersion::Java13 => write!(f, "Java SE 13"),
            MajorVersion::Java12 => write!(f, "Java SE 12"),
            MajorVersion::Java11 => write!(f, "Java SE 11"),
            MajorVersion::Java10 => write!(f, "Java SE 10"),
            MajorVersion::Java9 => write!(f, "Java SE 9"),
            MajorVersion::Java8 => write!(f, "Java SE 8"),
            MajorVersion::Java7 => write!(f, "Java SE 7"),
            MajorVersion::Java6 => write!(f, "Java SE 6.0"),
            MajorVersion::Java5 => write!(f, "Java SE 50."),
            MajorVersion::Java1_4 => write!(f, "Java 1.4"),
            MajorVersion::Java1_3 => write!(f, "Java 1.3"),
            MajorVersion::Java1_2 => write!(f, "Java 1.2"),
            MajorVersion::Java1_1 => write!(f, "Java 1.1"),
            _ => write!(f, "Unknown Version"),
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Magic Number: {:#X}\nMajor Version: {}\nMinor Version: {}",
            self.magic_number,
            self.get_major_version(),
            self.get_minor_version()
        )
    }
}

impl Header {
    pub fn new(bytes: [u8; 8]) -> Result<Header, String> {
        let h = Header {
            magic_number: (bytes[0] as u32) << 24
                | (bytes[1] as u32) << 16
                | (bytes[2] as u32) << 8
                | bytes[3] as u32,
            minor_version: (bytes[4] as u16) << 8 | bytes[5] as u16,
            major_version: MajorVersion::from_u16((bytes[6] as u16) << 8 | bytes[7] as u16),
        };

        if h.magic_number != 0xCAFEBABE {
            return Err(format!(
                "Magic number does not match 0xCAFEBABE. Found {:?}",
                h.magic_number
            ));
        }

        Ok(h)
    }

    pub fn get_major_version(self) -> MajorVersion {
        self.major_version
    }

    fn get_minor_version(self) -> u16 {
        self.minor_version
    }
}

#[cfg(test)]
mod tests {
    use super::Header;
    use super::MajorVersion;

    #[test]
    fn simple_header_test() {
        let test = Header::new([0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x79, 0x00, 0x3C]).unwrap();

        assert_eq!(test.magic_number, 0xCAFEBABE);
        assert_eq!(test.minor_version, 121);
        assert_eq!(test.major_version, MajorVersion::Java16);
    }

    #[test]
    fn empty_minor_version_test() {
        let test = Header::new([0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x00, 0x00, 0x3C]).unwrap();

        assert_eq!(test.magic_number, 0xCAFEBABE);
        assert_eq!(test.minor_version, 0);
        assert_eq!(test.major_version, MajorVersion::Java16);
    }

    #[test]
    fn unknown_minor_version_test() {
        let test = Header::new([0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x79, 0xFF, 0xFF]).unwrap();

        assert_eq!(test.magic_number, 0xCAFEBABE);
        assert_eq!(test.minor_version, 121);
        assert_eq!(test.major_version, MajorVersion::Unknown);
    }

    #[test]
    fn bad_magic_test() {
        let result = Header::new([0xCA, 0xFE, 0x0B, 0x0B, 0x00, 0x79, 0xFF, 0xFF]);

        assert!(result.is_err());
    }

    #[test]
    fn minor_version_func_test() {
        let test = Header::new([0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x79, 0x00, 0x3C]).unwrap();

        assert_eq!(test.get_minor_version(), 121);
    }

    #[test]
    fn major_version_func_test() {
        let test = Header::new([0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x79, 0x00, 0x37]).unwrap();

        assert_eq!(test.get_major_version(), MajorVersion::Java11);
    }

    #[test]
    fn major_version_string_func_test() {
        assert_eq!(format!("{}", MajorVersion::Java1_4), "Java 1.4");
        assert_eq!(format!("{}", MajorVersion::Java8), "Java SE 8");
    }
}
