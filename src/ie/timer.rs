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

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data for Timer",
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
    }
}
