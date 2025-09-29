//! PFD Contents IE.
//!
//! According to 3GPP TS 29.244, the PFD Contents IE contains the description of a PFD
//! with flags indicating which fields are present (FD, URL, DN, CP, DNP, AFD, AURL, ADNP).
//!
//! **Flow Description Encoding**: The Flow Description field, when present, shall be encoded
//! as an OctetString as specified in clause 6.4.3.7 of 3GPP TS 29.251 [21], which references
//! the IP filter rule syntax from 3GPP TS 29.212 clause 5.4.2. Flow descriptions should follow
//! the format: `action dir proto from src to dst [options]`
//!
//! Example flow descriptions:
//! - `"permit out tcp from any to any port 80"`
//! - `"deny in udp from 192.168.1.0/24 to any"`
//! - `"permit out ip from any to 10.0.0.0/8"`

use crate::ie::{Ie, IeType};
use std::io;

/// Represents PFD Contents Information Element.
///
/// According to 3GPP TS 29.244 Figure 8.2.39-1, PFD Contents IE contains packet flow
/// description information with the following optional fields:
///
/// - **Flow Description (FD)**: IP filter rule as OctetString per 3GPP TS 29.251 clause 6.4.3.7
/// - **URL**: Application URL pattern
/// - **Domain Name (DN)**: Domain name pattern
/// - **Custom PFD Content (CP)**: Vendor-specific detection content
/// - **Domain Name Protocol (DNP)**: Protocol associated with domain name
/// - **Additional Flow Description (AFD)**: Additional IP filter rules
/// - **Additional URL (AURL)**: Additional URL patterns
/// - **Additional Domain Name and Protocol (ADNP)**: Additional domain/protocol pairs
///
/// The flags field indicates which optional fields are present using bit positions 0-7.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PfdContents {
    pub flags: u8,
    /// Flow Description as IP filter rule (3GPP TS 29.251 clause 6.4.3.7).
    /// Format: "action dir proto from src to dst [options]"
    /// Example: "permit out tcp from any to any port 80"
    pub flow_description: Option<String>,
    pub url: Option<String>,
    pub domain_name: Option<String>,
    pub custom_pfd_content: Option<String>,
    pub domain_name_protocol: Option<String>,
    /// Additional Flow Descriptions as IP filter rules (3GPP TS 29.251 clause 6.4.3.7).
    /// Each entry follows the same format as flow_description.
    pub additional_flow_description: Vec<String>,
    pub additional_url: Vec<String>,
    pub additional_domain_name_and_protocol: Vec<String>,
}

/// Builder for PfdContents Information Element.
///
/// According to 3GPP TS 29.244, PFD Contents IE contains the description of a PFD
/// with flags indicating which fields are present (FD, URL, DN, CP, DNP, AFD, AURL, ADNP).
#[derive(Debug, Default)]
pub struct PfdContentsBuilder {
    flow_description: Option<String>,
    url: Option<String>,
    domain_name: Option<String>,
    custom_pfd_content: Option<String>,
    domain_name_protocol: Option<String>,
    additional_flow_description: Vec<String>,
    additional_url: Vec<String>,
    additional_domain_name_and_protocol: Vec<String>,
}

impl PfdContentsBuilder {
    /// Creates a new PfdContents builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the flow description (FD flag bit 0).
    ///
    /// The flow description shall be an IP filter rule as specified in 3GPP TS 29.251 clause 6.4.3.7.
    /// Format: "action dir proto from src to dst [options]"
    ///
    /// # Examples
    /// ```rust
    /// # use rs_pfcp::ie::pfd_contents::PfdContentsBuilder;
    /// let pfd = PfdContentsBuilder::new()
    ///     .flow_description("permit out tcp from any to any port 80")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn flow_description<S: Into<String>>(mut self, flow_description: S) -> Self {
        self.flow_description = Some(flow_description.into());
        self
    }

    /// Sets the URL (URL flag bit 1).
    pub fn url<S: Into<String>>(mut self, url: S) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Sets the domain name (DN flag bit 2).
    pub fn domain_name<S: Into<String>>(mut self, domain_name: S) -> Self {
        self.domain_name = Some(domain_name.into());
        self
    }

    /// Sets the custom PFD content (CP flag bit 3).
    pub fn custom_pfd_content<S: Into<String>>(mut self, custom_pfd_content: S) -> Self {
        self.custom_pfd_content = Some(custom_pfd_content.into());
        self
    }

    /// Sets the domain name protocol (DNP flag bit 4).
    pub fn domain_name_protocol<S: Into<String>>(mut self, domain_name_protocol: S) -> Self {
        self.domain_name_protocol = Some(domain_name_protocol.into());
        self
    }

    /// Adds an additional flow description (AFD flag bit 5).
    ///
    /// Each additional flow description shall be an IP filter rule as specified in 3GPP TS 29.251 clause 6.4.3.7.
    /// Multiple additional flow descriptions can be added using multiple calls to this method.
    pub fn add_additional_flow_description<S: Into<String>>(mut self, flow_description: S) -> Self {
        self.additional_flow_description
            .push(flow_description.into());
        self
    }

    /// Sets multiple additional flow descriptions (AFD flag bit 5).
    ///
    /// Each flow description shall be an IP filter rule as specified in 3GPP TS 29.251 clause 6.4.3.7.
    pub fn additional_flow_descriptions<I, S>(mut self, flow_descriptions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.additional_flow_description =
            flow_descriptions.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Adds an additional URL (AURL flag bit 6).
    pub fn add_additional_url<S: Into<String>>(mut self, url: S) -> Self {
        self.additional_url.push(url.into());
        self
    }

    /// Sets multiple additional URLs (AURL flag bit 6).
    pub fn additional_urls<I, S>(mut self, urls: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.additional_url = urls.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Adds an additional domain name and protocol (ADNP flag bit 7).
    pub fn add_additional_domain_name_and_protocol<S: Into<String>>(
        mut self,
        domain_name_and_protocol: S,
    ) -> Self {
        self.additional_domain_name_and_protocol
            .push(domain_name_and_protocol.into());
        self
    }

    /// Sets multiple additional domain names and protocols (ADNP flag bit 7).
    pub fn additional_domain_names_and_protocols<I, S>(
        mut self,
        domain_names_and_protocols: I,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.additional_domain_name_and_protocol = domain_names_and_protocols
            .into_iter()
            .map(|s| s.into())
            .collect();
        self
    }

    /// Builds the PfdContents Information Element.
    ///
    /// # Errors
    /// Returns an error if no fields are set (PFD Contents must have at least one field).
    pub fn build(self) -> Result<PfdContents, io::Error> {
        // Validate that at least one field is set
        if self.flow_description.is_none()
            && self.url.is_none()
            && self.domain_name.is_none()
            && self.custom_pfd_content.is_none()
            && self.domain_name_protocol.is_none()
            && self.additional_flow_description.is_empty()
            && self.additional_url.is_empty()
            && self.additional_domain_name_and_protocol.is_empty()
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "PfdContents must have at least one field set",
            ));
        }

        // Calculate flags based on which fields are set (3GPP TS 29.244 Figure 8.2.39-1)
        let mut flags = 0;
        if self.flow_description.is_some() {
            flags |= 0x01; // FD - Flow Description
        }
        if self.url.is_some() {
            flags |= 0x02; // URL - URL
        }
        if self.domain_name.is_some() {
            flags |= 0x04; // DN - Domain Name
        }
        if self.custom_pfd_content.is_some() {
            flags |= 0x08; // CP - Custom PFD Content
        }
        if self.domain_name_protocol.is_some() {
            flags |= 0x10; // DNP - Domain Name Protocol
        }
        if !self.additional_flow_description.is_empty() {
            flags |= 0x20; // AFD - Additional Flow Description
        }
        if !self.additional_url.is_empty() {
            flags |= 0x40; // AURL - Additional URL
        }
        if !self.additional_domain_name_and_protocol.is_empty() {
            flags |= 0x80; // ADNP - Additional Domain Name and Protocol
        }

        Ok(PfdContents {
            flags,
            flow_description: self.flow_description,
            url: self.url,
            domain_name: self.domain_name,
            custom_pfd_content: self.custom_pfd_content,
            domain_name_protocol: self.domain_name_protocol,
            additional_flow_description: self.additional_flow_description,
            additional_url: self.additional_url,
            additional_domain_name_and_protocol: self.additional_domain_name_and_protocol,
        })
    }
}

impl PfdContents {
    /// Creates a new PfdContents builder.
    pub fn builder() -> PfdContentsBuilder {
        PfdContentsBuilder::new()
    }

    /// Creates a PfdContents with just a flow description.
    ///
    /// The flow description shall be an IP filter rule as specified in 3GPP TS 29.251 clause 6.4.3.7.
    /// Format: "action dir proto from src to dst [options]"
    pub fn flow_description<S: Into<String>>(flow_description: S) -> Result<Self, io::Error> {
        PfdContentsBuilder::new()
            .flow_description(flow_description)
            .build()
    }

    /// Creates a PfdContents with just a URL.
    pub fn url<S: Into<String>>(url: S) -> Result<Self, io::Error> {
        PfdContentsBuilder::new().url(url).build()
    }

    /// Creates a PfdContents with just a domain name.
    pub fn domain_name<S: Into<String>>(domain_name: S) -> Result<Self, io::Error> {
        PfdContentsBuilder::new().domain_name(domain_name).build()
    }

    /// Creates a PfdContents with flow description and URL (common pattern).
    ///
    /// The flow description shall be an IP filter rule as specified in 3GPP TS 29.251 clause 6.4.3.7.
    pub fn flow_and_url<S1: Into<String>, S2: Into<String>>(
        flow_description: S1,
        url: S2,
    ) -> Result<Self, io::Error> {
        PfdContentsBuilder::new()
            .flow_description(flow_description)
            .url(url)
            .build()
    }

    /// Creates a PfdContents with domain name and protocol (common pattern).
    pub fn domain_and_protocol<S1: Into<String>, S2: Into<String>>(
        domain_name: S1,
        protocol: S2,
    ) -> Result<Self, io::Error> {
        PfdContentsBuilder::new()
            .domain_name(domain_name)
            .domain_name_protocol(protocol)
            .build()
    }
    /// Marshals the PFD Contents into a byte vector, which is the payload of the IE.
    ///
    /// Encoding follows 3GPP TS 29.244 Figure 8.2.39-1. Flow descriptions are encoded
    /// as OctetString per 3GPP TS 29.251 clause 6.4.3.7 (length-prefixed UTF-8 strings).
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = vec![self.flags, 0]; // Flags and spare

        fn write_field(data: &mut Vec<u8>, field: &Option<String>) {
            if let Some(ref val) = field {
                data.extend_from_slice(&(val.len() as u16).to_be_bytes());
                data.extend_from_slice(val.as_bytes());
            }
        }

        fn write_vec_field(data: &mut Vec<u8>, vec_field: &Vec<String>) {
            for val in vec_field {
                data.extend_from_slice(&(val.len() as u16).to_be_bytes());
                data.extend_from_slice(val.as_bytes());
            }
        }

        write_field(&mut data, &self.flow_description);
        write_field(&mut data, &self.url);
        write_field(&mut data, &self.domain_name);
        write_field(&mut data, &self.custom_pfd_content);
        write_field(&mut data, &self.domain_name_protocol);
        write_vec_field(&mut data, &self.additional_flow_description);
        write_vec_field(&mut data, &self.additional_url);
        write_vec_field(&mut data, &self.additional_domain_name_and_protocol);

        data
    }

    /// Unmarshals a byte slice into PFD Contents.
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        if payload.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "PFD Contents payload too short",
            ));
        }
        let flags = payload[0];
        let mut offset = 2;

        let read_field = |offset: &mut usize| -> Result<Option<String>, io::Error> {
            if payload.len() < *offset + 2 {
                return Ok(None);
            }
            let len = u16::from_be_bytes([payload[*offset], payload[*offset + 1]]) as usize;
            *offset += 2;
            if payload.len() < *offset + len {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Not enough data for field",
                ));
            }
            let val = String::from_utf8(payload[*offset..*offset + len].to_vec())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            *offset += len;
            Ok(Some(val))
        };

        let flow_description = if flags & 0x01 != 0 {
            read_field(&mut offset)?
        } else {
            None
        };
        let url = if flags & 0x02 != 0 {
            read_field(&mut offset)?
        } else {
            None
        };
        let domain_name = if flags & 0x04 != 0 {
            read_field(&mut offset)?
        } else {
            None
        };
        let custom_pfd_content = if flags & 0x08 != 0 {
            read_field(&mut offset)?
        } else {
            None
        };
        let domain_name_protocol = if flags & 0x10 != 0 {
            read_field(&mut offset)?
        } else {
            None
        };
        // For additional fields, we need to know how many entries there are
        // The spec doesn't specify this clearly, so we'll read until we hit different flag types
        // or end of data. This is a limitation of the current spec format.
        let mut additional_flow_description = Vec::new();
        let mut additional_url = Vec::new();
        let mut additional_domain_name_and_protocol = Vec::new();

        // Read additional fields in the order they appear
        if flags & 0x20 != 0 {
            // AFD - Additional Flow Description
            // Read all consecutive additional flow descriptions
            while offset < payload.len() {
                if let Some(val) = read_field(&mut offset)? {
                    additional_flow_description.push(val);
                } else {
                    break;
                }
                // Check if we should continue reading AFD or move to next field type
                // This is a heuristic since spec doesn't clearly define boundaries
                if flags & 0x40 != 0 || flags & 0x80 != 0 {
                    // We have more field types coming, so limit AFD entries
                    // In practice, implementations typically use just one entry per type
                    break;
                }
            }
        }

        if flags & 0x40 != 0 {
            // AURL - Additional URL
            while offset < payload.len() {
                if let Some(val) = read_field(&mut offset)? {
                    additional_url.push(val);
                } else {
                    break;
                }
                // Check if we should continue reading AURL or move to next field type
                if flags & 0x80 != 0 {
                    // We have ADNP coming, so limit AURL entries
                    break;
                }
            }
        }

        if flags & 0x80 != 0 {
            // ADNP - Additional Domain Name and Protocol
            while offset < payload.len() {
                if let Some(val) = read_field(&mut offset)? {
                    additional_domain_name_and_protocol.push(val);
                } else {
                    break;
                }
            }
        }

        Ok(PfdContents {
            flags,
            flow_description,
            url,
            domain_name,
            custom_pfd_content,
            domain_name_protocol,
            additional_flow_description,
            additional_url,
            additional_domain_name_and_protocol,
        })
    }

    /// Wraps the PFD Contents in a PFDContents IE.
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::PfdContents, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pfd_contents_builder_basic() {
        let pfd_contents = PfdContentsBuilder::new()
            .flow_description("flow desc")
            .url("http://example.com")
            .domain_name("example.com")
            .build()
            .unwrap();

        assert_eq!(pfd_contents.flags, 0x07); // FD | URL | DN
        assert_eq!(pfd_contents.flow_description, Some("flow desc".to_string()));
        assert_eq!(pfd_contents.url, Some("http://example.com".to_string()));
        assert_eq!(pfd_contents.domain_name, Some("example.com".to_string()));
    }

    #[test]
    fn test_pfd_contents_builder_all_fields() {
        let pfd_contents = PfdContentsBuilder::new()
            .flow_description("flow desc")
            .url("http://example.com")
            .domain_name("example.com")
            .custom_pfd_content("custom content")
            .domain_name_protocol("https")
            .add_additional_flow_description("additional flow 1")
            .add_additional_flow_description("additional flow 2")
            .add_additional_url("http://additional1.com")
            .add_additional_url("http://additional2.com")
            .add_additional_domain_name_and_protocol("additional domain protocol")
            .build()
            .unwrap();

        assert_eq!(pfd_contents.flags, 0xFF); // All flags set
        assert_eq!(pfd_contents.additional_flow_description.len(), 2);
        assert_eq!(pfd_contents.additional_url.len(), 2);
        assert_eq!(pfd_contents.additional_domain_name_and_protocol.len(), 1);
    }

    #[test]
    fn test_pfd_contents_builder_with_iterators() {
        let flow_descriptions = vec!["flow1", "flow2", "flow3"];
        let urls = vec!["http://url1.com", "http://url2.com"];
        let domain_protocols = vec!["protocol1", "protocol2"];

        let pfd_contents = PfdContentsBuilder::new()
            .flow_description("main flow")
            .additional_flow_descriptions(flow_descriptions)
            .additional_urls(urls)
            .additional_domain_names_and_protocols(domain_protocols)
            .build()
            .unwrap();

        assert_eq!(pfd_contents.flags, 0xE1); // FD | AFD | AURL | ADNP
        assert_eq!(pfd_contents.additional_flow_description.len(), 3);
        assert_eq!(pfd_contents.additional_url.len(), 2);
        assert_eq!(pfd_contents.additional_domain_name_and_protocol.len(), 2);
    }

    #[test]
    fn test_pfd_contents_builder_empty_error() {
        let result = PfdContentsBuilder::new().build();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "PfdContents must have at least one field set"
        );
    }

    #[test]
    fn test_pfd_contents_convenience_methods() {
        // Test flow_description convenience method
        let flow_only = PfdContents::flow_description("test flow").unwrap();
        assert_eq!(flow_only.flags, 0x01);
        assert_eq!(flow_only.flow_description, Some("test flow".to_string()));

        // Test url convenience method
        let url_only = PfdContents::url("http://test.com").unwrap();
        assert_eq!(url_only.flags, 0x02);
        assert_eq!(url_only.url, Some("http://test.com".to_string()));

        // Test domain_name convenience method
        let domain_only = PfdContents::domain_name("test.domain").unwrap();
        assert_eq!(domain_only.flags, 0x04);
        assert_eq!(domain_only.domain_name, Some("test.domain".to_string()));

        // Test flow_and_url convenience method
        let flow_and_url = PfdContents::flow_and_url("flow", "http://url.com").unwrap();
        assert_eq!(flow_and_url.flags, 0x03); // FD | URL
        assert_eq!(flow_and_url.flow_description, Some("flow".to_string()));
        assert_eq!(flow_and_url.url, Some("http://url.com".to_string()));

        // Test domain_and_protocol convenience method
        let domain_and_protocol = PfdContents::domain_and_protocol("domain.com", "https").unwrap();
        assert_eq!(domain_and_protocol.flags, 0x14); // DN | DNP
        assert_eq!(
            domain_and_protocol.domain_name,
            Some("domain.com".to_string())
        );
        assert_eq!(
            domain_and_protocol.domain_name_protocol,
            Some("https".to_string())
        );
    }

    #[test]
    fn test_pfd_contents_marshal_unmarshal() {
        let pfd_contents = PfdContentsBuilder::new()
            .flow_description("flow desc")
            .url("http://example.com")
            .domain_name("example.com")
            .custom_pfd_content("custom")
            .add_additional_flow_description("additional flow")
            .build()
            .unwrap();

        let marshaled = pfd_contents.marshal();
        let unmarshaled = PfdContents::unmarshal(&marshaled).unwrap();

        assert_eq!(pfd_contents, unmarshaled);
    }

    #[test]
    fn test_pfd_contents_to_ie() {
        let pfd_contents = PfdContents::flow_description("test").unwrap();
        let ie = pfd_contents.to_ie();

        assert_eq!(ie.ie_type, IeType::PfdContents);
        assert!(!ie.payload.is_empty());
    }
}
