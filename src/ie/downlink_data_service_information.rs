// src/ie/downlink_data_service_information.rs

//! Downlink Data Service Information IE and its flags.

use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DownlinkDataServiceInformation {
    pub ppi: bool,  // Paging Policy Indication
    pub qfii: bool, // QoS Flow Identifier Indication
}

impl DownlinkDataServiceInformation {
    pub fn new(ppi: bool, qfii: bool) -> Self {
        DownlinkDataServiceInformation { ppi, qfii }
    }

    pub fn marshal(&self) -> [u8; 1] {
        let mut data = [0; 1];
        if self.ppi {
            data[0] |= 1;
        }
        if self.qfii {
            data[0] |= 1 << 1;
        }
        data
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data for DownlinkDataServiceInformation",
            ));
        }
        Ok(DownlinkDataServiceInformation {
            ppi: data[0] & 1 != 0,
            qfii: data[0] & (1 << 1) != 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_downlink_data_service_information_marshal_unmarshal() {
        let ddsi = DownlinkDataServiceInformation::new(true, false);
        let marshaled = ddsi.marshal();
        let unmarshaled = DownlinkDataServiceInformation::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, ddsi);

        let ddsi = DownlinkDataServiceInformation::new(false, true);
        let marshaled = ddsi.marshal();
        let unmarshaled = DownlinkDataServiceInformation::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, ddsi);

        let ddsi = DownlinkDataServiceInformation::new(true, true);
        let marshaled = ddsi.marshal();
        let unmarshaled = DownlinkDataServiceInformation::unmarshal(&marshaled).unwrap();
        assert_eq!(unmarshaled, ddsi);
    }

    #[test]
    fn test_downlink_data_service_information_unmarshal_invalid_data() {
        let data = [];
        let result = DownlinkDataServiceInformation::unmarshal(&data);
        assert!(result.is_err());
    }
}
