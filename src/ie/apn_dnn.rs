//! APN/DNN IE.

use crate::ie::{Ie, IeType};
use std::io;

/// Represents the APN/DNN (Access Point Name / Data Network Name) Information Element.
/// Used to identify the access point or data network in 4G/5G networks.
/// Defined in 3GPP TS 29.244 Section 8.2.103.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApnDnn {
    pub name: String,
}

impl ApnDnn {
    /// Creates a new APN/DNN IE.
    pub fn new(name: String) -> Self {
        ApnDnn { name }
    }

    /// Creates an APN/DNN from a string slice.
    pub fn from_str(name: &str) -> Self {
        ApnDnn::new(name.to_string())
    }

    /// Gets the APN/DNN name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the length of the marshaled APN/DNN.
    pub fn len(&self) -> usize {
        // APN/DNN encoding uses DNS label format with length prefixes
        self.encoded_name().len()
    }

    /// Checks if the APN/DNN is empty.
    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
    }

    /// Encodes the APN/DNN name in DNS label format.
    /// Each label is prefixed with its length (1 byte).
    /// Example: "internet.mnc001.mcc001.gprs" becomes:
    /// [8]internet[6]mnc001[6]mcc001[4]gprs[0]
    fn encoded_name(&self) -> Vec<u8> {
        if self.name.is_empty() {
            return vec![0]; // Empty name is encoded as single zero byte
        }

        let mut encoded = Vec::new();
        let labels: Vec<&str> = self.name.split('.').collect();

        for label in labels {
            if label.len() > 63 {
                // RFC 1035: Label length limit is 63 octets
                // In practice, we'll truncate to 63 for robustness
                let truncated = &label[..63];
                encoded.push(truncated.len() as u8);
                encoded.extend_from_slice(truncated.as_bytes());
            } else {
                encoded.push(label.len() as u8);
                encoded.extend_from_slice(label.as_bytes());
            }
        }

        // DNS names end with a zero-length label
        encoded.push(0);
        encoded
    }

    /// Decodes the APN/DNN name from DNS label format.
    fn decode_name(encoded: &[u8]) -> Result<String, io::Error> {
        if encoded.is_empty() {
            return Ok(String::new());
        }

        let mut labels = Vec::new();
        let mut offset = 0;

        while offset < encoded.len() {
            let label_len = encoded[offset] as usize;
            offset += 1;

            if label_len == 0 {
                // End of name
                break;
            }

            if offset + label_len > encoded.len() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid APN/DNN label length",
                ));
            }

            let label = String::from_utf8(encoded[offset..offset + label_len].to_vec())
                .map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid APN/DNN label UTF-8")
                })?;

            labels.push(label);
            offset += label_len;
        }

        Ok(labels.join("."))
    }

    /// Marshals the APN/DNN into a byte vector.
    pub fn marshal(&self) -> Vec<u8> {
        self.encoded_name()
    }

    /// Unmarshals a byte slice into an APN/DNN IE.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let name = Self::decode_name(payload)?;
        Ok(ApnDnn { name })
    }

    /// Wraps the APN/DNN in an APN/DNN IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ApnDnn, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apn_dnn_marshal_unmarshal_simple() {
        let apn_dnn = ApnDnn::new("internet".to_string());
        let marshaled = apn_dnn.marshal();
        let unmarshaled = ApnDnn::unmarshal(&marshaled).unwrap();

        assert_eq!(apn_dnn, unmarshaled);
        assert_eq!(unmarshaled.name(), "internet");
        assert_eq!(marshaled, vec![8, b'i', b'n', b't', b'e', b'r', b'n', b'e', b't', 0]);
        assert_eq!(apn_dnn.len(), 10);
        assert!(!apn_dnn.is_empty());
    }

    #[test]
    fn test_apn_dnn_marshal_unmarshal_complex() {
        let apn_dnn = ApnDnn::new("internet.mnc001.mcc001.gprs".to_string());
        let marshaled = apn_dnn.marshal();
        let unmarshaled = ApnDnn::unmarshal(&marshaled).unwrap();

        assert_eq!(apn_dnn, unmarshaled);
        assert_eq!(unmarshaled.name(), "internet.mnc001.mcc001.gprs");

        // Check encoding format
        let expected = vec![
            8, b'i', b'n', b't', b'e', b'r', b'n', b'e', b't',      // "internet"
            6, b'm', b'n', b'c', b'0', b'0', b'1',                  // "mnc001"
            6, b'm', b'c', b'c', b'0', b'0', b'1',                  // "mcc001"
            4, b'g', b'p', b'r', b's',                              // "gprs"
            0,                                                       // End of name
        ];
        assert_eq!(marshaled, expected);
    }

    #[test]
    fn test_apn_dnn_marshal_unmarshal_empty() {
        let apn_dnn = ApnDnn::new("".to_string());
        let marshaled = apn_dnn.marshal();
        let unmarshaled = ApnDnn::unmarshal(&marshaled).unwrap();

        assert_eq!(apn_dnn, unmarshaled);
        assert_eq!(unmarshaled.name(), "");
        assert_eq!(marshaled, vec![0]);
        assert!(apn_dnn.is_empty());
    }

    #[test]
    fn test_apn_dnn_from_str() {
        let apn_dnn = ApnDnn::from_str("ims");
        assert_eq!(apn_dnn.name(), "ims");

        let marshaled = apn_dnn.marshal();
        let expected = vec![3, b'i', b'm', b's', 0];
        assert_eq!(marshaled, expected);
    }

    #[test]
    fn test_apn_dnn_5g_dnn_examples() {
        let test_cases = vec![
            "internet",
            "ims",
            "xcap",
            "mms",
            "supl",
            "internet.mnc123.mcc456.gprs",
            "ims.mnc001.mcc001.gprs",
        ];

        for name in test_cases {
            let apn_dnn = ApnDnn::from_str(name);
            let marshaled = apn_dnn.marshal();
            let unmarshaled = ApnDnn::unmarshal(&marshaled).unwrap();

            assert_eq!(apn_dnn, unmarshaled);
            assert_eq!(unmarshaled.name(), name);
        }
    }

    #[test]
    fn test_apn_dnn_long_label() {
        // Test with a 63-character label (maximum allowed)
        let long_label = "a".repeat(63);
        let apn_dnn = ApnDnn::new(long_label.clone());
        let marshaled = apn_dnn.marshal();
        let unmarshaled = ApnDnn::unmarshal(&marshaled).unwrap();

        assert_eq!(apn_dnn, unmarshaled);
        assert_eq!(unmarshaled.name(), long_label);
    }

    #[test]
    fn test_apn_dnn_very_long_label_truncation() {
        // Test with a label longer than 63 characters (should be handled gracefully)
        let very_long_label = "a".repeat(100);
        let apn_dnn = ApnDnn::new(very_long_label.clone());
        let marshaled = apn_dnn.marshal();
        let unmarshaled = ApnDnn::unmarshal(&marshaled).unwrap();

        // The first label should be truncated to 63 characters
        assert_eq!(unmarshaled.name(), "a".repeat(63));
    }

    #[test]
    fn test_apn_dnn_to_ie() {
        let apn_dnn = ApnDnn::from_str("test.network");
        let ie = apn_dnn.to_ie();

        assert_eq!(ie.ie_type, IeType::ApnDnn);

        let unmarshaled = ApnDnn::unmarshal(&ie.payload).unwrap();
        assert_eq!(apn_dnn, unmarshaled);
    }

    #[test]
    fn test_apn_dnn_unmarshal_invalid_label_length() {
        // Create malformed encoding where label length exceeds available data
        let malformed = vec![10, b'a', b'b', b'c']; // Says 10 bytes but only 3 available
        let result = ApnDnn::unmarshal(&malformed);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid APN/DNN label length"));
    }

    #[test]
    fn test_apn_dnn_unmarshal_invalid_utf8() {
        // Create encoding with invalid UTF-8 bytes
        let invalid_utf8 = vec![3, 0xFF, 0xFE, 0xFD, 0]; // Invalid UTF-8 sequence
        let result = ApnDnn::unmarshal(&invalid_utf8);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid APN/DNN label UTF-8"));
    }

    #[test]
    fn test_apn_dnn_single_label() {
        let apn_dnn = ApnDnn::from_str("x");
        let marshaled = apn_dnn.marshal();
        let unmarshaled = ApnDnn::unmarshal(&marshaled).unwrap();

        assert_eq!(apn_dnn, unmarshaled);
        assert_eq!(unmarshaled.name(), "x");
        assert_eq!(marshaled, vec![1, b'x', 0]);
    }

    #[test]
    fn test_apn_dnn_multiple_short_labels() {
        let apn_dnn = ApnDnn::from_str("a.b.c.d.e");
        let marshaled = apn_dnn.marshal();
        let unmarshaled = ApnDnn::unmarshal(&marshaled).unwrap();

        assert_eq!(apn_dnn, unmarshaled);
        assert_eq!(unmarshaled.name(), "a.b.c.d.e");
        
        let expected = vec![
            1, b'a',
            1, b'b', 
            1, b'c',
            1, b'd',
            1, b'e',
            0,
        ];
        assert_eq!(marshaled, expected);
    }

    #[test]
    fn test_apn_dnn_decode_name_empty() {
        let result = ApnDnn::decode_name(&[]).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_apn_dnn_decode_name_zero_only() {
        let result = ApnDnn::decode_name(&[0]).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_apn_dnn_round_trip_various_names() {
        let test_cases = vec![
            "",
            "a",
            "internet",
            "ims.network",
            "test.mnc123.mcc456.gprs",
            "very.long.domain.name.with.many.labels.for.testing",
        ];

        for name in test_cases {
            let original = ApnDnn::from_str(name);
            let marshaled = original.marshal();
            let unmarshaled = ApnDnn::unmarshal(&marshaled).unwrap();
            assert_eq!(original, unmarshaled);
        }
    }
}