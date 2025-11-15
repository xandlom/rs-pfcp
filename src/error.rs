use std::fmt;
use std::io;
use crate::ie::IeType;
use crate::message::MsgType;

/// Errors that can occur when working with PFCP messages and IEs.
///
/// This error type provides structured error information with context
/// about where the error occurred and what went wrong.
///
/// # Examples
///
/// ```
/// use rs_pfcp::error::PfcpError;
/// use rs_pfcp::ie::IeType;
///
/// let err = PfcpError::InvalidIePayload {
///     ie_type: IeType::Fteid,
///     reason: "F-TEID requires at least 5 bytes".into(),
///     expected_min_length: Some(5),
///     actual_length: Some(3),
/// };
///
/// println!("Error: {}", err);
/// ```
#[derive(Debug)]
pub enum PfcpError {
    /// Error parsing or constructing a PFCP header
    InvalidHeader {
        reason: String,
        position: Option<usize>,
    },

    /// Required Information Element is missing
    MissingMandatoryIe {
        ie_type: IeType,
        message_type: Option<MsgType>,
    },

    /// Information Element payload is invalid
    InvalidIePayload {
        ie_type: IeType,
        reason: String,
        expected_min_length: Option<usize>,
        actual_length: Option<usize>,
    },

    /// Information Element has invalid value
    InvalidIeValue {
        ie_type: IeType,
        field_name: String,
        reason: String,
    },

    /// Zero-length IE not allowed (security validation)
    ZeroLengthIeNotAllowed {
        ie_type: IeType,
        ie_type_value: u16,
    },

    /// Message type is unknown or unsupported
    UnknownMessageType {
        type_value: u8,
    },

    /// Message validation failed
    InvalidMessage {
        message_type: MsgType,
        reason: String,
    },

    /// Builder missing required field
    BuilderMissingField {
        field_name: String,
        builder_type: String,
    },

    /// Builder field has invalid value
    BuilderInvalidValue {
        field_name: String,
        reason: String,
    },

    /// UTF-8 encoding error
    Utf8Error {
        ie_type: Option<IeType>,
        source: std::string::FromUtf8Error,
    },

    /// I/O error (wraps std::io::Error)
    Io {
        source: io::Error,
    },

    /// Generic error with context
    Other {
        context: String,
    },
}

impl fmt::Display for PfcpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PfcpError::InvalidHeader { reason, position } => {
                write!(f, "Invalid PFCP header: {}", reason)?;
                if let Some(pos) = position {
                    write!(f, " at byte offset {}", pos)?;
                }
                Ok(())
            }

            PfcpError::MissingMandatoryIe { ie_type, message_type } => {
                write!(f, "Missing mandatory IE {:?} (type {})", ie_type, *ie_type as u16)?;
                if let Some(msg_type) = message_type {
                    write!(f, " in message {:?}", msg_type)?;
                }
                Ok(())
            }

            PfcpError::InvalidIePayload {
                ie_type,
                reason,
                expected_min_length,
                actual_length,
            } => {
                write!(f, "Invalid payload for IE {:?}: {}", ie_type, reason)?;
                if let (Some(expected), Some(actual)) = (expected_min_length, actual_length) {
                    write!(f, " (expected at least {} bytes, got {})", expected, actual)?;
                }
                Ok(())
            }

            PfcpError::InvalidIeValue {
                ie_type,
                field_name,
                reason,
            } => {
                write!(
                    f,
                    "Invalid value for field '{}' in IE {:?}: {}",
                    field_name, ie_type, reason
                )
            }

            PfcpError::ZeroLengthIeNotAllowed { ie_type, ie_type_value } => {
                write!(
                    f,
                    "Zero-length IE not allowed for {:?} (IE type: {}) per 3GPP TS 29.244 R18",
                    ie_type, ie_type_value
                )
            }

            PfcpError::UnknownMessageType { type_value } => {
                write!(f, "Unknown message type: {}", type_value)
            }

            PfcpError::InvalidMessage { message_type, reason } => {
                write!(f, "Invalid {:?} message: {}", message_type, reason)
            }

            PfcpError::BuilderMissingField {
                field_name,
                builder_type,
            } => {
                write!(
                    f,
                    "Builder {} is missing required field '{}'",
                    builder_type, field_name
                )
            }

            PfcpError::BuilderInvalidValue { field_name, reason } => {
                write!(f, "Invalid value for field '{}': {}", field_name, reason)
            }

            PfcpError::Utf8Error { ie_type, source } => {
                write!(f, "UTF-8 encoding error")?;
                if let Some(ie) = ie_type {
                    write!(f, " in IE {:?}", ie)?;
                }
                write!(f, ": {}", source)
            }

            PfcpError::Io { source } => {
                write!(f, "I/O error: {}", source)
            }

            PfcpError::Other { context } => {
                write!(f, "{}", context)
            }
        }
    }
}

impl std::error::Error for PfcpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PfcpError::Utf8Error { source, .. } => Some(source),
            PfcpError::Io { source } => Some(source),
            _ => None,
        }
    }
}

// Conversions from other error types
impl From<io::Error> for PfcpError {
    fn from(source: io::Error) -> Self {
        PfcpError::Io { source }
    }
}

impl From<std::string::FromUtf8Error> for PfcpError {
    fn from(source: std::string::FromUtf8Error) -> Self {
        PfcpError::Utf8Error {
            ie_type: None,
            source,
        }
    }
}

// Convert PfcpError to io::Error for backward compatibility
impl From<PfcpError> for io::Error {
    fn from(err: PfcpError) -> Self {
        match err {
            PfcpError::Io { source } => source,
            other => io::Error::new(io::ErrorKind::InvalidData, other.to_string()),
        }
    }
}

impl PfcpError {
    /// Map PFCP error to 3GPP TS 29.244 Cause code
    ///
    /// Returns the appropriate Cause value for error responses.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::error::PfcpError;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let err = PfcpError::MissingMandatoryIe {
    ///     ie_type: IeType::NodeId,
    ///     message_type: None,
    /// };
    ///
    /// assert_eq!(err.to_cause_code(), 66); // MandatoryIeMissing
    /// ```
    pub fn to_cause_code(&self) -> u8 {
        use crate::ie::cause::CauseValue;

        match self {
            PfcpError::MissingMandatoryIe { .. } => CauseValue::MandatoryIeMissing as u8,
            PfcpError::InvalidIePayload { .. } | PfcpError::InvalidIeValue { .. } => {
                CauseValue::InvalidLength as u8
            }
            PfcpError::UnknownMessageType { .. } => {
                CauseValue::ServiceNotSupported as u8
            }
            _ => CauseValue::RequestRejected as u8,
        }
    }

    /// Add context to an error
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::error::PfcpError;
    ///
    /// let err = PfcpError::Other { context: "parse failed".into() };
    /// let err = err.context("while processing message");
    ///
    /// assert!(err.to_string().contains("while processing message"));
    /// assert!(err.to_string().contains("parse failed"));
    /// ```
    pub fn context(self, context: impl Into<String>) -> Self {
        match self {
            PfcpError::Other { context: existing } => PfcpError::Other {
                context: format!("{}: {}", context.into(), existing),
            },
            other => PfcpError::Other {
                context: format!("{}: {}", context.into(), other),
            },
        }
    }
}

/// Helper trait for adding context to Results
///
/// # Examples
///
/// ```
/// use rs_pfcp::error::{PfcpError, ResultExt};
///
/// fn parse_data(data: &[u8]) -> Result<(), PfcpError> {
///     if data.is_empty() {
///         return Err(PfcpError::Other { context: "no data".into() });
///     }
///     Ok(())
/// }
///
/// let result = parse_data(&[])
///     .context("Failed to parse PFCP message");
///
/// assert!(result.is_err());
/// ```
pub trait ResultExt<T> {
    fn context(self, context: impl Into<String>) -> Result<T, PfcpError>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<PfcpError>,
{
    fn context(self, context: impl Into<String>) -> Result<T, PfcpError> {
        self.map_err(|e| e.into().context(context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_invalid_header() {
        let err = PfcpError::InvalidHeader {
            reason: "version mismatch".into(),
            position: Some(0),
        };

        let msg = err.to_string();
        assert!(msg.contains("Invalid PFCP header"));
        assert!(msg.contains("version mismatch"));
        assert!(msg.contains("byte offset 0"));
    }

    #[test]
    fn test_display_missing_mandatory_ie() {
        let err = PfcpError::MissingMandatoryIe {
            ie_type: IeType::NodeId,
            message_type: Some(MsgType::SessionEstablishmentRequest),
        };

        let msg = err.to_string();
        assert!(msg.contains("Missing mandatory IE"));
        assert!(msg.contains("NodeId"));
        assert!(msg.contains("SessionEstablishmentRequest"));
    }

    #[test]
    fn test_display_invalid_ie_payload() {
        let err = PfcpError::InvalidIePayload {
            ie_type: IeType::Fteid,
            reason: "too short".into(),
            expected_min_length: Some(9),
            actual_length: Some(5),
        };

        let msg = err.to_string();
        assert!(msg.contains("Fteid"));
        assert!(msg.contains("too short"));
        assert!(msg.contains("at least 9 bytes"));
        assert!(msg.contains("got 5"));
    }

    #[test]
    fn test_display_invalid_ie_value() {
        let err = PfcpError::InvalidIeValue {
            ie_type: IeType::GateStatus,
            field_name: "status".into(),
            reason: "value must be 0 or 1".into(),
        };

        let msg = err.to_string();
        assert!(msg.contains("Invalid value for field 'status'"));
        assert!(msg.contains("GateStatus"));
        assert!(msg.contains("value must be 0 or 1"));
    }

    #[test]
    fn test_display_zero_length_ie() {
        let err = PfcpError::ZeroLengthIeNotAllowed {
            ie_type: IeType::PdrId,
            ie_type_value: 56,
        };

        let msg = err.to_string();
        assert!(msg.contains("Zero-length IE not allowed"));
        assert!(msg.contains("PdrId"));
        assert!(msg.contains("IE type: 56"));
        assert!(msg.contains("3GPP TS 29.244 R18"));
    }

    #[test]
    fn test_display_builder_missing_field() {
        let err = PfcpError::BuilderMissingField {
            field_name: "pdr_id".into(),
            builder_type: "CreatePdrBuilder".into(),
        };

        let msg = err.to_string();
        assert!(msg.contains("Builder CreatePdrBuilder"));
        assert!(msg.contains("missing required field 'pdr_id'"));
    }

    #[test]
    fn test_to_cause_code_missing_ie() {
        let err = PfcpError::MissingMandatoryIe {
            ie_type: IeType::NodeId,
            message_type: None,
        };
        assert_eq!(err.to_cause_code(), 66); // MandatoryIeMissing
    }

    #[test]
    fn test_to_cause_code_invalid_payload() {
        let err = PfcpError::InvalidIePayload {
            ie_type: IeType::Fteid,
            reason: "too short".into(),
            expected_min_length: None,
            actual_length: None,
        };
        assert_eq!(err.to_cause_code(), 68); // InvalidLength
    }

    #[test]
    fn test_to_cause_code_unknown_message() {
        let err = PfcpError::UnknownMessageType { type_value: 255 };
        assert_eq!(err.to_cause_code(), 76); // ServiceNotSupported
    }

    #[test]
    fn test_error_context() {
        let err = PfcpError::Other { context: "inner".into() };
        let err = err.context("outer");

        match err {
            PfcpError::Other { context } => {
                assert!(context.contains("outer"));
                assert!(context.contains("inner"));
            }
            _ => panic!("Expected Other variant"),
        }
    }

    #[test]
    fn test_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::UnexpectedEof, "eof");
        let pfcp_err = PfcpError::from(io_err);

        match pfcp_err {
            PfcpError::Io { source } => {
                assert_eq!(source.kind(), io::ErrorKind::UnexpectedEof);
            }
            _ => panic!("Expected Io variant"),
        }
    }

    #[test]
    fn test_to_io_error() {
        let pfcp_err = PfcpError::MissingMandatoryIe {
            ie_type: IeType::NodeId,
            message_type: None,
        };
        let io_err: io::Error = pfcp_err.into();

        assert_eq!(io_err.kind(), io::ErrorKind::InvalidData);
        assert!(io_err.to_string().contains("Missing mandatory IE"));
    }

    #[test]
    fn test_result_ext_context() {
        fn inner_fn() -> Result<(), PfcpError> {
            Err(PfcpError::Other { context: "inner error".into() })
        }

        let result = inner_fn().context("outer context");

        match result {
            Err(PfcpError::Other { context }) => {
                assert!(context.contains("outer context"));
                assert!(context.contains("inner error"));
            }
            _ => panic!("Expected Other error"),
        }
    }

    #[test]
    fn test_error_trait_source() {
        use std::error::Error;

        let utf8_err = String::from_utf8(vec![0xFF]).unwrap_err();
        let pfcp_err = PfcpError::Utf8Error {
            ie_type: Some(IeType::NodeId),
            source: utf8_err,
        };

        assert!(pfcp_err.source().is_some());
    }
}
