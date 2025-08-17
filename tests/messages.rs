// tests/messages.rs

use rs_pfcp::ie::{Ie, IeType};
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

    let req = SessionDeletionRequest::new(0x1122334455667788, 0x112233, fseid_ie.clone(), vec![]);

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
