// src/ie/timer.rs

//! Timer Information Element.

use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timer {
    pub value: u32,
}

impl Timer {
    pub fn new(value: u32) -> Self {
        Timer { value }
    }

    pub fn marshal(&self) -> [u8; 4] {
        self.value.to_be_bytes()
    }

    /// Unmarshals a byte slice into a Timer.
    ///
    /// Per 3GPP TS 29.244, Timer requires exactly 4 bytes (u32).
    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Timer requires 4 bytes (u32), got {}", data.len()),
            ));
        }
        Ok(Timer {
            value: u32::from_be_bytes(data[0..4].try_into().unwrap()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_marshal_unmarshal() {
        let timer = Timer::new(3600);
        let marshaled = timer.marshal();
        let unmarshaled = Timer::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, timer);
    }

    #[test]
    fn test_timer_unmarshal_invalid_data() {
        let data = [0; 3];
        let result = Timer::unmarshal(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("requires 4 bytes"));
        assert!(err.to_string().contains("got 3"));
    }

    #[test]
    fn test_timer_unmarshal_empty() {
        let result = Timer::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("requires 4 bytes"));
        assert!(err.to_string().contains("got 0"));
    }

    #[test]
    fn test_timer_round_trip() {
        let test_values = vec![0, 1, 60, 3600, 86400, 0xFFFFFFFF];
        for value in test_values {
            let timer = Timer::new(value);
            let marshaled = timer.marshal();
            let unmarshaled = Timer::unmarshal(&marshaled).unwrap();
            assert_eq!(unmarshaled.value, value);
        }
    }
}
