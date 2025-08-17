//! PFD Contents IE.

use crate::ie::{Ie, IeType};
use std::io;

/// Represents PFD Contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PfdContents {
    pub flags: u8,
    pub flow_description: Option<String>,
    pub url: Option<String>,
    pub domain_name: Option<String>,
    pub custom_pfd_content: Option<String>,
    pub domain_name_protocol: Option<String>,
    pub additional_flow_description: Vec<String>,
    pub additional_url: Vec<String>,
    pub additional_domain_name_and_protocol: Vec<String>,
}

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

impl Default for PfdContentsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PfdContentsBuilder {
    pub fn new() -> Self {
        PfdContentsBuilder {
            flow_description: None,
            url: None,
            domain_name: None,
            custom_pfd_content: None,
            domain_name_protocol: None,
            additional_flow_description: Vec::new(),
            additional_url: Vec::new(),
            additional_domain_name_and_protocol: Vec::new(),
        }
    }

    pub fn flow_description(mut self, flow_description: &str) -> Self {
        self.flow_description = Some(flow_description.to_string());
        self
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    pub fn domain_name(mut self, domain_name: &str) -> Self {
        self.domain_name = Some(domain_name.to_string());
        self
    }

    pub fn custom_pfd_content(mut self, custom_pfd_content: &str) -> Self {
        self.custom_pfd_content = Some(custom_pfd_content.to_string());
        self
    }

    pub fn domain_name_protocol(mut self, domain_name_protocol: &str) -> Self {
        self.domain_name_protocol = Some(domain_name_protocol.to_string());
        self
    }

    pub fn additional_flow_description(mut self, additional_flow_description: Vec<&str>) -> Self {
        self.additional_flow_description = additional_flow_description
            .iter()
            .map(|s| s.to_string())
            .collect();
        self
    }

    pub fn additional_url(mut self, additional_url: Vec<&str>) -> Self {
        self.additional_url = additional_url.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn additional_domain_name_and_protocol(
        mut self,
        additional_domain_name_and_protocol: Vec<&str>,
    ) -> Self {
        self.additional_domain_name_and_protocol = additional_domain_name_and_protocol
            .iter()
            .map(|s| s.to_string())
            .collect();
        self
    }

    pub fn build(self) -> PfdContents {
        let mut flags = 0;
        if self.flow_description.is_some() {
            flags |= 0x01;
        }
        if self.url.is_some() {
            flags |= 0x02;
        }
        if self.domain_name.is_some() {
            flags |= 0x04;
        }
        if self.custom_pfd_content.is_some() {
            flags |= 0x08;
        }
        if self.domain_name_protocol.is_some() {
            flags |= 0x10;
        }
        if !self.additional_flow_description.is_empty() {
            flags |= 0x20;
        }
        if !self.additional_url.is_empty() {
            flags |= 0x40;
        }
        if !self.additional_domain_name_and_protocol.is_empty() {
            flags |= 0x80;
        }

        PfdContents {
            flags,
            flow_description: self.flow_description,
            url: self.url,
            domain_name: self.domain_name,
            custom_pfd_content: self.custom_pfd_content,
            domain_name_protocol: self.domain_name_protocol,
            additional_flow_description: self.additional_flow_description,
            additional_url: self.additional_url,
            additional_domain_name_and_protocol: self.additional_domain_name_and_protocol,
        }
    }
}

impl PfdContents {
    /// Marshals the PFD Contents into a byte vector, which is the payload of the IE.
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

        let read_vec_field = |offset: &mut usize| -> Result<Vec<String>, io::Error> {
            let mut vec = Vec::new();
            while let Some(val) = read_field(offset)? {
                vec.push(val);
            }
            Ok(vec)
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
        let additional_flow_description = if flags & 0x20 != 0 {
            read_vec_field(&mut offset)?
        } else {
            Vec::new()
        };
        let additional_url = if flags & 0x40 != 0 {
            read_vec_field(&mut offset)?
        } else {
            Vec::new()
        };
        let additional_domain_name_and_protocol = if flags & 0x80 != 0 {
            read_vec_field(&mut offset)?
        } else {
            Vec::new()
        };

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
    fn test_pfd_contents_marshal_unmarshal() {
        let pfd_contents = PfdContentsBuilder::new()
            .flow_description("flow desc")
            .url("http://example.com")
            .domain_name("example.com")
            .build();

        let marshaled = pfd_contents.marshal();
        let unmarshaled = PfdContents::unmarshal(&marshaled).unwrap();

        assert_eq!(pfd_contents, unmarshaled);
    }
}
