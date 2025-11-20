// src/ie/pfcp_association_release_request.rs

//! PFCP Association Release Request Information Element.
//!
//! Per 3GPP TS 29.244 Section 8.2.111, the PFCP Association Release Request IE
//! is used to request the graceful release of a PFCP association between control
//! plane and user plane functions.

use crate::ie::graceful_release_period::GracefulReleasePeriod;
use crate::ie::{Ie, IeType};
use std::io;

/// Represents the PFCP Association Release Request IE.
///
/// This IE is used in PFCP Association Release Request messages to specify
/// parameters for graceful shutdown of a PFCP association.
///
/// # Structure
///
/// - Graceful Release Period (mandatory) - Specifies the grace period for shutdown
///
/// # Examples
///
/// ```
/// use rs_pfcp::ie::pfcp_association_release_request::PfcpAssociationReleaseRequest;
/// use rs_pfcp::ie::graceful_release_period::GracefulReleasePeriod;
///
/// // Create a release request with 60 second grace period
/// let period = GracefulReleasePeriod::new(60);
/// let release_req = PfcpAssociationReleaseRequest::new(period);
///
/// // Marshal and unmarshal
/// let marshaled = release_req.marshal();
/// let unmarshaled = PfcpAssociationReleaseRequest::unmarshal(&marshaled).unwrap();
/// assert_eq!(unmarshaled, release_req);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PfcpAssociationReleaseRequest {
    /// Graceful Release Period (mandatory)
    pub graceful_release_period: GracefulReleasePeriod,
}

impl PfcpAssociationReleaseRequest {
    /// Creates a new PFCP Association Release Request IE.
    ///
    /// # Arguments
    ///
    /// * `graceful_release_period` - The grace period for association release
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::ie::pfcp_association_release_request::PfcpAssociationReleaseRequest;
    /// use rs_pfcp::ie::graceful_release_period::GracefulReleasePeriod;
    ///
    /// let period = GracefulReleasePeriod::new(120);
    /// let release_req = PfcpAssociationReleaseRequest::new(period);
    /// assert_eq!(release_req.graceful_release_period.period(), 120);
    /// ```
    pub fn new(graceful_release_period: GracefulReleasePeriod) -> Self {
        PfcpAssociationReleaseRequest {
            graceful_release_period,
        }
    }

    /// Creates a PFCP Association Release Request with a period in seconds.
    ///
    /// # Arguments
    ///
    /// * `seconds` - The grace period in seconds (0-65535)
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::ie::pfcp_association_release_request::PfcpAssociationReleaseRequest;
    ///
    /// let release_req = PfcpAssociationReleaseRequest::with_period_seconds(30);
    /// assert_eq!(release_req.graceful_release_period.period(), 30);
    /// ```
    pub fn with_period_seconds(seconds: u16) -> Self {
        PfcpAssociationReleaseRequest {
            graceful_release_period: GracefulReleasePeriod::new(seconds),
        }
    }

    /// Marshals the PFCP Association Release Request into a byte vector.
    ///
    /// Encodes the Graceful Release Period IE according to 3GPP TS 29.244.
    pub fn marshal(&self) -> Vec<u8> {
        let mut data = Vec::new();

        // Add Graceful Release Period IE (mandatory)
        if let Ok(ie) = self.graceful_release_period.to_ie() {
            data.extend_from_slice(&ie.marshal());
        }

        data
    }

    /// Unmarshals a byte slice into a PFCP Association Release Request IE.
    ///
    /// # Arguments
    ///
    /// * `payload` - The byte slice to unmarshal
    ///
    /// # Returns
    ///
    /// Returns `Ok(PfcpAssociationReleaseRequest)` on success, or an error if the payload is invalid.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The Graceful Release Period (mandatory IE) is missing
    /// - Any IE cannot be unmarshaled
    pub fn unmarshal(payload: &[u8]) -> Result<Self, io::Error> {
        let mut graceful_release_period = None;

        let mut offset = 0;
        while offset < payload.len() {
            let ie = Ie::unmarshal(&payload[offset..])?;
            match ie.ie_type {
                IeType::GracefulReleasePeriod => {
                    graceful_release_period = Some(GracefulReleasePeriod::unmarshal(&ie.payload)?);
                }
                _ => {
                    // Ignore unknown IEs for forward compatibility
                }
            }
            offset += ie.len() as usize;
        }

        Ok(PfcpAssociationReleaseRequest {
            graceful_release_period: graceful_release_period.ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "PFCP Association Release Request missing mandatory Graceful Release Period. Per 3GPP TS 29.244 Section 8.2.111.",
                )
            })?,
        })
    }

    /// Wraps the PFCP Association Release Request in an IE.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_pfcp::ie::pfcp_association_release_request::PfcpAssociationReleaseRequest;
    /// use rs_pfcp::ie::IeType;
    ///
    /// let release_req = PfcpAssociationReleaseRequest::with_period_seconds(45);
    /// let ie = release_req.to_ie();
    /// assert_eq!(ie.ie_type, IeType::PfcpAssociationReleaseRequest);
    /// ```
    pub fn to_ie(&self) -> Ie {
        Ie::new(IeType::PfcpAssociationReleaseRequest, self.marshal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pfcp_association_release_request_marshal_unmarshal() {
        let period = GracefulReleasePeriod::new(60);
        let release_req = PfcpAssociationReleaseRequest::new(period);

        let marshaled = release_req.marshal();
        let unmarshaled = PfcpAssociationReleaseRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(release_req, unmarshaled);
        assert_eq!(unmarshaled.graceful_release_period.period(), 60);
    }

    #[test]
    fn test_pfcp_association_release_request_with_period_seconds() {
        let release_req = PfcpAssociationReleaseRequest::with_period_seconds(120);

        assert_eq!(release_req.graceful_release_period.period(), 120);

        let marshaled = release_req.marshal();
        let unmarshaled = PfcpAssociationReleaseRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(release_req, unmarshaled);
    }

    #[test]
    fn test_pfcp_association_release_request_to_ie() {
        let release_req = PfcpAssociationReleaseRequest::with_period_seconds(30);

        let ie = release_req.to_ie();
        assert_eq!(ie.ie_type, IeType::PfcpAssociationReleaseRequest);

        let unmarshaled = PfcpAssociationReleaseRequest::unmarshal(&ie.payload).unwrap();
        assert_eq!(release_req, unmarshaled);
    }

    #[test]
    fn test_pfcp_association_release_request_unmarshal_missing_period() {
        // Empty payload (missing mandatory Graceful Release Period)
        let result = PfcpAssociationReleaseRequest::unmarshal(&[]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("mandatory"));
        assert!(err.to_string().contains("Graceful Release Period"));
        assert!(err.to_string().contains("3GPP TS 29.244"));
    }

    #[test]
    fn test_pfcp_association_release_request_round_trip() {
        let test_periods = vec![0, 1, 30, 60, 300, 3600, 65535];

        for period_val in test_periods {
            let release_req = PfcpAssociationReleaseRequest::with_period_seconds(period_val);

            let marshaled = release_req.marshal();
            let unmarshaled = PfcpAssociationReleaseRequest::unmarshal(&marshaled).unwrap();

            assert_eq!(release_req, unmarshaled);
            assert_eq!(unmarshaled.graceful_release_period.period(), period_val);
        }
    }

    #[test]
    fn test_pfcp_association_release_request_zero_period() {
        let release_req = PfcpAssociationReleaseRequest::with_period_seconds(0);
        assert_eq!(release_req.graceful_release_period.period(), 0);

        let marshaled = release_req.marshal();
        let unmarshaled = PfcpAssociationReleaseRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(release_req, unmarshaled);
    }

    #[test]
    fn test_pfcp_association_release_request_max_period() {
        let release_req = PfcpAssociationReleaseRequest::with_period_seconds(u16::MAX);
        assert_eq!(release_req.graceful_release_period.period(), u16::MAX);

        let marshaled = release_req.marshal();
        let unmarshaled = PfcpAssociationReleaseRequest::unmarshal(&marshaled).unwrap();

        assert_eq!(release_req, unmarshaled);
    }

    #[test]
    fn test_pfcp_association_release_request_equality() {
        let release_req1 = PfcpAssociationReleaseRequest::with_period_seconds(100);
        let release_req2 = PfcpAssociationReleaseRequest::with_period_seconds(100);
        let release_req3 = PfcpAssociationReleaseRequest::with_period_seconds(200);

        assert_eq!(release_req1, release_req2);
        assert_ne!(release_req1, release_req3);
    }

    #[test]
    fn test_pfcp_association_release_request_typical_values() {
        // Typical graceful shutdown periods
        let short_shutdown = PfcpAssociationReleaseRequest::with_period_seconds(10);
        let medium_shutdown = PfcpAssociationReleaseRequest::with_period_seconds(60);
        let long_shutdown = PfcpAssociationReleaseRequest::with_period_seconds(300);

        for release_req in &[short_shutdown, medium_shutdown, long_shutdown] {
            let marshaled = release_req.marshal();
            let unmarshaled = PfcpAssociationReleaseRequest::unmarshal(&marshaled).unwrap();
            assert_eq!(release_req, &unmarshaled);
        }
    }
}
