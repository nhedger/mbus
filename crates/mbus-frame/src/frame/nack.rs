#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NackFrame;

impl NackFrame {
    /// The length of a negative acknowledgment frame in bytes.
    pub const BYTE_LEN: usize = 1;

    /// The byte value representing a negative acknowledgment.
    pub const NACK_BYTE: u8 = 0xA2;

    /// Create a new negative acknowledgment frame.
    pub fn new() -> Self {
        Self
    }

    /// Convert the negative acknowledgment frame to a byte array.
    pub fn to_bytes(&self) -> [u8; Self::BYTE_LEN] {
        [NackFrame::NACK_BYTE]
    }
}

impl Default for NackFrame {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert a negative acknowledgment frame into a byte vector.
impl From<NackFrame> for Vec<u8> {
    fn from(frame: NackFrame) -> Self {
        frame.to_bytes().to_vec()
    }
}

/// Convert a reference to a negative acknowledgment frame into a byte vector.
impl From<&NackFrame> for Vec<u8> {
    fn from(frame: &NackFrame) -> Self {
        frame.to_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::NackFrame;

    #[test]
    fn to_bytes_returns_single_byte() {
        let frame = NackFrame::new();

        assert_eq!(frame.to_bytes(), [NackFrame::NACK_BYTE]);
    }

    #[test]
    fn from_converts_to_vec() {
        let frame = NackFrame::new();

        let bytes: Vec<u8> = frame.into();
        assert_eq!(bytes, vec![NackFrame::NACK_BYTE]);
    }

    #[test]
    fn from_ref_converts_to_vec() {
        let frame = NackFrame::new();

        let bytes: Vec<u8> = (&frame).into();
        assert_eq!(bytes, vec![NackFrame::NACK_BYTE]);
    }
}
