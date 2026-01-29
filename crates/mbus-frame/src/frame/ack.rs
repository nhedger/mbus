#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AckFrame;

impl AckFrame {
    /// The length of an acknowledgment frame in bytes.
    pub const BYTE_LEN: usize = 1;

    /// The byte value representing an acknowledgment.
    pub const ACK_BYTE: u8 = 0xE5;

    /// Create a new acknowledgment frame.
    pub fn new() -> Self {
        Self
    }

    /// Convert the acknowledgment frame to a byte array.
    pub fn to_bytes(&self) -> [u8; Self::BYTE_LEN] {
        [AckFrame::ACK_BYTE]
    }
}

impl Default for AckFrame {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert an acknowledgment frame into a byte vector.
impl From<AckFrame> for Vec<u8> {
    fn from(frame: AckFrame) -> Self {
        frame.to_bytes().to_vec()
    }
}

/// Convert a reference to an acknowledgment frame into a byte vector.
impl From<&AckFrame> for Vec<u8> {
    fn from(frame: &AckFrame) -> Self {
        frame.to_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::AckFrame;

    #[test]
    fn to_bytes_returns_single_byte() {
        let frame = AckFrame::new();

        assert_eq!(frame.to_bytes(), [AckFrame::ACK_BYTE]);
    }

    #[test]
    fn from_converts_to_vec() {
        let frame = AckFrame::new();

        let bytes: Vec<u8> = frame.into();
        assert_eq!(bytes, vec![AckFrame::ACK_BYTE]);
    }

    #[test]
    fn from_ref_converts_to_vec() {
        let frame = AckFrame::new();

        let bytes: Vec<u8> = (&frame).into();
        assert_eq!(bytes, vec![AckFrame::ACK_BYTE]);
    }
}
