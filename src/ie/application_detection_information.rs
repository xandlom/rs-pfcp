use std::io;

use crate::ie::{Ie, IeType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationDetectionInformation {
    pub application_id: String,
    pub application_instance_id: Option<String>,
    pub flow_information: Option<String>,
}

impl ApplicationDetectionInformation {
    pub fn new(
        application_id: String,
        application_instance_id: Option<String>,
        flow_information: Option<String>,
    ) -> Self {
        Self {
            application_id,
            application_instance_id,
            flow_information,
        }
    }

    // Convenience constructors for common application detection scenarios
    pub fn simple_app(app_id: &str) -> Self {
        Self::new(app_id.to_string(), None, None)
    }

    pub fn app_with_instance(app_id: &str, instance_id: &str) -> Self {
        Self::new(app_id.to_string(), Some(instance_id.to_string()), None)
    }

    pub fn app_with_flow_info(app_id: &str, flow_info: &str) -> Self {
        Self::new(app_id.to_string(), None, Some(flow_info.to_string()))
    }

    pub fn full_app_detection(app_id: &str, instance_id: &str, flow_info: &str) -> Self {
        Self::new(
            app_id.to_string(),
            Some(instance_id.to_string()),
            Some(flow_info.to_string()),
        )
    }

    pub fn marshal_len(&self) -> usize {
        let mut len = 1; // flags byte

        // Application ID: 1 byte length + string content
        len += 1 + self.application_id.len();

        // Application Instance ID (optional): 1 byte length + string content
        if let Some(ref instance_id) = self.application_instance_id {
            len += 1 + instance_id.len();
        }

        // Flow Information (optional): 1 byte length + string content
        if let Some(ref flow_info) = self.flow_information {
            len += 1 + flow_info.len();
        }

        len
    }

    pub fn marshal(&self) -> Result<Vec<u8>, io::Error> {
        let mut buf = Vec::with_capacity(self.marshal_len());
        self.marshal_to(&mut buf)?;
        Ok(buf)
    }

    pub fn marshal_to(&self, buf: &mut Vec<u8>) -> Result<(), io::Error> {
        // Marshal flags byte to indicate which optional fields are present
        let mut flags = 0u8;
        if self.application_instance_id.is_some() {
            flags |= 0x01; // Application Instance ID present
        }
        if self.flow_information.is_some() {
            flags |= 0x02; // Flow Information present
        }
        buf.push(flags);

        // Marshal Application ID (required)
        if self.application_id.len() > 255 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Application ID too long (max 255 bytes)",
            ));
        }
        buf.push(self.application_id.len() as u8);
        buf.extend_from_slice(self.application_id.as_bytes());

        // Marshal Application Instance ID (optional)
        if let Some(ref instance_id) = self.application_instance_id {
            if instance_id.len() > 255 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Application Instance ID too long (max 255 bytes)",
                ));
            }
            buf.push(instance_id.len() as u8);
            buf.extend_from_slice(instance_id.as_bytes());
        }

        // Marshal Flow Information (optional)
        if let Some(ref flow_info) = self.flow_information {
            if flow_info.len() > 255 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Flow Information too long (max 255 bytes)",
                ));
            }
            buf.push(flow_info.len() as u8);
            buf.extend_from_slice(flow_info.as_bytes());
        }

        Ok(())
    }

    pub fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        if data.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Application detection information requires at least 2 bytes",
            ));
        }

        let mut cursor = 0;

        // Parse flags byte
        let flags = data[cursor];
        cursor += 1;

        // Parse Application ID (required)
        if cursor >= data.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Missing application ID length",
            ));
        }
        let app_id_len = data[cursor] as usize;
        cursor += 1;

        if cursor + app_id_len > data.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Insufficient data for application ID",
            ));
        }
        let application_id = String::from_utf8(data[cursor..cursor + app_id_len].to_vec())
            .map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid UTF-8 in application ID",
                )
            })?;
        cursor += app_id_len;

        // Parse Application Instance ID (optional, based on flags)
        let mut application_instance_id = None;
        if (flags & 0x01) != 0 {
            if cursor >= data.len() {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Missing application instance ID length",
                ));
            }
            let instance_id_len = data[cursor] as usize;
            cursor += 1;

            if cursor + instance_id_len > data.len() {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Insufficient data for application instance ID",
                ));
            }
            let instance_id = String::from_utf8(data[cursor..cursor + instance_id_len].to_vec())
                .map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid UTF-8 in application instance ID",
                    )
                })?;
            application_instance_id = Some(instance_id);
            cursor += instance_id_len;
        }

        // Parse Flow Information (optional, based on flags)
        let mut flow_information = None;
        if (flags & 0x02) != 0 {
            if cursor >= data.len() {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Missing flow information length",
                ));
            }
            let flow_info_len = data[cursor] as usize;
            cursor += 1;

            if cursor + flow_info_len > data.len() {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Insufficient data for flow information",
                ));
            }
            let flow_info = String::from_utf8(data[cursor..cursor + flow_info_len].to_vec())
                .map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid UTF-8 in flow information",
                    )
                })?;
            flow_information = Some(flow_info);
        }

        Ok(Self {
            application_id,
            application_instance_id,
            flow_information,
        })
    }

    pub fn to_ie(&self) -> Result<Ie, io::Error> {
        let data = self.marshal()?;
        Ok(Ie::new(IeType::ApplicationDetectionInformation, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_detection_information_new() {
        let app_id = "HTTP".to_string();
        let instance_id = Some("session_123".to_string());
        let flow_info = Some("tcp:80".to_string());
        let adi = ApplicationDetectionInformation::new(
            app_id.clone(),
            instance_id.clone(),
            flow_info.clone(),
        );

        assert_eq!(adi.application_id, app_id);
        assert_eq!(adi.application_instance_id, instance_id);
        assert_eq!(adi.flow_information, flow_info);
    }

    #[test]
    fn test_application_detection_information_simple_app() {
        let adi = ApplicationDetectionInformation::simple_app("HTTPS");

        assert_eq!(adi.application_id, "HTTPS");
        assert!(adi.application_instance_id.is_none());
        assert!(adi.flow_information.is_none());
    }

    #[test]
    fn test_application_detection_information_convenience_constructors() {
        let app_with_instance =
            ApplicationDetectionInformation::app_with_instance("YouTube", "video_session_456");
        assert_eq!(app_with_instance.application_id, "YouTube");
        assert_eq!(
            app_with_instance.application_instance_id,
            Some("video_session_456".to_string())
        );
        assert!(app_with_instance.flow_information.is_none());

        let app_with_flow =
            ApplicationDetectionInformation::app_with_flow_info("Netflix", "tcp:443,udp:53");
        assert_eq!(app_with_flow.application_id, "Netflix");
        assert!(app_with_flow.application_instance_id.is_none());
        assert_eq!(
            app_with_flow.flow_information,
            Some("tcp:443,udp:53".to_string())
        );

        let full_app = ApplicationDetectionInformation::full_app_detection(
            "WhatsApp",
            "chat_789",
            "tcp:443,udp:5222",
        );
        assert_eq!(full_app.application_id, "WhatsApp");
        assert_eq!(
            full_app.application_instance_id,
            Some("chat_789".to_string())
        );
        assert_eq!(
            full_app.flow_information,
            Some("tcp:443,udp:5222".to_string())
        );
    }

    #[test]
    fn test_application_detection_information_marshal_unmarshal() {
        let adi = ApplicationDetectionInformation::full_app_detection(
            "Facebook",
            "timeline_abc",
            "tcp:80,tcp:443",
        );

        let data = adi.marshal().unwrap();
        let unmarshaled = ApplicationDetectionInformation::unmarshal(&data).unwrap();

        assert_eq!(adi, unmarshaled);
        assert_eq!(unmarshaled.application_id, "Facebook");
        assert_eq!(
            unmarshaled.application_instance_id,
            Some("timeline_abc".to_string())
        );
        assert_eq!(
            unmarshaled.flow_information,
            Some("tcp:80,tcp:443".to_string())
        );
    }

    #[test]
    fn test_application_detection_information_marshal_simple() {
        let adi = ApplicationDetectionInformation::simple_app("DNS");

        let data = adi.marshal().unwrap();
        let unmarshaled = ApplicationDetectionInformation::unmarshal(&data).unwrap();

        assert_eq!(adi, unmarshaled);
        assert_eq!(unmarshaled.application_id, "DNS");
        assert!(unmarshaled.application_instance_id.is_none());
        assert!(unmarshaled.flow_information.is_none());
    }

    #[test]
    fn test_application_detection_information_marshal_partial() {
        let adi = ApplicationDetectionInformation::app_with_instance("Skype", "call_xyz");

        let data = adi.marshal().unwrap();
        let unmarshaled = ApplicationDetectionInformation::unmarshal(&data).unwrap();

        assert_eq!(adi, unmarshaled);
        assert_eq!(unmarshaled.application_id, "Skype");
        assert_eq!(
            unmarshaled.application_instance_id,
            Some("call_xyz".to_string())
        );
        assert!(unmarshaled.flow_information.is_none());
    }

    #[test]
    fn test_application_detection_information_to_ie() {
        let adi = ApplicationDetectionInformation::simple_app("SSH");

        let ie = adi.to_ie().unwrap();
        assert_eq!(ie.ie_type, IeType::ApplicationDetectionInformation);
    }

    #[test]
    fn test_application_detection_information_unmarshal_empty_data() {
        let data = [];
        let result = ApplicationDetectionInformation::unmarshal(&data);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);

        // Test with only 1 byte (should also fail)
        let data = [0x00];
        let result = ApplicationDetectionInformation::unmarshal(&data);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_application_detection_information_marshal_too_long() {
        // Test application ID too long
        let long_app_id = "A".repeat(256);
        let adi = ApplicationDetectionInformation::simple_app(&long_app_id);
        assert!(adi.marshal().is_err());

        // Test application instance ID too long
        let long_instance_id = "B".repeat(256);
        let adi2 = ApplicationDetectionInformation::app_with_instance("App", &long_instance_id);
        assert!(adi2.marshal().is_err());

        // Test flow information too long
        let long_flow_info = "C".repeat(256);
        let adi3 = ApplicationDetectionInformation::app_with_flow_info("App", &long_flow_info);
        assert!(adi3.marshal().is_err());
    }

    #[test]
    fn test_application_detection_information_marshal_len() {
        let adi = ApplicationDetectionInformation::simple_app("HTTP");
        assert_eq!(adi.marshal_len(), 1 + 1 + 4); // flags + 1 byte length + 4 bytes "HTTP"

        let adi2 = ApplicationDetectionInformation::app_with_instance("HTTPS", "session");
        assert_eq!(adi2.marshal_len(), 1 + 1 + 5 + 1 + 7); // flags + "HTTPS" + "session"

        let adi3 = ApplicationDetectionInformation::full_app_detection("FTP", "transfer", "tcp:21");
        assert_eq!(adi3.marshal_len(), 1 + 1 + 3 + 1 + 8 + 1 + 6); // flags + "FTP" + "transfer" + "tcp:21"
    }

    #[test]
    fn test_application_detection_information_real_world_scenarios() {
        // Test common DPI detection scenarios
        let web_browsing =
            ApplicationDetectionInformation::app_with_flow_info("HTTP", "tcp:80,tcp:8080");
        let video_streaming = ApplicationDetectionInformation::full_app_detection(
            "YouTube",
            "video_4k_stream",
            "tcp:443,udp:443",
        );
        let social_media =
            ApplicationDetectionInformation::app_with_instance("Instagram", "mobile_app_session");
        let gaming = ApplicationDetectionInformation::simple_app("Fortnite");
        let voip =
            ApplicationDetectionInformation::app_with_flow_info("SIP", "udp:5060,rtp:10000-20000");

        for scenario in [web_browsing, video_streaming, social_media, gaming, voip] {
            let data = scenario.marshal().unwrap();
            let unmarshaled = ApplicationDetectionInformation::unmarshal(&data).unwrap();
            assert_eq!(scenario, unmarshaled);
        }
    }

    #[test]
    fn test_application_detection_information_unicode_support() {
        // Test Unicode application names (international apps)
        let chinese_app = ApplicationDetectionInformation::simple_app("微信"); // WeChat in Chinese
        let arabic_app = ApplicationDetectionInformation::simple_app("واتساب"); // WhatsApp in Arabic
        let emoji_app = ApplicationDetectionInformation::simple_app("🎵Music");

        for app in [chinese_app, arabic_app, emoji_app] {
            let data = app.marshal().unwrap();
            let unmarshaled = ApplicationDetectionInformation::unmarshal(&data).unwrap();
            assert_eq!(app, unmarshaled);
        }
    }
}
