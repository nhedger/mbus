#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LongFrame {
    /// The start byte of the long frame.
    pub start_byte: u8,

    /// The length field of the long frame.
    pub length: u8,

    /// The repeated length field of the long frame.
    pub length_repeat: u8,

    /// The repeated start byte of the long frame.
    pub start_byte_repeat: u8,

    /// The control field of the long frame.
    pub control: u8,

    /// The address field of the long frame.
    pub address: u8,

    /// The payload of the long frame.
    pub payload: Vec<u8>,

    /// The checksum of the long frame.
    pub checksum: u8,

    /// The stop byte of the long frame.
    pub stop_byte: u8,
}

impl LongFrame {
    /// The base length of a long frame without payload.
    pub const BASE_LEN: usize = 8;

    /// The start byte constant of a long frame.
    pub const START_BYTE: u8 = 0x68;

    /// The stop byte constant of a long frame.
    pub const STOP_BYTE: u8 = 0x16;

    /// Create a new long frame.
    ///
    /// Create a new long frame with the given control and address fields, and payload.
    ///
    /// The length and checksum are calculated automatically.
    pub fn new(control: u8, address: u8, payload: Vec<u8>) -> Self {
        let length = (payload.len() + 2) as u8;
        let mut checksum = control.wrapping_add(address);

        for byte in &payload {
            checksum = checksum.wrapping_add(*byte);
        }

        Self {
            start_byte: Self::START_BYTE,
            length,
            length_repeat: length,
            start_byte_repeat: Self::START_BYTE,
            control,
            address,
            payload,
            checksum,
            stop_byte: Self::STOP_BYTE,
        }
    }

    /// Convert the long frame to a byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(Self::BASE_LEN + self.payload.len());

        out.extend_from_slice(&[
            self.start_byte,
            self.length,
            self.length_repeat,
            self.start_byte_repeat,
            self.control,
            self.address,
        ]);
        out.extend_from_slice(&self.payload);
        out.push(self.checksum);
        out.push(self.stop_byte);

        out
    }
}

/// Convert a long frame into a byte vector.s
impl From<LongFrame> for Vec<u8> {
    fn from(frame: LongFrame) -> Self {
        frame.to_bytes()
    }
}

/// Convert a reference to a long frame into a byte vector.
impl From<&LongFrame> for Vec<u8> {
    fn from(frame: &LongFrame) -> Self {
        frame.to_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::LongFrame;

    #[test]
    fn new_computes_length_checksum_and_constants() {
        let payload = vec![0x01, 0x02, 0x03];
        let frame = LongFrame::new(0x44, 0x01, payload);

        assert_eq!(frame.start_byte, LongFrame::START_BYTE);
        assert_eq!(frame.start_byte_repeat, LongFrame::START_BYTE);
        assert_eq!(frame.stop_byte, LongFrame::STOP_BYTE);
        assert_eq!(frame.control, 0x44);
        assert_eq!(frame.address, 0x01);
        assert_eq!(frame.length, 5);
        assert_eq!(frame.length_repeat, 5);
        assert_eq!(frame.checksum, 0x4b);
    }

    #[test]
    fn to_bytes_matches_fields() {
        let payload = vec![0x10, 0x20];
        let frame = LongFrame {
            start_byte: 0x68,
            length: 4,
            length_repeat: 4,
            start_byte_repeat: 0x68,
            control: 0x40,
            address: 0x02,
            payload,
            checksum: 0x72,
            stop_byte: 0x16,
        };

        assert_eq!(
            frame.to_bytes(),
            vec![0x68, 0x04, 0x04, 0x68, 0x40, 0x02, 0x10, 0x20, 0x72, 0x16]
        );
    }

    #[test]
    fn from_converts_to_vec() {
        let payload = vec![0x0a, 0x0b, 0x0c];
        let frame = LongFrame::new(0x50, 0x03, payload);

        let bytes: Vec<u8> = frame.into();
        assert_eq!(
            bytes,
            vec![0x68, 0x05, 0x05, 0x68, 0x50, 0x03, 0x0a, 0x0b, 0x0c, 0x74, 0x16]
        );
    }

    #[test]
    fn from_ref_converts_to_vec() {
        let payload = vec![0x99];
        let frame = LongFrame::new(0x01, 0x02, payload);

        let bytes: Vec<u8> = (&frame).into();
        assert_eq!(
            bytes,
            vec![0x68, 0x03, 0x03, 0x68, 0x01, 0x02, 0x99, 0x9c, 0x16]
        );
    }
}
