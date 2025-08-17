use crate::ie::{
    cause::Cause, f_teid::Fteid, header::{Header, IE_HEADER_SIZE},
    recovery_time_stamp::RecoveryTimeStamp, up_function_features::UpFunctionFeatures,
    alternate_smf_ip_address::AlternateSmfIpAddress, network_instance::NetworkInstance,
};

pub const SESSION_REPORT_RESPONSE: u8 = 53;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SessionReportResponse {
    pub header: Header,
    pub cause: Cause,
    pub cp_f_seid: Option<Fteid>,
    pub up_function_features: Option<UpFunctionFeatures>,
    pub alternate_smf_ip_address: Option<AlternateSmfIpAddress>,
    pub recovery_time_stamp: Option<RecoveryTimeStamp>,
    pub network_instance: Option<NetworkInstance>,
}

impl SessionReportResponse {
    pub fn new(
        cause: Cause,
        cp_f_seid: Option<Fteid>,
        up_function_features: Option<UpFunctionFeatures>,
        alternate_smf_ip_address: Option<AlternateSmfIpAddress>,
        recovery_time_stamp: Option<RecoveryTimeStamp>,
        network_instance: Option<NetworkInstance>,
        sequence: u32,
        seid: u64,
    ) -> Self {
        let mut message = SessionReportResponse {
            header: Header {
                version: 1,
                message_type: SESSION_REPORT_RESPONSE,
                message_length: 0,
                seid,
                sequence,
                ..Default::default()
            },
            cause,
            cp_f_seid,
            up_function_features,
            alternate_smf_ip_address,
            recovery_time_stamp,
            network_instance,
        };
        message.set_length();
        message
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut buffer = self.header.marshal();
        buffer.extend_from_slice(&self.cause.marshal());
        if let Some(cp_f_seid) = &self.cp_f_seid {
            buffer.extend_from_slice(&cp_f_seid.marshal());
        }
        if let Some(up_function_features) = &self.up_function_features {
            buffer.extend_from_slice(&up_function_features.marshal());
        }
        if let Some(alternate_smf_ip_address) = &self.alternate_smf_ip_address {
            buffer.extend_from_slice(&alternate_smf_ip_address.marshal());
        }
        if let Some(recovery_time_stamp) = &self.recovery_time_stamp {
            buffer.extend_from_slice(&recovery_time_stamp.marshal());
        }
        if let Some(network_instance) = &self.network_instance {
            buffer.extend_from_slice(&network_instance.marshal());
        }
        buffer
    }

    pub fn unmarshal(buffer: &[u8]) -> Result<Self, String> {
        let mut message = SessionReportResponse::default();
        message.header.unmarshal(buffer)?;

        let mut offset = message.header.get_length() as usize;
        while offset < buffer.len() {
            let header = Header::unmarshal(&buffer[offset..])?;
            match header.ie_type {
                crate::ie::cause::CAUSE => {
                    message.cause = Cause::unmarshal(&buffer[offset..])?;
                }
                crate::ie::f_teid::F_TEID => {
                    message.cp_f_seid = Some(Fteid::unmarshal(&buffer[offset..])?);
                }
                crate::ie::up_function_features::UP_FUNCTION_FEATURES => {
                    message.up_function_features =
                        Some(UpFunctionFeatures::unmarshal(&buffer[offset..])?);
                }
                crate::ie::alternate_smf_ip_address::ALTERNATE_SMF_IP_ADDRESS => {
                    message.alternate_smf_ip_address =
                        Some(AlternateSmfIpAddress::unmarshal(&buffer[offset..])?);
                }
                crate::ie::recovery_time_stamp::RECOVERY_TIME_STAMP => {
                    message.recovery_time_stamp =
                        Some(RecoveryTimeStamp::unmarshal(&buffer[offset..])?);
                }
                crate::ie::network_instance::NETWORK_INSTANCE => {
                    message.network_instance = Some(NetworkInstance::unmarshal(&buffer[offset..])?);
                }
                _ => {
                    // Ignore unknown IEs
                }
            }
            offset += (header.ie_length + IE_HEADER_SIZE) as usize;
        }
        Ok(message)
    }

    fn set_length(&mut self) {
        let mut length = self.cause.get_length();
        if let Some(cp_f_seid) = &self.cp_f_seid {
            length += cp_f_seid.get_length();
        }
        if let Some(up_function_features) = &self.up_function_features {
            length += up_function_features.get_length();
        }
        if let Some(alternate_smf_ip_address) = &self.alternate_smf_ip_address {
            length += alternate_smf_ip_address.get_length();
        }
        if let Some(recovery_time_stamp) = &self.recovery_time_stamp {
            length += recovery_time_stamp.get_length();
        }
        if let Some(network_instance) = &self.network_instance {
            length += network_instance.get_length();
        }
        self.header.message_length = length;
    }
}
