//! Application Instance ID Information Element
//!
//! The Application Instance ID IE identifies a specific application instance
//! that provides application services for detected application traffic.
//! Per 3GPP TS 29.244 Section 8.2.60, this is an OctetString type IE.

use crate::error::PfcpError;
use crate::ie::{Ie, IeType};

/// Application Instance ID
///
/// Identifies a specific application instance in the data network for
/// detected application traffic. Used in conjunction with Application Detection
/// to route traffic to specific application server instances.
///
/// # 3GPP Reference
/// 3GPP TS 29.244 Section 8.2.60
///
/// # Structure
/// Variable-length OctetString containing the application instance identifier
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::application_instance_id::ApplicationInstanceId;
///
/// // Create application instance ID for a specific server
/// let app_id = ApplicationInstanceId::new("server-01.example.com");
/// assert_eq!(app_id.value(), "server-01.example.com");
///
/// // Marshal and unmarshal
/// let bytes = app_id.marshal();
/// let parsed = ApplicationInstanceId::unmarshal(&bytes).unwrap();
/// assert_eq!(app_id, parsed);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ApplicationInstanceId {
    /// Application instance identifier
    instance_id: String,
}

impl ApplicationInstanceId {
    /// Create a new Application Instance ID
    ///
    /// # Arguments
    /// * `instance_id` - Application instance identifier string
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::application_instance_id::ApplicationInstanceId;
    ///
    /// let app_id = ApplicationInstanceId::new("cdn-server-1");
    /// assert_eq!(app_id.value(), "cdn-server-1");
    /// ```
    pub fn new(instance_id: &str) -> Self {
        ApplicationInstanceId {
            instance_id: instance_id.to_string(),
        }
    }

    /// Get the application instance identifier value
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::application_instance_id::ApplicationInstanceId;
    ///
    /// let app_id = ApplicationInstanceId::new("video-server-01");
    /// assert_eq!(app_id.value(), "video-server-01");
    /// ```
    pub fn value(&self) -> &str {
        &self.instance_id
    }

    /// Check if the application instance ID is empty
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::application_instance_id::ApplicationInstanceId;
    ///
    /// let app_id = ApplicationInstanceId::new("server-1");
    /// assert!(!app_id.is_empty());
    ///
    /// let empty_id = ApplicationInstanceId::new("");
    /// assert!(empty_id.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.instance_id.is_empty()
    }

    /// Marshal Application Instance ID to bytes
    ///
    /// # Returns
    /// Byte vector containing UTF-8 encoded instance identifier
    pub fn marshal(&self) -> Vec<u8> {
        self.instance_id.as_bytes().to_vec()
    }

    /// Unmarshal Application Instance ID from bytes
    ///
    /// # Arguments
    /// * `data` - Byte slice containing UTF-8 encoded instance identifier
    ///
    /// # Errors
    /// Returns error if data is not valid UTF-8
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::application_instance_id::ApplicationInstanceId;
    ///
    /// let app_id = ApplicationInstanceId::new("game-server-5");
    /// let bytes = app_id.marshal();
    /// let parsed = ApplicationInstanceId::unmarshal(&bytes).unwrap();
    /// assert_eq!(app_id, parsed);
    /// ```
    pub fn unmarshal(data: &[u8]) -> Result<Self, PfcpError> {
        let instance_id = String::from_utf8(data.to_vec()).map_err(|_| {
            PfcpError::invalid_value("Application Instance ID", "instance_id", "invalid UTF-8")
        })?;
        Ok(ApplicationInstanceId { instance_id })
    }

    /// Convert to generic IE
    ///
    /// # Example
    /// ```
    /// use rs_pfcp::ie::application_instance_id::ApplicationInstanceId;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let app_id = ApplicationInstanceId::new("app-server-1");
    /// let ie = app_id.to_ie();
    /// assert_eq!(ie.ie_type, IeType::ApplicationInstanceId);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::ApplicationInstanceId, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_instance_id_new() {
        let app_id = ApplicationInstanceId::new("server-01");
        assert_eq!(app_id.value(), "server-01");
    }

    #[test]
    fn test_application_instance_id_value() {
        let app_id = ApplicationInstanceId::new("cdn.example.com");
        assert_eq!(app_id.value(), "cdn.example.com");
    }

    #[test]
    fn test_application_instance_id_empty() {
        let empty = ApplicationInstanceId::new("");
        assert!(empty.is_empty());
        assert_eq!(empty.value(), "");

        let non_empty = ApplicationInstanceId::new("server");
        assert!(!non_empty.is_empty());
    }

    #[test]
    fn test_application_instance_id_marshal() {
        let app_id = ApplicationInstanceId::new("test-server");
        let bytes = app_id.marshal();
        assert_eq!(bytes, b"test-server");
    }

    #[test]
    fn test_application_instance_id_marshal_empty() {
        let app_id = ApplicationInstanceId::new("");
        let bytes = app_id.marshal();
        assert!(bytes.is_empty());
    }

    #[test]
    fn test_application_instance_id_unmarshal() {
        let data = b"video-server-1";
        let app_id = ApplicationInstanceId::unmarshal(data).unwrap();
        assert_eq!(app_id.value(), "video-server-1");
    }

    #[test]
    fn test_application_instance_id_unmarshal_empty() {
        let data = b"";
        let app_id = ApplicationInstanceId::unmarshal(data).unwrap();
        assert!(app_id.is_empty());
    }

    #[test]
    fn test_application_instance_id_unmarshal_invalid_utf8() {
        let invalid_data = vec![0xFF, 0xFE, 0xFD]; // Invalid UTF-8
        let result = ApplicationInstanceId::unmarshal(&invalid_data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, PfcpError::InvalidValue { .. }));
    }

    #[test]
    fn test_application_instance_id_round_trip() {
        let original = ApplicationInstanceId::new("app-instance-42");
        let marshaled = original.marshal();
        let unmarshaled = ApplicationInstanceId::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_application_instance_id_round_trip_empty() {
        let original = ApplicationInstanceId::new("");
        let marshaled = original.marshal();
        let unmarshaled = ApplicationInstanceId::unmarshal(&marshaled).unwrap();
        assert_eq!(original, unmarshaled);
    }

    #[test]
    fn test_application_instance_id_to_ie() {
        let app_id = ApplicationInstanceId::new("game-server");
        let ie = app_id.to_ie();
        assert_eq!(ie.ie_type, IeType::ApplicationInstanceId);
        assert_eq!(ie.payload, b"game-server");

        // Verify IE can be unmarshaled
        let parsed = ApplicationInstanceId::unmarshal(&ie.payload).unwrap();
        assert_eq!(app_id, parsed);
    }

    #[test]
    fn test_application_instance_id_clone() {
        let app_id1 = ApplicationInstanceId::new("server-1");
        let app_id2 = app_id1.clone();
        assert_eq!(app_id1, app_id2);
    }

    #[test]
    fn test_application_instance_id_various_formats() {
        // Test different identifier formats
        let formats = vec![
            "simple-id",
            "server-01",
            "app.example.com",
            "192.168.1.10",
            "server_with_underscore",
            "UPPERCASE",
            "MixedCase123",
            "hyphen-separated-id",
        ];

        for format in formats {
            let app_id = ApplicationInstanceId::new(format);
            let marshaled = app_id.marshal();
            let unmarshaled = ApplicationInstanceId::unmarshal(&marshaled).unwrap();
            assert_eq!(app_id, unmarshaled, "Failed for format: {}", format);
        }
    }

    #[test]
    fn test_application_instance_id_5g_scenarios() {
        // Scenario 1: CDN server selection
        let cdn_server = ApplicationInstanceId::new("cdn-edge-server-nyc-01");
        assert_eq!(cdn_server.value(), "cdn-edge-server-nyc-01");

        // Scenario 2: Gaming server instance
        let game_server = ApplicationInstanceId::new("game.region-us-east.server-5");
        assert_eq!(game_server.value(), "game.region-us-east.server-5");

        // Scenario 3: Video streaming server
        let video_server = ApplicationInstanceId::new("video-transcode-server-2");
        assert_eq!(video_server.value(), "video-transcode-server-2");

        // Scenario 4: IoT application instance
        let iot_app = ApplicationInstanceId::new("iot-data-collector-v2");
        assert_eq!(iot_app.value(), "iot-data-collector-v2");
    }

    #[test]
    fn test_application_instance_id_unicode() {
        // Test Unicode support (though typically ASCII in practice)
        let unicode_id = ApplicationInstanceId::new("服务器-01");
        let marshaled = unicode_id.marshal();
        let unmarshaled = ApplicationInstanceId::unmarshal(&marshaled).unwrap();
        assert_eq!(unicode_id, unmarshaled);
    }

    #[test]
    fn test_application_instance_id_long_identifier() {
        // Test with longer identifier
        let long_id = ApplicationInstanceId::new(
            "very-long-application-instance-identifier-with-many-components.subdomain.example.com",
        );
        let marshaled = long_id.marshal();
        let unmarshaled = ApplicationInstanceId::unmarshal(&marshaled).unwrap();
        assert_eq!(long_id, unmarshaled);
    }
}
