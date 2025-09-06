//! User ID IE.

use crate::ie::{Ie, IeType};
use std::io;

/// Represents the User ID Information Element.
/// Used for enhanced user identification in 5G networks.
/// Defined in 3GPP TS 29.244 Section 8.2.100.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId {
    pub user_id_type: UserIdType,
    pub user_id_value: Vec<u8>,
}

/// User ID type values as defined in 3GPP TS 29.244.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum UserIdType {
    /// IMSI (International Mobile Subscriber Identity)
    Imsi = 0,
    /// IMEI (International Mobile Equipment Identity)
    Imei = 1,
    /// MSISDN (Mobile Station International Subscriber Directory Number)
    Msisdn = 2,
    /// NAI (Network Access Identifier)
    Nai = 3,
    /// SUPI (Subscription Permanent Identifier)
    Supi = 4,
    /// GPSI (Generic Public Subscription Identifier)
    Gpsi = 5,
    /// Unknown/reserved user ID type
    Unknown(u8),
}

impl From<u8> for UserIdType {
    fn from(value: u8) -> Self {
        match value {
            0 => UserIdType::Imsi,
            1 => UserIdType::Imei,
            2 => UserIdType::Msisdn,
            3 => UserIdType::Nai,
            4 => UserIdType::Supi,
            5 => UserIdType::Gpsi,
            _ => UserIdType::Unknown(value),
        }
    }
}

impl From<UserIdType> for u8 {
    fn from(user_id_type: UserIdType) -> u8 {
        match user_id_type {
            UserIdType::Imsi => 0,
            UserIdType::Imei => 1,
            UserIdType::Msisdn => 2,
            UserIdType::Nai => 3,
            UserIdType::Supi => 4,
            UserIdType::Gpsi => 5,
            UserIdType::Unknown(value) => value,
        }
    }
}

impl UserId {
    /// Creates a new User ID IE.
    pub fn new(user_id_type: UserIdType, user_id_value: Vec<u8>) -> Self {
        UserId {
            user_id_type,
            user_id_value,
        }
    }

    /// Creates a User ID with IMSI.
    pub fn imsi(imsi: Vec<u8>) -> Self {
        UserId::new(UserIdType::Imsi, imsi)
    }

    /// Creates a User ID with IMEI.
    pub fn imei(imei: Vec<u8>) -> Self {
        UserId::new(UserIdType::Imei, imei)
    }

    /// Creates a User ID with MSISDN.
    pub fn msisdn(msisdn: Vec<u8>) -> Self {
        UserId::new(UserIdType::Msisdn, msisdn)
    }

    /// Creates a User ID with NAI.
    pub fn nai(nai: Vec<u8>) -> Self {
        UserId::new(UserIdType::Nai, nai)
    }

    /// Creates a User ID with SUPI.
    pub fn supi(supi: Vec<u8>) -> Self {
        UserId::new(UserIdType::Supi, supi)
    }

    /// Creates a User ID with GPSI.
    pub fn gpsi(gpsi: Vec<u8>) -> Self {
        UserId::new(UserIdType::Gpsi, gpsi)
    }

    /// Creates a User ID with NAI from string (convenience method).
    pub fn nai_from_string(nai: String) -> Self {
        UserId::nai(nai.into_bytes())
    }

    /// Creates a User ID with SUPI from string (convenience method).
    pub fn supi_from_string(supi: String) -> Self {
        UserId::supi(supi.into_bytes())
    }

    /// Gets the user ID value as a string (for text-based types like NAI).
    pub fn as_string(&self) -> Option<String> {
        match self.user_id_type {
            UserIdType::Nai | UserIdType::Supi | UserIdType::Gpsi => {
                String::from_utf8(self.user_id_value.clone()).ok()
            }
            _ => None,
        }
    }

    /// Gets the length of the marshaled User ID.
    pub fn len(&self) -> usize {
        1 + self.user_id_value.len() // 1 byte for type + value length
    }

    /// Checks if the User ID is empty.
    pub fn is_empty(&self) -> bool {
        self.user_id_value.is_empty()
    }

    /// Marshals the User ID into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(u8::from(self.user_id_type.clone()));
        data.extend_from_slice(&self.user_id_value);
        data
    }

    /// Unmarshals a byte slice into a User ID IE.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "User ID payload too short",
            ));
        }

        let user_id_type = UserIdType::from(payload[0]);
        let user_id_value = payload[1..].to_vec();

        Ok(UserId {
            user_id_type,
            user_id_value,
        })
    }

    /// Wraps the User ID in a User ID IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::UserId, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_marshal_unmarshal_imsi() {
        let imsi_data = vec![0x12, 0x34, 0x56, 0x78, 0x90];
        let user_id = UserId::imsi(imsi_data.clone());
        let marshaled = user_id.marshal();
        let unmarshaled = UserId::unmarshal(&marshaled).unwrap();

        assert_eq!(user_id, unmarshaled);
        assert_eq!(unmarshaled.user_id_type, UserIdType::Imsi);
        assert_eq!(unmarshaled.user_id_value, imsi_data);
        assert_eq!(marshaled[0], 0); // IMSI type
        assert_eq!(marshaled[1..], imsi_data);
        assert_eq!(user_id.len(), 6);
        assert!(!user_id.is_empty());
    }

    #[test]
    fn test_user_id_marshal_unmarshal_nai_string() {
        let nai_string = "user@example.com".to_string();
        let user_id = UserId::nai_from_string(nai_string.clone());
        let marshaled = user_id.marshal();
        let unmarshaled = UserId::unmarshal(&marshaled).unwrap();

        assert_eq!(user_id, unmarshaled);
        assert_eq!(unmarshaled.user_id_type, UserIdType::Nai);
        assert_eq!(unmarshaled.user_id_value, nai_string.as_bytes());
        assert_eq!(unmarshaled.as_string(), Some(nai_string.clone()));
        assert_eq!(marshaled[0], 3); // NAI type
        assert_eq!(&marshaled[1..], nai_string.as_bytes());
    }

    #[test]
    fn test_user_id_marshal_unmarshal_supi() {
        let supi_string = "supi-imsi-001010123456789".to_string();
        let user_id = UserId::supi_from_string(supi_string.clone());
        let marshaled = user_id.marshal();
        let unmarshaled = UserId::unmarshal(&marshaled).unwrap();

        assert_eq!(user_id, unmarshaled);
        assert_eq!(unmarshaled.user_id_type, UserIdType::Supi);
        assert_eq!(unmarshaled.as_string(), Some(supi_string.clone()));
        assert_eq!(marshaled[0], 4); // SUPI type
        assert_eq!(&marshaled[1..], supi_string.as_bytes());
    }

    #[test]
    fn test_user_id_marshal_unmarshal_imei() {
        let imei_data = vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
        let user_id = UserId::imei(imei_data.clone());
        let marshaled = user_id.marshal();
        let unmarshaled = UserId::unmarshal(&marshaled).unwrap();

        assert_eq!(user_id, unmarshaled);
        assert_eq!(unmarshaled.user_id_type, UserIdType::Imei);
        assert_eq!(unmarshaled.user_id_value, imei_data);
        assert_eq!(marshaled[0], 1); // IMEI type
        assert_eq!(marshaled[1..], imei_data);
    }

    #[test]
    fn test_user_id_marshal_unmarshal_msisdn() {
        let msisdn_data = vec![0x91, 0x12, 0x34, 0x56, 0x78, 0x90];
        let user_id = UserId::msisdn(msisdn_data.clone());
        let marshaled = user_id.marshal();
        let unmarshaled = UserId::unmarshal(&marshaled).unwrap();

        assert_eq!(user_id, unmarshaled);
        assert_eq!(unmarshaled.user_id_type, UserIdType::Msisdn);
        assert_eq!(unmarshaled.user_id_value, msisdn_data);
        assert_eq!(marshaled[0], 2); // MSISDN type
    }

    #[test]
    fn test_user_id_marshal_unmarshal_gpsi() {
        let gpsi_string = "msisdn-1234567890".to_string();
        let user_id = UserId::gpsi(gpsi_string.as_bytes().to_vec());
        let marshaled = user_id.marshal();
        let unmarshaled = UserId::unmarshal(&marshaled).unwrap();

        assert_eq!(user_id, unmarshaled);
        assert_eq!(unmarshaled.user_id_type, UserIdType::Gpsi);
        assert_eq!(unmarshaled.as_string(), Some(gpsi_string));
        assert_eq!(marshaled[0], 5); // GPSI type
    }

    #[test]
    fn test_user_id_unknown_type() {
        let unknown_data = vec![0xAA, 0xBB, 0xCC];
        let user_id = UserId::new(UserIdType::Unknown(99), unknown_data.clone());
        let marshaled = user_id.marshal();
        let unmarshaled = UserId::unmarshal(&marshaled).unwrap();

        assert_eq!(user_id, unmarshaled);
        assert_eq!(unmarshaled.user_id_type, UserIdType::Unknown(99));
        assert_eq!(unmarshaled.user_id_value, unknown_data);
        assert_eq!(marshaled[0], 99);
        assert_eq!(unmarshaled.as_string(), None);
    }

    #[test]
    fn test_user_id_to_ie() {
        let user_id = UserId::nai_from_string("test@domain.com".to_string());
        let ie = user_id.to_ie();

        assert_eq!(ie.ie_type, IeType::UserId);

        let unmarshaled = UserId::unmarshal(&ie.payload).unwrap();
        assert_eq!(user_id, unmarshaled);
    }

    #[test]
    fn test_user_id_unmarshal_empty() {
        let result = UserId::unmarshal(&[]);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("User ID payload too short"));
    }

    #[test]
    fn test_user_id_unmarshal_type_only() {
        let result = UserId::unmarshal(&[3]); // NAI type with no value
        assert!(result.is_ok());
        let user_id = result.unwrap();
        assert_eq!(user_id.user_id_type, UserIdType::Nai);
        assert!(user_id.user_id_value.is_empty());
        assert!(user_id.is_empty());
    }

    #[test]
    fn test_user_id_type_conversions() {
        // Test u8 to UserIdType conversion
        assert_eq!(UserIdType::from(0), UserIdType::Imsi);
        assert_eq!(UserIdType::from(1), UserIdType::Imei);
        assert_eq!(UserIdType::from(2), UserIdType::Msisdn);
        assert_eq!(UserIdType::from(3), UserIdType::Nai);
        assert_eq!(UserIdType::from(4), UserIdType::Supi);
        assert_eq!(UserIdType::from(5), UserIdType::Gpsi);
        assert_eq!(UserIdType::from(99), UserIdType::Unknown(99));

        // Test UserIdType to u8 conversion
        assert_eq!(u8::from(UserIdType::Imsi), 0);
        assert_eq!(u8::from(UserIdType::Imei), 1);
        assert_eq!(u8::from(UserIdType::Msisdn), 2);
        assert_eq!(u8::from(UserIdType::Nai), 3);
        assert_eq!(u8::from(UserIdType::Supi), 4);
        assert_eq!(u8::from(UserIdType::Gpsi), 5);
        assert_eq!(u8::from(UserIdType::Unknown(99)), 99);
    }

    #[test]
    fn test_user_id_as_string_binary_types() {
        // Binary types (IMSI, IMEI, MSISDN) should not return string
        let imsi = UserId::imsi(vec![0x12, 0x34, 0x56]);
        let imei = UserId::imei(vec![0x12, 0x34, 0x56]);
        let msisdn = UserId::msisdn(vec![0x12, 0x34, 0x56]);

        assert_eq!(imsi.as_string(), None);
        assert_eq!(imei.as_string(), None);
        assert_eq!(msisdn.as_string(), None);
    }

    #[test]
    fn test_user_id_round_trip_all_types() {
        let test_cases = vec![
            UserId::imsi(vec![0x12, 0x34, 0x56, 0x78, 0x90]),
            UserId::imei(vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF]),
            UserId::msisdn(vec![0x91, 0x12, 0x34, 0x56, 0x78, 0x90]),
            UserId::nai_from_string("user@example.com".to_string()),
            UserId::supi_from_string("supi-imsi-001010123456789".to_string()),
            UserId::gpsi(b"msisdn-1234567890".to_vec()),
            UserId::new(UserIdType::Unknown(200), vec![0xFF, 0xEE, 0xDD]),
        ];

        for original in test_cases {
            let marshaled = original.marshal();
            let unmarshaled = UserId::unmarshal(&marshaled).unwrap();
            assert_eq!(original, unmarshaled);
        }
    }
}
