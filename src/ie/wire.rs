//! Bounds-checked big-endian readers shared by IE parsers.
//!
//! These replace `data[a..b].try_into().unwrap()` so that a missed length
//! check can never turn into a panic on untrusted input; it becomes an
//! [`PfcpError::InvalidLength`] instead. Call sites normally validate the
//! length first to report an IE-specific error message; these helpers are the
//! backstop.

use crate::error::PfcpError;
use crate::ie::IeType;

/// Copies `N` bytes starting at `offset` out of `data`.
pub(crate) fn read_array<const N: usize>(data: &[u8], offset: usize) -> Result<[u8; N], PfcpError> {
    offset
        .checked_add(N)
        .and_then(|end| data.get(offset..end))
        .and_then(|slice| <[u8; N]>::try_from(slice).ok())
        .ok_or_else(|| {
            PfcpError::invalid_length(
                "IE payload",
                IeType::Unknown,
                offset.saturating_add(N),
                data.len(),
            )
        })
}

macro_rules! be_readers {
    ($($name:ident => $ty:ty),* $(,)?) => {
        $(
            pub(crate) fn $name(data: &[u8], offset: usize) -> Result<$ty, PfcpError> {
                read_array(data, offset).map(<$ty>::from_be_bytes)
            }
        )*
    };
}

be_readers!(
    read_u16 => u16,
    read_u32 => u32,
    read_u64 => u64,
    read_i32 => i32,
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reads_at_offset() {
        let data = [0xAA, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        assert_eq!(read_u16(&data, 1).unwrap(), 0x0001);
        assert_eq!(read_u32(&data, 1).unwrap(), 0x0001_0203);
        assert_eq!(read_u64(&data, 1).unwrap(), 0x0001_0203_0405_0607);
    }

    #[test]
    fn test_short_or_overflowing_offset_is_error_not_panic() {
        let data = [0u8; 3];
        assert!(matches!(
            read_u32(&data, 0),
            Err(PfcpError::InvalidLength {
                expected: 4,
                actual: 3,
                ..
            })
        ));
        assert!(read_u16(&data, 2).is_err());
        assert!(read_u16(&data, usize::MAX).is_err());
        assert!(read_array::<0>(&data, 4).is_err());
        assert!(read_array::<0>(&data, 3).is_ok());
    }
}
