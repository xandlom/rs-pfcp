use super::header::Header;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct UsageReport {
    pub header: Header,
    // Other fields will be added here
}

impl UsageReport {
    pub fn new() -> Self {
        UsageReport::default()
    }

    pub fn marshal(&self) -> Vec<u8> {
        vec![]
    }

    pub fn unmarshal(_buffer: &[u8]) -> Result<Self, String> {
        Ok(UsageReport::default())
    }

    pub fn get_length(&self) -> u16 {
        0
    }
}
