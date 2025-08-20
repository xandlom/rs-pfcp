// tests/messages.rs

use rs_pfcp::ie::{Ie, IeType};
use rs_pfcp::message::session_report_response::SessionReportResponse;
use rs_pfcp::message::{header::Header, Message, MsgType};
use std::net::{Ipv4Addr, Ipv6Addr};

#[test]
fn test_heartbeat_request_marshal_unmarshal() {
    // 2019-01-01 00:00:00 UTC -> 3755289600
    let ts_payload = 3755289600_u32.to_be_bytes().to_vec();
    let ts_ie = Ie::new(IeType::RecoveryTimeStamp, ts_payload);

    let mut ip_payload = vec![0x03]; // V4 and V6
    ip_payload.extend_from_slice(&Ipv4Addr::new(127, 0, 0, 1).octets());
    let ipv6_addr = Ipv6Addr::new(0x2001, 0, 0, 0, 0, 0, 0, 1);
    ip_payload.extend_from_slice(&ipv6_addr.octets());
    let ip_ie = Ie::new(IeType::SourceIPAddress, ip_payload);

    let req = rs_pfcp::message::heartbeat_request::HeartbeatRequest::new(
        0x112233,
        Some(ts_ie.clone()),
        Some(ip_ie.clone()),
        vec![],
    );

    let serialized = req.marshal();

    // Unmarshal and compare
    let unmarshaled_req =
        rs_pfcp::message::heartbeat_request::HeartbeatRequest::unmarshal(&serialized).unwrap();
    assert_eq!(unmarshaled_req, req);
}

#[test]
fn test_heartbeat_response_marshal_unmarshal() {
    // 2019-01-01 00:00:00 UTC -> 3755289600
    let ts_payload = 3755289600_u32.to_be_bytes().to_vec();
    let ts_ie = Ie::new(IeType::RecoveryTimeStamp, ts_payload);

    let res = rs_pfcp::message::heartbeat_response::HeartbeatResponse::new(
        0x112233,
        Some(ts_ie.clone()),
        vec![],
    );

    let serialized = res.marshal();

    // Unmarshal and compare
    let unmarshaled_res =
        rs_pfcp::message::heartbeat_response::HeartbeatResponse::unmarshal(&serialized).unwrap();
    assert_eq!(unmarshaled_res, res);
}

#[test]
fn test_association_setup_response_marshal_unmarshal() {
    use rs_pfcp::ie::cause::CauseValue;
    use rs_pfcp::ie::node_id::NodeId;
    use rs_pfcp::message::association_setup_response::AssociationSetupResponse;

    let cause_ie = Ie::new(IeType::Cause, vec![CauseValue::RequestAccepted as u8]);
    let node_id = NodeId::new_ipv4(Ipv4Addr::new(127, 0, 0, 1));
    let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());
    let up_features_ie = Ie::new(IeType::UpFunctionFeatures, vec![0x01, 0x02]);
    let cp_features_ie = Ie::new(IeType::CpFunctionFeatures, vec![0x03]);
    let ts_ie = Ie::new(
        IeType::RecoveryTimeStamp,
        3755289600_u32.to_be_bytes().to_vec(),
    );

    let mut header = Header::new(MsgType::AssociationSetupResponse, false, 0, 0x112233);
    let payload_len = cause_ie.len()
        + node_id_ie.len()
        + up_features_ie.len()
        + cp_features_ie.len()
        + ts_ie.len();
    header.length = payload_len + header.len() - 4;

    let res = AssociationSetupResponse {
        header,
        cause: cause_ie.clone(),
        node_id: node_id_ie.clone(),
        up_function_features: Some(up_features_ie.clone()),
        cp_function_features: Some(cp_features_ie.clone()),
        recovery_time_stamp: Some(ts_ie.clone()),
        ies: vec![],
    };

    let serialized = res.marshal();
    let unmarshaled = AssociationSetupResponse::unmarshal(&serialized).unwrap();

    assert_eq!(res, unmarshaled);
}

#[test]
fn test_association_setup_response_from_request() {
    use rs_pfcp::ie::cause::CauseValue;
    use rs_pfcp::ie::node_id::NodeId;
    use rs_pfcp::message::association_setup_request::AssociationSetupRequest;
    use rs_pfcp::message::association_setup_response::AssociationSetupResponse;

    let node_id = NodeId::new_ipv4(Ipv4Addr::new(127, 0, 0, 1));
    let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());
    let ts_ie = Ie::new(
        IeType::RecoveryTimeStamp,
        3755289600_u32.to_be_bytes().to_vec(),
    );
    let up_features_ie = Ie::new(IeType::UpFunctionFeatures, vec![0x01, 0x02]);
    let cp_features_ie = Ie::new(IeType::CpFunctionFeatures, vec![0x03]);

    let req = AssociationSetupRequest::new(
        0x112233,
        node_id_ie.clone(),
        ts_ie.clone(),
        Some(up_features_ie.clone()),
        Some(cp_features_ie.clone()),
        vec![],
    );

    let cause_ie = Ie::new(IeType::Cause, vec![CauseValue::RequestAccepted as u8]);

    let mut header = Header::new(MsgType::AssociationSetupResponse, false, 0, req.sequence());
    header.length = 8
        + cause_ie.len()
        + req.find_ie(IeType::NodeId).unwrap().len()
        + req.find_ie(IeType::UpFunctionFeatures).unwrap().len()
        + req.find_ie(IeType::CpFunctionFeatures).unwrap().len()
        + req.find_ie(IeType::RecoveryTimeStamp).unwrap().len();

    let res = AssociationSetupResponse {
        header,
        cause: cause_ie.clone(),
        node_id: req.find_ie(IeType::NodeId).unwrap().clone(),
        up_function_features: req.find_ie(IeType::UpFunctionFeatures).cloned(),
        cp_function_features: req.find_ie(IeType::CpFunctionFeatures).cloned(),
        recovery_time_stamp: req.find_ie(IeType::RecoveryTimeStamp).cloned(),
        ies: vec![],
    };

    assert_eq!(res.msg_type(), MsgType::AssociationSetupResponse);
    assert_eq!(res.sequence(), req.sequence());
    assert_eq!(res.cause, cause_ie);
    assert_eq!(res.node_id, node_id_ie);
    assert_eq!(res.up_function_features, Some(up_features_ie));
    assert_eq!(res.cp_function_features, Some(cp_features_ie));
    assert_eq!(res.recovery_time_stamp, Some(ts_ie));
}

#[test]
fn test_association_update_request_marshal_unmarshal() {
    use rs_pfcp::ie::node_id::NodeId;
    use rs_pfcp::message::association_update_request::AssociationUpdateRequest;

    let node_id = NodeId::new_ipv4(Ipv4Addr::new(127, 0, 0, 1));
    let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());
    let up_features_ie = Ie::new(IeType::UpFunctionFeatures, vec![0x01, 0x02]);
    let cp_features_ie = Ie::new(IeType::CpFunctionFeatures, vec![0x03]);

    let mut header = Header::new(MsgType::AssociationUpdateRequest, false, 0, 0x112233);
    let payload_len = node_id_ie.len() + up_features_ie.len() + cp_features_ie.len();
    header.length = payload_len + header.len() - 4;

    let req = AssociationUpdateRequest {
        header,
        node_id: node_id_ie.clone(),
        up_function_features: Some(up_features_ie.clone()),
        cp_function_features: Some(cp_features_ie.clone()),
        ies: vec![],
    };

    let serialized = req.marshal();
    let unmarshaled = AssociationUpdateRequest::unmarshal(&serialized).unwrap();

    assert_eq!(req, unmarshaled);
}

#[test]
fn test_association_release_request_marshal_unmarshal() {
    use rs_pfcp::ie::node_id::NodeId;
    use rs_pfcp::message::association_release_request::AssociationReleaseRequest;

    let node_id = NodeId::new_ipv4(Ipv4Addr::new(127, 0, 0, 1));
    let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

    let mut header = Header::new(MsgType::AssociationReleaseRequest, false, 0, 0x112233);
    let payload_len = node_id_ie.len();
    header.length = payload_len + header.len() - 4;

    let req = AssociationReleaseRequest {
        header,
        node_id: node_id_ie.clone(),
    };

    let serialized = req.marshal();
    let unmarshaled = AssociationReleaseRequest::unmarshal(&serialized).unwrap();

    assert_eq!(req, unmarshaled);
}

#[test]
fn test_association_release_response_marshal_unmarshal() {
    use rs_pfcp::ie::cause::CauseValue;
    use rs_pfcp::ie::node_id::NodeId;
    use rs_pfcp::message::association_release_response::AssociationReleaseResponse;

    let cause_ie = Ie::new(IeType::Cause, vec![CauseValue::RequestAccepted as u8]);
    let node_id = NodeId::new_ipv4(Ipv4Addr::new(127, 0, 0, 1));
    let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());

    let mut header = Header::new(MsgType::AssociationReleaseResponse, false, 0, 0x112233);
    let payload_len = cause_ie.len() + node_id_ie.len();
    header.length = payload_len + header.len() - 4;

    let res = AssociationReleaseResponse {
        header,
        cause: cause_ie.clone(),
        node_id: node_id_ie.clone(),
    };

    let serialized = res.marshal();
    let unmarshaled = AssociationReleaseResponse::unmarshal(&serialized).unwrap();

    assert_eq!(res, unmarshaled);
}

#[test]
fn test_pfd_management_response_marshal_unmarshal() {
    use rs_pfcp::ie::cause::CauseValue;
    use rs_pfcp::message::pfd_management_response::PfdManagementResponse;

    let cause_ie = Ie::new(IeType::Cause, vec![CauseValue::RequestAccepted as u8]);
    let offending_ie = Ie::new(IeType::OffendingIe, vec![0x01, 0x02, 0x03, 0x04]);

    let res = PfdManagementResponse::new(
        0x112233,
        cause_ie.clone(),
        Some(offending_ie.clone()),
        vec![],
    );

    let serialized = res.marshal();
    let unmarshaled = PfdManagementResponse::unmarshal(&serialized).unwrap();

    assert_eq!(res, unmarshaled);
}

#[test]
fn test_session_establishment_request_marshal_unmarshal() {
    use rs_pfcp::ie::node_id::NodeId;
    use rs_pfcp::message::session_establishment_request::{
        SessionEstablishmentRequest, SessionEstablishmentRequestBuilder,
    };

    let node_id = NodeId::new_ipv4(Ipv4Addr::new(127, 0, 0, 1));
    let node_id_ie = Ie::new(IeType::NodeId, node_id.marshal());
    let fseid_ie = Ie::new(
        IeType::Fseid,
        vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
    );

    let pdr_ie = Ie::new(IeType::CreatePdr, vec![0x01, 0x02, 0x03, 0x04]);
    let far_ie = Ie::new(IeType::CreateFar, vec![0x05, 0x06, 0x07, 0x08]);
    let req = SessionEstablishmentRequestBuilder::new(0x1122334455667788, 0x112233)
        .node_id(node_id_ie.clone())
        .fseid(fseid_ie.clone())
        .create_pdrs(vec![pdr_ie])
        .create_fars(vec![far_ie])
        .build()
        .unwrap();

    let serialized = req.marshal();
    let unmarshaled = SessionEstablishmentRequest::unmarshal(&serialized).unwrap();

    assert_eq!(req, unmarshaled);
}

#[test]
fn test_session_deletion_request_marshal_unmarshal() {
    use rs_pfcp::message::session_deletion_request::SessionDeletionRequest;

    let fseid_ie = Ie::new(
        IeType::Fseid,
        vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
    );

    let req = SessionDeletionRequest::new(
        0x1122334455667788,
        0x112233,
        fseid_ie.clone(),
        None,
        None,
        None,
        vec![],
        vec![],
        vec![],
    );

    let serialized = req.marshal();
    let unmarshaled = SessionDeletionRequest::unmarshal(&serialized).unwrap();

    assert_eq!(req, unmarshaled);
}

#[test]
fn test_session_deletion_response_marshal_unmarshal() {
    use rs_pfcp::ie::cause::CauseValue;
    use rs_pfcp::message::session_deletion_response::SessionDeletionResponse;

    let cause_ie = Ie::new(IeType::Cause, vec![CauseValue::RequestAccepted as u8]);

    let res =
        SessionDeletionResponse::new(0x1122334455667788, 0x112233, cause_ie.clone(), None, vec![]);

    let serialized = res.marshal();
    let unmarshaled = SessionDeletionResponse::unmarshal(&serialized).unwrap();

    assert_eq!(res, unmarshaled);
}

#[test]
fn test_session_modification_request_marshal_unmarshal() {
    use rs_pfcp::message::session_modification_request::{
        SessionModificationRequest, SessionModificationRequestBuilder,
    };

    let fseid_ie = Ie::new(
        IeType::Fseid,
        vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
    );
    let pdr_ie = Ie::new(IeType::CreatePdr, vec![0x01, 0x02, 0x03, 0x04]);
    let far_ie = Ie::new(IeType::CreateFar, vec![0x05, 0x06, 0x07, 0x08]);
    let req = SessionModificationRequestBuilder::new(0x1122334455667788, 0x112233)
        .fseid(fseid_ie.clone())
        .create_pdrs(vec![pdr_ie.clone()])
        .create_fars(vec![far_ie.clone()])
        .build();

    let serialized = req.marshal();
    let unmarshaled = SessionModificationRequest::unmarshal(&serialized).unwrap();

    assert_eq!(req, unmarshaled);
}

#[test]
fn test_session_modification_response_marshal_unmarshal() {
    use rs_pfcp::ie::cause::CauseValue;
    use rs_pfcp::message::session_modification_response::SessionModificationResponse;

    let cause_ie = Ie::new(IeType::Cause, vec![CauseValue::RequestAccepted as u8]);
    let pdr_ie = Ie::new(IeType::CreatedPdr, vec![0x01, 0x02, 0x03, 0x04]);

    let mut header = Header::new(
        MsgType::SessionModificationResponse,
        true,
        0x1122334455667788,
        0x112233,
    );
    let payload_len = cause_ie.len() + pdr_ie.len();
    header.length = (payload_len + header.len() - 4) as u16;

    let res = SessionModificationResponse {
        header,
        cause: cause_ie.clone(),
        offending_ie: None,
        created_pdr: Some(pdr_ie.clone()),
        ies: vec![],
    };

    let serialized = res.marshal();
    let unmarshaled = SessionModificationResponse::unmarshal(&serialized).unwrap();

    assert_eq!(res, unmarshaled);
}

// Session Report Response tests
#[test]
fn test_session_report_response_marshal_unmarshal_minimal() {
    use rs_pfcp::ie::cause::CauseValue;
    use rs_pfcp::message::session_report_response::SessionReportResponse;

    let seid = 0x1122334455667788;
    let sequence = 0x112233;
    let cause_ie = Ie::new(IeType::Cause, vec![CauseValue::RequestAccepted as u8]);

    let res = SessionReportResponse::new(seid, sequence, cause_ie.clone(), None, vec![], vec![]);

    let serialized = res.marshal();
    let unmarshaled = SessionReportResponse::unmarshal(&serialized).unwrap();

    assert_eq!(res, unmarshaled);
    assert_eq!(res.msg_type(), MsgType::SessionReportResponse);
    assert_eq!(res.seid(), Some(seid));
    assert_eq!(res.sequence(), sequence);
    assert_eq!(res.find_ie(IeType::Cause), Some(&cause_ie));
}

#[test]
fn test_session_report_response_marshal_unmarshal_with_offending_ie() {
    use rs_pfcp::ie::cause::CauseValue;
    use rs_pfcp::message::session_report_response::SessionReportResponse;

    let seid = 0x1122334455667788;
    let sequence = 0x112233;
    let cause_ie = Ie::new(IeType::Cause, vec![CauseValue::MandatoryIeMissing as u8]);
    let offending_ie = Ie::new(IeType::OffendingIe, vec![0x01, 0x02, 0x03, 0x04]);

    let res = SessionReportResponse::new(
        seid,
        sequence,
        cause_ie.clone(),
        Some(offending_ie.clone()),
        vec![],
        vec![],
    );

    let serialized = res.marshal();
    let unmarshaled = SessionReportResponse::unmarshal(&serialized).unwrap();

    assert_eq!(res, unmarshaled);
    assert_eq!(res.find_ie(IeType::Cause), Some(&cause_ie));
    assert_eq!(res.find_ie(IeType::OffendingIe), Some(&offending_ie));
}

#[test]
fn test_session_report_response_marshal_unmarshal_with_usage_reports() {
    use rs_pfcp::ie::cause::CauseValue;
    use rs_pfcp::ie::sequence_number::SequenceNumber;
    use rs_pfcp::ie::urr_id::UrrId;
    use rs_pfcp::ie::usage_report::UsageReport;
    use rs_pfcp::ie::usage_report_trigger::UsageReportTrigger;
    use rs_pfcp::message::session_report_response::SessionReportResponse;

    let seid = 0x1122334455667788;
    let sequence = 0x112233;
    let cause_ie = Ie::new(IeType::Cause, vec![CauseValue::RequestAccepted as u8]);

    // Create usage report IE
    let urr_id = UrrId::new(1);
    let ur_seqn = SequenceNumber::new(1);
    let usage_report_trigger = UsageReportTrigger::new(1);
    let usage_report = UsageReport::new(urr_id, ur_seqn, usage_report_trigger);
    let usage_report_ie = usage_report.to_ie();

    let usage_reports = vec![usage_report_ie.clone()];

    let res = SessionReportResponse::new(
        seid,
        sequence,
        cause_ie.clone(),
        None,
        usage_reports.clone(),
        vec![],
    );

    let serialized = res.marshal();
    let unmarshaled = SessionReportResponse::unmarshal(&serialized).unwrap();

    assert_eq!(res, unmarshaled);
    assert_eq!(res.usage_reports.len(), 1);
    assert_eq!(res.find_ie(IeType::UsageReport), Some(&usage_report_ie));
}

#[test]
fn test_session_report_response_builder() {
    use rs_pfcp::ie::cause::CauseValue;
    use rs_pfcp::message::session_report_response::SessionReportResponseBuilder;

    let seid = 0x1122334455667788;
    let sequence = 0x112233;
    let cause_ie = Ie::new(IeType::Cause, vec![CauseValue::RequestAccepted as u8]);
    let usage_report_ie = Ie::new(IeType::UsageReport, vec![0x01, 0x02, 0x03, 0x04]);
    let cp_features_ie = Ie::new(IeType::CpFunctionFeatures, vec![0x05, 0x06]);

    let res = SessionReportResponseBuilder::new(seid, sequence, cause_ie.clone())
        .usage_reports(vec![usage_report_ie.clone()])
        .cp_function_features(cp_features_ie.clone())
        .build()
        .unwrap();

    assert_eq!(res.msg_type(), MsgType::SessionReportResponse);
    assert_eq!(res.seid(), Some(seid));
    assert_eq!(res.sequence(), sequence);
    assert_eq!(res.cause, cause_ie);
    assert_eq!(res.usage_reports, vec![usage_report_ie]);
    assert_eq!(res.cp_function_features, Some(cp_features_ie));

    let serialized = res.marshal();
    let unmarshaled = SessionReportResponse::unmarshal(&serialized).unwrap();
    assert_eq!(res, unmarshaled);
}

#[test]
fn test_session_report_response_builder_comprehensive() {
    use rs_pfcp::ie::cause::CauseValue;
    use rs_pfcp::message::session_report_response::SessionReportResponseBuilder;

    let seid = 0x1122334455667788;
    let sequence = 0x112233;
    let cause_ie = Ie::new(IeType::Cause, vec![CauseValue::RequestAccepted as u8]);
    let offending_ie = Ie::new(IeType::OffendingIe, vec![0x01, 0x02, 0x03, 0x04]);
    let update_bar_ie = Ie::new(IeType::UpdateBar, vec![0x05, 0x06]);
    let pfcpsrrsp_flags_ie = Ie::new(IeType::PfcpsrrspFlags, vec![0x07]);
    let cp_features_ie = Ie::new(IeType::CpFunctionFeatures, vec![0x08, 0x09]);

    // Create multiple usage reports
    let usage_report1 = Ie::new(IeType::UsageReport, vec![0x0A, 0x0B, 0x0C]);
    let usage_report2 = Ie::new(IeType::UsageReport, vec![0x0D, 0x0E, 0x0F]);
    let usage_reports = vec![usage_report1, usage_report2];

    let additional_ie = Ie::new(IeType::Timer, vec![0x10, 0x11, 0x12, 0x13]);

    let res = SessionReportResponseBuilder::new(seid, sequence, cause_ie.clone())
        .offending_ie(offending_ie.clone())
        .update_bar(update_bar_ie.clone())
        .pfcpsrrsp_flags(pfcpsrrsp_flags_ie.clone())
        .cp_function_features(cp_features_ie.clone())
        .usage_reports(usage_reports.clone())
        .ies(vec![additional_ie.clone()])
        .build()
        .unwrap();

    assert_eq!(res.cause, cause_ie);
    assert_eq!(res.offending_ie, Some(offending_ie));
    assert_eq!(res.update_bar, Some(update_bar_ie));
    assert_eq!(res.pfcpsrrsp_flags, Some(pfcpsrrsp_flags_ie));
    assert_eq!(res.cp_function_features, Some(cp_features_ie));
    assert_eq!(res.usage_reports, usage_reports);
    assert_eq!(res.ies, vec![additional_ie]);

    let serialized = res.marshal();
    let unmarshaled = SessionReportResponse::unmarshal(&serialized).unwrap();
    assert_eq!(res, unmarshaled);
}

#[test]
fn test_session_report_response_set_sequence() {
    use rs_pfcp::ie::cause::CauseValue;
    use rs_pfcp::message::session_report_response::SessionReportResponse;

    let seid = 0x1122334455667788;
    let sequence = 0x112233;
    let new_sequence = 0x445566;
    let cause_ie = Ie::new(IeType::Cause, vec![CauseValue::RequestAccepted as u8]);

    let mut res = SessionReportResponse::new(seid, sequence, cause_ie, None, vec![], vec![]);

    assert_eq!(res.sequence(), sequence);
    res.set_sequence(new_sequence);
    assert_eq!(res.sequence(), new_sequence);
}

#[test]
fn test_session_report_response_find_ie() {
    use rs_pfcp::ie::cause::CauseValue;
    use rs_pfcp::message::session_report_response::SessionReportResponseBuilder;

    let seid = 0x1122334455667788;
    let sequence = 0x112233;
    let cause_ie = Ie::new(IeType::Cause, vec![CauseValue::RequestAccepted as u8]);
    let usage_report_ie = Ie::new(IeType::UsageReport, vec![0x01, 0x02]);
    let unknown_ie = Ie::new(IeType::Timer, vec![0x03, 0x04]);

    let res = SessionReportResponseBuilder::new(seid, sequence, cause_ie.clone())
        .usage_reports(vec![usage_report_ie.clone()])
        .ies(vec![unknown_ie.clone()])
        .build()
        .unwrap();

    assert_eq!(res.find_ie(IeType::Cause), Some(&cause_ie));
    assert_eq!(res.find_ie(IeType::UsageReport), Some(&usage_report_ie));
    assert_eq!(res.find_ie(IeType::Timer), Some(&unknown_ie));
    assert_eq!(res.find_ie(IeType::NodeId), None);
}

#[test]
fn test_session_report_response_empty_unmarshal() {
    use rs_pfcp::ie::cause::CauseValue;
    use rs_pfcp::message::session_report_response::SessionReportResponse;

    let seid = 0x1122334455667788;
    let sequence = 0x112233;
    let cause_ie = Ie::new(IeType::Cause, vec![CauseValue::RequestAccepted as u8]);

    let mut header = Header::new(MsgType::SessionReportResponse, true, seid, sequence);
    header.length = cause_ie.len() + (header.len() - 4);

    let mut serialized = header.marshal();
    serialized.extend_from_slice(&cause_ie.marshal());

    let unmarshaled = SessionReportResponse::unmarshal(&serialized).unwrap();

    assert_eq!(unmarshaled.msg_type(), MsgType::SessionReportResponse);
    assert_eq!(unmarshaled.seid(), Some(seid));
    assert_eq!(unmarshaled.sequence(), sequence);
    assert_eq!(unmarshaled.cause, cause_ie);
    assert!(unmarshaled.offending_ie.is_none());
    assert!(unmarshaled.usage_reports.is_empty());
    assert!(unmarshaled.ies.is_empty());
}

#[test]
fn test_session_report_response_unmarshal_missing_cause() {
    use rs_pfcp::message::session_report_response::SessionReportResponse;

    let seid = 0x1122334455667788;
    let sequence = 0x112233;

    // Create header without cause IE
    let header = Header::new(MsgType::SessionReportResponse, true, seid, sequence);
    let serialized = header.marshal();

    let result = SessionReportResponse::unmarshal(&serialized);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidData);
}

#[test]
fn test_session_establishment_response_multiple_created_pdrs() {
    use rs_pfcp::ie::{cause::Cause, created_pdr::CreatedPdr, f_teid::Fteid, pdr_id::PdrId, fseid::Fseid};
    use rs_pfcp::message::session_establishment_response::SessionEstablishmentResponseBuilder;

    // Test SessionEstablishmentResponse with multiple Created PDR IEs
    let seid = 0x0000000000000001;
    let sequence = 2;
    
    // Create cause IE
    let cause_ie = Ie::new(IeType::Cause, Cause::new(1.into()).marshal().to_vec());
    
    // Create F-SEID IE
    let fseid = Fseid::new(0x0102030405060709u64, Some(Ipv4Addr::new(127, 0, 0, 1)), None);
    let fseid_ie = Ie::new(IeType::Fseid, fseid.marshal());
    
    // Create two Created PDR IEs with different PDR IDs and F-TEIDs
    let fteid1 = Fteid::new(true, false, 0x12345679, Some(Ipv4Addr::new(192, 168, 1, 100)), None, 0);
    let created_pdr1 = CreatedPdr::new(PdrId::new(1), fteid1);
    let created_pdr1_ie = created_pdr1.to_ie();
    
    let fteid2 = Fteid::new(true, false, 0x1234567a, Some(Ipv4Addr::new(192, 168, 1, 100)), None, 0);
    let created_pdr2 = CreatedPdr::new(PdrId::new(2), fteid2);
    let created_pdr2_ie = created_pdr2.to_ie();
    
    // Build SessionEstablishmentResponse with multiple Created PDRs using the builder pattern
    let response = SessionEstablishmentResponseBuilder::new(seid, sequence, cause_ie)
        .fseid(fseid_ie)
        .created_pdr(created_pdr1_ie)
        .created_pdr(created_pdr2_ie)
        .build()
        .unwrap();
    
    // Verify the response contains both Created PDR IEs
    assert_eq!(response.created_pdrs.len(), 2);
    assert_eq!(response.seid(), Some(seid));
    assert_eq!(response.sequence(), sequence);
    
    // Marshal and unmarshal to test round-trip
    let marshaled = response.marshal();
    let unmarshaled = rs_pfcp::message::session_establishment_response::SessionEstablishmentResponse::unmarshal(&marshaled).unwrap();
    
    // Verify unmarshaled response has both Created PDR IEs
    assert_eq!(unmarshaled.created_pdrs.len(), 2);
    assert_eq!(unmarshaled.seid(), Some(seid));
    assert_eq!(unmarshaled.sequence(), sequence);
    
    // Verify the Created PDR contents
    let created_pdr1_unmarshaled = CreatedPdr::unmarshal(&unmarshaled.created_pdrs[0].payload).unwrap();
    let created_pdr2_unmarshaled = CreatedPdr::unmarshal(&unmarshaled.created_pdrs[1].payload).unwrap();
    
    assert_eq!(created_pdr1_unmarshaled.pdr_id.value, 1);
    assert_eq!(created_pdr1_unmarshaled.f_teid.teid, 0x12345679);
    
    assert_eq!(created_pdr2_unmarshaled.pdr_id.value, 2);
    assert_eq!(created_pdr2_unmarshaled.f_teid.teid, 0x1234567a);
    
    // Verify the length field is correctly calculated
    let expected_length = marshaled.len() - 4; // Total length minus first 4 header bytes
    let header_length = u16::from_be_bytes([marshaled[2], marshaled[3]]);
    assert_eq!(header_length as usize, expected_length);
}
