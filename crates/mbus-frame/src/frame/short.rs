#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortFrame {
    /// The start byte of the short frame.
    pub start_byte: u8,

    /// The control field of the short frame.
    pub control: u8,

    /// The address field of the short frame.
    pub address: u8,

    /// The checksum of the short frame.
    pub checksum: u8,

    /// The stop byte of the short frame.
    pub stop_byte: u8,
}

impl ShortFrame {
    /// The length of a short frame in bytes.
    pub const BYTE_LEN: usize = 5;

    /// The start byte constant of a short frame.
    pub const START_BYTE: u8 = 0x10;

    /// The stop byte constant of a short frame.
    pub const STOP_BYTE: u8 = 0x16;

    /// Create a new short frame.
    ///
    /// Create a new short frame with the given control and address fields.
    ///
    /// The checksum is calculated automatically.
    pub fn new(control: u8, address: u8) -> Self {
        let checksum = control.wrapping_add(address);

        Self {
            start_byte: Self::START_BYTE,
            control,
            address,
            checksum,
            stop_byte: Self::STOP_BYTE,
        }
    }

    /// Convert the short frame to a byte array.
    pub fn to_bytes(&self) -> [u8; Self::BYTE_LEN] {
        [
            self.start_byte,
            self.control,
            self.address,
            self.checksum,
            self.stop_byte,
        ]
    }
}

/// Convert a short frame into a byte vector.
impl From<ShortFrame> for Vec<u8> {
    fn from(frame: ShortFrame) -> Self {
        frame.to_bytes().to_vec()
    }
}

/// Convert a reference to a short frame into a byte vector.
impl From<&ShortFrame> for Vec<u8> {
    fn from(frame: &ShortFrame) -> Self {
        frame.to_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::ShortFrame;

    #[test]
    fn new_computes_checksum_and_constants() {
        let frame = ShortFrame::new(0x40, 0x01);

        assert_eq!(frame.start_byte, ShortFrame::START_BYTE);
        assert_eq!(frame.stop_byte, ShortFrame::STOP_BYTE);
        assert_eq!(frame.control, 0x40);
        assert_eq!(frame.address, 0x01);
        assert_eq!(frame.checksum, 0x41);
    }

    #[test]
    fn to_bytes_matches_fields() {
        let frame = ShortFrame {
            start_byte: 0x10,
            control: 0x53,
            address: 0x02,
            checksum: 0x55,
            stop_byte: 0x16,
        };

        assert_eq!(frame.to_bytes(), [0x10, 0x53, 0x02, 0x55, 0x16]);
    }

    #[test]
    fn from_converts_to_vec() {
        let frame = ShortFrame {
            start_byte: 0x10,
            control: 0x11,
            address: 0x22,
            checksum: 0x33,
            stop_byte: 0x16,
        };

        let bytes: Vec<u8> = frame.into();
        assert_eq!(bytes, vec![0x10, 0x11, 0x22, 0x33, 0x16]);
    }

    #[test]
    fn from_ref_converts_to_vec() {
        let frame = ShortFrame {
            start_byte: 0x10,
            control: 0x21,
            address: 0x32,
            checksum: 0x53,
            stop_byte: 0x16,
        };

        let bytes: Vec<u8> = (&frame).into();
        assert_eq!(bytes, vec![0x10, 0x21, 0x32, 0x53, 0x16]);
    }
}
