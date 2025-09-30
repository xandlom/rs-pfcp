//! Header Enrichment IE.
//!
//! Specifies HTTP header or URL/URI enrichment for traffic steering
//! and service identification in 5G networks.

use crate::ie::{Ie, IeType};
use std::io;

/// Header Type for Header Enrichment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderType {
    HttpHeaderField = 0,
    HttpUrlUri = 1,
}

impl HeaderType {
    fn from_u8(value: u8) -> Result<Self, io::Error> {
        match value {
            0 => Ok(HeaderType::HttpHeaderField),
            1 => Ok(HeaderType::HttpUrlUri),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid Header Type value: {}", value),
            )),
        }
    }

    fn to_u8(self) -> u8 {
        self as u8
    }
}

/// Header Enrichment IE
///
/// Used to enrich HTTP headers or URLs for advanced traffic steering
/// and application detection in 5G networks.
///
/// # 3GPP TS 29.244 Reference
/// - Section 8.2.98: Header Enrichment
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderEnrichment {
    pub header_type: HeaderType,
    pub name: String,  // Header field name or URL/URI prefix
    pub value: String, // Header field value or URL/URI suffix
}

impl HeaderEnrichment {
    /// Creates a new Header Enrichment IE
    pub fn new(header_type: HeaderType, name: String, value: String) -> Self {
        HeaderEnrichment {
            header_type,
            name,
            value,
        }
    }

    /// Creates HTTP header field enrichment
    ///
    /// # Example
    /// ```
    /// # use rs_pfcp::ie::header_enrichment::HeaderEnrichment;
    /// let enrichment = HeaderEnrichment::http_header(
    ///     "X-Operator-ID".to_string(),
    ///     "operator123".to_string()
    /// );
    /// ```
    pub fn http_header(name: String, value: String) -> Self {
        HeaderEnrichment {
            header_type: HeaderType::HttpHeaderField,
            name,
            value,
        }
    }

    /// Creates HTTP URL/URI enrichment
    ///
    /// # Example
    /// ```
    /// # use rs_pfcp::ie::header_enrichment::HeaderEnrichment;
    /// let enrichment = HeaderEnrichment::http_url(
    ///     "https://example.com/".to_string(),
    ///     "?session=123".to_string()
    /// );
    /// ```
    pub fn http_url(prefix: String, suffix: String) -> Self {
        HeaderEnrichment {
            header_type: HeaderType::HttpUrlUri,
            name: prefix,
            value: suffix,
        }
    }

    /// Marshals the Header Enrichment IE into bytes
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();

        // Header Type (1 byte) + spare (1 byte)
        data.push(self.header_type.to_u8());
        data.push(0); // Spare byte

        // Length of Name (1 byte)
        let name_bytes = self.name.as_bytes();
        if name_bytes.len() > 255 {
            // Truncate if too long
            data.push(255);
            data.extend_from_slice(&name_bytes[..255]);
        } else {
            data.push(name_bytes.len() as u8);
            data.extend_from_slice(name_bytes);
        }

        // Length of Value (1 byte)
        let value_bytes = self.value.as_bytes();
        if value_bytes.len() > 255 {
            // Truncate if too long
            data.push(255);
            data.extend_from_slice(&value_bytes[..255]);
        } else {
            data.push(value_bytes.len() as u8);
            data.extend_from_slice(value_bytes);
        }

        data
    }

    /// Unmarshals bytes into a Header Enrichment IE
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Header Enrichment payload too short",
            ));
        }

        let mut offset = 0;

        // Header Type (1 byte)
        let header_type = HeaderType::from_u8(payload[offset])?;
        offset += 1;

        // Spare byte
        offset += 1;

        // Length of Name (1 byte)
        let name_len = payload[offset] as usize;
        offset += 1;

        if offset + name_len > payload.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Header Enrichment name length exceeds payload",
            ));
        }

        let name =
            String::from_utf8(payload[offset..offset + name_len].to_vec()).map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid UTF-8 in Header Enrichment name",
                )
            })?;
        offset += name_len;

        // Length of Value (1 byte)
        if offset >= payload.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Missing value length in Header Enrichment",
            ));
        }

        let value_len = payload[offset] as usize;
        offset += 1;

        if offset + value_len > payload.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Header Enrichment value length exceeds payload",
            ));
        }

        let value =
            String::from_utf8(payload[offset..offset + value_len].to_vec()).map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid UTF-8 in Header Enrichment value",
                )
            })?;

        Ok(HeaderEnrichment {
            header_type,
            name,
            value,
        })
    }

    /// Wraps the Header Enrichment in an IE
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::HeaderEnrichment, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_enrichment_http_header() {
        let enrichment =
            HeaderEnrichment::http_header("X-Custom-Header".to_string(), "value123".to_string());

        assert_eq!(enrichment.header_type, HeaderType::HttpHeaderField);
        assert_eq!(enrichment.name, "X-Custom-Header");
        assert_eq!(enrichment.value, "value123");

        let marshaled = enrichment.marshal();
        let unmarshaled = HeaderEnrichment::unmarshal(&marshaled).unwrap();

        assert_eq!(enrichment, unmarshaled);
    }

    #[test]
    fn test_header_enrichment_http_url() {
        let enrichment = HeaderEnrichment::http_url(
            "https://example.com/path".to_string(),
            "?param=value".to_string(),
        );

        assert_eq!(enrichment.header_type, HeaderType::HttpUrlUri);
        assert_eq!(enrichment.name, "https://example.com/path");
        assert_eq!(enrichment.value, "?param=value");

        let marshaled = enrichment.marshal();
        let unmarshaled = HeaderEnrichment::unmarshal(&marshaled).unwrap();

        assert_eq!(enrichment, unmarshaled);
    }

    #[test]
    fn test_header_enrichment_empty_strings() {
        let enrichment =
            HeaderEnrichment::new(HeaderType::HttpHeaderField, String::new(), String::new());

        let marshaled = enrichment.marshal();
        let unmarshaled = HeaderEnrichment::unmarshal(&marshaled).unwrap();

        assert_eq!(enrichment, unmarshaled);
    }

    #[test]
    fn test_header_enrichment_unicode() {
        let enrichment =
            HeaderEnrichment::http_header("X-Unicode-Header".to_string(), "值123".to_string());

        let marshaled = enrichment.marshal();
        let unmarshaled = HeaderEnrichment::unmarshal(&marshaled).unwrap();

        assert_eq!(enrichment, unmarshaled);
    }

    #[test]
    fn test_header_enrichment_to_ie() {
        let enrichment =
            HeaderEnrichment::http_header("X-Test".to_string(), "test-value".to_string());
        let ie = enrichment.to_ie();

        assert_eq!(ie.ie_type, IeType::HeaderEnrichment);

        let unmarshaled = HeaderEnrichment::unmarshal(&ie.payload).unwrap();
        assert_eq!(enrichment, unmarshaled);
    }

    #[test]
    fn test_header_type_conversion() {
        assert_eq!(HeaderType::from_u8(0).unwrap(), HeaderType::HttpHeaderField);
        assert_eq!(HeaderType::from_u8(1).unwrap(), HeaderType::HttpUrlUri);
        assert!(HeaderType::from_u8(2).is_err());
    }

    #[test]
    fn test_header_enrichment_unmarshal_invalid_utf8() {
        let mut data = vec![0, 0, 3]; // header type, spare, name length
        data.extend_from_slice(&[0xFF, 0xFE, 0xFD]); // Invalid UTF-8
        data.push(0); // value length

        let result = HeaderEnrichment::unmarshal(&data);
        assert!(result.is_err());
    }
}
