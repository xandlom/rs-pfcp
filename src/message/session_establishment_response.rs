//! Session Establishment Response message.

use crate::ie::{Ie, IeType};
use crate::message::{header::Header, Message, MsgType};
use std::io;

/// Represents a Session Establishment Response message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionEstablishmentResponse {
    pub header: Header,
    pub cause: Ie,
    pub offending_ie: Option<Ie>,
    pub fseid: Ie,
    pub created_pdrs: Vec<Ie>,
    pub pdn_type: Option<Ie>,
    pub load_control_information: Option<Ie>,
    pub overload_control_information: Option<Ie>,
    pub ies: Vec<Ie>,
}

impl Message for SessionEstablishmentResponse {
    fn marshal(&self) -> Vec<u8> {
        let mut header = self.header.clone();
        // Recalculate length to include all IEs
        let mut payload_len = self.cause.len();
        if let Some(ie) = &self.offending_ie {
            payload_len += ie.len();
        }
        payload_len += self.fseid.len();
        for ie in &self.created_pdrs {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.pdn_type {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.load_control_information {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.overload_control_information {
            payload_len += ie.len();
        }
        for ie in &self.ies {
            payload_len += ie.len();
        }
        header.length = payload_len + header.len() - 4;

        let mut data = header.marshal();
        data.extend_from_slice(&self.cause.marshal());
        if let Some(ie) = &self.offending_ie {
            data.extend_from_slice(&ie.marshal());
        }
        data.extend_from_slice(&self.fseid.marshal());
        for ie in &self.created_pdrs {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.pdn_type {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.load_control_information {
            data.extend_from_slice(&ie.marshal());
        }
        if let Some(ie) = &self.overload_control_information {
            data.extend_from_slice(&ie.marshal());
        }
        for ie in &self.ies {
            data.extend_from_slice(&ie.marshal());
        }
        data
    }

    fn unmarshal(data: &[u8]) -> Result<Self, io::Error> {
        let header = Header::unmarshal(data)?;
        let mut cause = None;
        let mut offending_ie = None;
        let mut fseid = None;
        let mut created_pdrs = Vec::new();
        let mut pdn_type = None;
        let mut load_control_information = None;
        let mut overload_control_information = None;
        let mut ies = Vec::new();

        let mut offset = header.len() as usize;
        while offset < data.len() {
            let ie = Ie::unmarshal(&data[offset..])?;
            let ie_len = ie.len() as usize;
            match ie.ie_type {
                IeType::Cause => cause = Some(ie),
                IeType::OffendingIe => offending_ie = Some(ie),
                IeType::Fseid => fseid = Some(ie),
                IeType::CreatedPdr => created_pdrs.push(ie),
                IeType::PdnType => pdn_type = Some(ie),
                IeType::LoadControlInformation => load_control_information = Some(ie),
                IeType::OverloadControlInformation => overload_control_information = Some(ie),
                _ => ies.push(ie),
            }
            offset += ie_len;
        }

        Ok(SessionEstablishmentResponse {
            header,
            cause: cause
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Cause IE not found"))?,
            offending_ie,
            fseid: fseid
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "F-SEID IE not found"))?,
            created_pdrs,
            pdn_type,
            load_control_information,
            overload_control_information,
            ies,
        })
    }

    fn msg_type(&self) -> MsgType {
        MsgType::SessionEstablishmentResponse
    }

    fn seid(&self) -> Option<u64> {
        if self.header.has_seid {
            Some(self.header.seid)
        } else {
            None
        }
    }

    fn sequence(&self) -> u32 {
        self.header.sequence_number
    }

    fn set_sequence(&mut self, seq: u32) {
        self.header.sequence_number = seq;
    }

    fn find_ie(&self, ie_type: IeType) -> Option<&Ie> {
        match ie_type {
            IeType::Cause => Some(&self.cause),
            IeType::Fseid => Some(&self.fseid),
            IeType::OffendingIe => self.offending_ie.as_ref(),
            IeType::CreatedPdr => self.created_pdrs.first(),
            IeType::PdnType => self.pdn_type.as_ref(),
            IeType::LoadControlInformation => self.load_control_information.as_ref(),
            IeType::OverloadControlInformation => self.overload_control_information.as_ref(),
            _ => self.ies.iter().find(|ie| ie.ie_type == ie_type),
        }
    }
}

pub struct SessionEstablishmentResponseBuilder {
    seid: u64,
    seq: u32,
    cause: Option<Ie>,
    offending_ie: Option<Ie>,
    fseid: Option<Ie>,
    created_pdrs: Vec<Ie>,
    pdn_type: Option<Ie>,
    load_control_information: Option<Ie>,
    overload_control_information: Option<Ie>,
    ies: Vec<Ie>,
}

impl SessionEstablishmentResponseBuilder {
    /// Creates a new SessionEstablishmentResponse builder with a CauseValue.
    ///
    /// For convenience, use [`accepted()`] or [`rejected()`] constructors.
    /// For full IE control, use [`new_with_ie()`].
    ///
    /// [`accepted()`]: #method.accepted
    /// [`rejected()`]: #method.rejected
    /// [`new_with_ie()`]: #method.new_with_ie
    pub fn new(seid: u64, seq: u32, cause: crate::ie::cause::CauseValue) -> Self {
        use crate::ie::cause::Cause;
        use crate::ie::{Ie, IeType};
        let cause_ie = Ie::new(IeType::Cause, Cause::new(cause).marshal().to_vec());
        SessionEstablishmentResponseBuilder {
            seid,
            seq,
            cause: Some(cause_ie),
            offending_ie: None,
            fseid: None,
            created_pdrs: Vec::new(),
            pdn_type: None,
            load_control_information: None,
            overload_control_information: None,
            ies: Vec::new(),
        }
    }

    /// Convenience constructor for an accepted response.
    ///
    /// Equivalent to `new(seid, seq, CauseValue::RequestAccepted)`.
    pub fn accepted(seid: u64, seq: u32) -> Self {
        Self::new(seid, seq, crate::ie::cause::CauseValue::RequestAccepted)
    }

    /// Convenience constructor for a rejected response.
    ///
    /// Equivalent to `new(seid, seq, CauseValue::RequestRejected)`.
    pub fn rejected(seid: u64, seq: u32) -> Self {
        Self::new(seid, seq, crate::ie::cause::CauseValue::RequestRejected)
    }

    /// Creates a new SessionEstablishmentResponse builder with a cause IE.
    ///
    /// For common cases, use [`new()`], [`accepted()`], or [`rejected()`].
    ///
    /// [`new()`]: #method.new
    /// [`accepted()`]: #method.accepted
    /// [`rejected()`]: #method.rejected
    pub fn new_with_ie(seid: u64, seq: u32, cause: Ie) -> Self {
        SessionEstablishmentResponseBuilder {
            seid,
            seq,
            cause: Some(cause),
            offending_ie: None,
            fseid: None,
            created_pdrs: Vec::new(),
            pdn_type: None,
            load_control_information: None,
            overload_control_information: None,
            ies: Vec::new(),
        }
    }

    pub fn offending_ie(mut self, offending_ie: Ie) -> Self {
        self.offending_ie = Some(offending_ie);
        self
    }

    /// Sets the F-SEID from a SEID value and IP address.
    ///
    /// For full control, use [`fseid_ie`].
    ///
    /// [`fseid_ie`]: #method.fseid_ie
    pub fn fseid<T>(mut self, seid: u64, ip_addr: T) -> Self
    where
        T: Into<std::net::IpAddr>,
    {
        use crate::ie::fseid::Fseid;
        use crate::ie::IeType;
        let ip_addr = ip_addr.into();
        let fseid = match ip_addr {
            std::net::IpAddr::V4(v4) => Fseid::new(seid, Some(v4), None),
            std::net::IpAddr::V6(v6) => Fseid::new(seid, None, Some(v6)),
        };
        self.fseid = Some(crate::ie::Ie::new(IeType::Fseid, fseid.marshal()));
        self
    }

    /// Sets the F-SEID IE directly.
    ///
    /// [`fseid`]: #method.fseid
    pub fn fseid_ie(mut self, fseid: Ie) -> Self {
        self.fseid = Some(fseid);
        self
    }

    pub fn created_pdr(mut self, created_pdr: Ie) -> Self {
        self.created_pdrs.push(created_pdr);
        self
    }

    pub fn pdn_type(mut self, pdn_type: Ie) -> Self {
        self.pdn_type = Some(pdn_type);
        self
    }

    pub fn load_control_information(mut self, load_control_information: Ie) -> Self {
        self.load_control_information = Some(load_control_information);
        self
    }

    pub fn overload_control_information(mut self, overload_control_information: Ie) -> Self {
        self.overload_control_information = Some(overload_control_information);
        self
    }

    pub fn ies(mut self, ies: Vec<Ie>) -> Self {
        self.ies = ies;
        self
    }

    pub fn build(self) -> Result<SessionEstablishmentResponse, io::Error> {
        let cause = self
            .cause
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Cause IE required"))?;
        let fseid = self
            .fseid
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "F-SEID IE required"))?;

        let mut payload_len = cause.len() + fseid.len();
        if let Some(ie) = &self.offending_ie {
            payload_len += ie.len();
        }
        for ie in &self.created_pdrs {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.pdn_type {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.load_control_information {
            payload_len += ie.len();
        }
        if let Some(ie) = &self.overload_control_information {
            payload_len += ie.len();
        }
        for ie in &self.ies {
            payload_len += ie.len();
        }

        let mut header = Header::new(
            MsgType::SessionEstablishmentResponse,
            true,
            self.seid,
            self.seq,
        );
        header.length = payload_len + (header.len() - 4);

        Ok(SessionEstablishmentResponse {
            header,
            cause,
            offending_ie: self.offending_ie,
            fseid,
            created_pdrs: self.created_pdrs,
            pdn_type: self.pdn_type,
            load_control_information: self.load_control_information,
            overload_control_information: self.overload_control_information,
            ies: self.ies,
        })
    }

    /// Builds and marshals the SessionEstablishmentResponse in one step.
    pub fn marshal(self) -> Result<Vec<u8>, io::Error> {
        Ok(self.build()?.marshal())
    }
}
