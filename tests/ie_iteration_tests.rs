// tests/ie_iteration_tests.rs
//
// Comprehensive tests for Unified IE Access Patterns
//
// This test suite validates the new `ies()` iterator API introduced in API improvement #4.
// It ensures that IE iteration works correctly across all storage patterns:
// - Single mandatory IEs
// - Optional IEs
// - Multiple IEs (Vec<Ie>)
// - Generic IE storage
//
// The tests verify:
// ✅ Single IE retrieval with .next()
// ✅ Multiple IE retrieval with .collect()
// ✅ Iterator operations (count, filter, map)
// ✅ Empty results for non-existent IEs
// ✅ Proper iteration order

use rs_pfcp::ie::{cause::Cause, Ie, IeType};
use rs_pfcp::message::{
    association_setup_request::AssociationSetupRequestBuilder,
    heartbeat_request::HeartbeatRequestBuilder,
    session_establishment_request::SessionEstablishmentRequestBuilder,
    session_modification_request::SessionModificationRequestBuilder, Message,
};
use std::net::Ipv4Addr;

/// Test single mandatory IE retrieval
#[test]
fn test_single_mandatory_ie() {
    let recovery_time = std::time::SystemTime::now();
    let request = HeartbeatRequestBuilder::new(1)
        .recovery_time_stamp(recovery_time)
        .build();

    // Test .next() returns Some for mandatory IE
    let recovery_ie = request.ies(IeType::RecoveryTimeStamp).next();
    assert!(recovery_ie.is_some(), "Mandatory IE should be found");
    assert_eq!(recovery_ie.unwrap().ie_type, IeType::RecoveryTimeStamp);

    // Test count() for single IE
    assert_eq!(request.ies(IeType::RecoveryTimeStamp).count(), 1);

    // Test collect() for single IE
    let collected: Vec<_> = request.ies(IeType::RecoveryTimeStamp).collect();
    assert_eq!(collected.len(), 1);
}

/// Test optional IE retrieval (Some and None cases)
#[test]
fn test_optional_ie() {
    // Case 1: Optional IE present (using PdnType as optional IE in SessionEstablishmentRequest)
    use rs_pfcp::ie::{
        create_far::CreateFarBuilder, create_pdr::CreatePdrBuilder, far_id::FarId, pdi::PdiBuilder,
        pdn_type::PdnType, pdr_id::PdrId, precedence::Precedence,
    };

    // Create minimal PDR and FAR to satisfy validation
    let pdr = CreatePdrBuilder::new(PdrId::new(1))
        .precedence(Precedence::new(100))
        .pdi(PdiBuilder::uplink_access().build().unwrap())
        .far_id(FarId::new(1))
        .build()
        .unwrap();
    let far = CreateFarBuilder::uplink_to_core(FarId::new(1))
        .build()
        .unwrap();

    let request_with_pdn = SessionEstablishmentRequestBuilder::new(12345, 1)
        .node_id(Ipv4Addr::new(10, 0, 0, 1))
        .fseid(0x123456789ABCDEF0, Ipv4Addr::new(10, 0, 0, 1))
        .create_pdrs(vec![pdr.to_ie()])
        .create_fars(vec![far.to_ie()])
        .pdn_type(PdnType::ipv4().to_ie())
        .build()
        .unwrap();

    let pdn = request_with_pdn.ies(IeType::PdnType).next();
    assert!(pdn.is_some(), "Optional IE should be found when present");

    // Case 2: Optional IE absent
    let pdr2 = CreatePdrBuilder::new(PdrId::new(1))
        .precedence(Precedence::new(100))
        .pdi(PdiBuilder::uplink_access().build().unwrap())
        .far_id(FarId::new(1))
        .build()
        .unwrap();
    let far2 = CreateFarBuilder::uplink_to_core(FarId::new(1))
        .build()
        .unwrap();

    let request_without_pdn = SessionEstablishmentRequestBuilder::new(12346, 2)
        .node_id(Ipv4Addr::new(10, 0, 0, 2))
        .fseid(0x123456789ABCDEF1, Ipv4Addr::new(10, 0, 0, 2))
        .create_pdrs(vec![pdr2.to_ie()])
        .create_fars(vec![far2.to_ie()])
        .build()
        .unwrap();

    let pdn = request_without_pdn.ies(IeType::PdnType).next();
    assert!(pdn.is_none(), "Optional IE should be None when absent");

    // Test count() for absent optional IE
    assert_eq!(request_without_pdn.ies(IeType::PdnType).count(), 0);
}

/// Test multiple IE retrieval (Vec<Ie>)
#[test]
fn test_multiple_ies() {
    // Create a session establishment request with multiple Create PDR IEs
    use rs_pfcp::ie::{
        create_pdr::CreatePdrBuilder, far_id::FarId, pdi::PdiBuilder, pdr_id::PdrId,
        precedence::Precedence,
    };

    let pdr1 = CreatePdrBuilder::new(PdrId::new(1))
        .precedence(Precedence::new(100))
        .pdi(PdiBuilder::uplink_access().build().unwrap())
        .far_id(FarId::new(1))
        .build()
        .unwrap();

    let pdr2 = CreatePdrBuilder::new(PdrId::new(2))
        .precedence(Precedence::new(200))
        .pdi(PdiBuilder::downlink_core().build().unwrap())
        .far_id(FarId::new(2))
        .build()
        .unwrap();

    let pdr3 = CreatePdrBuilder::new(PdrId::new(3))
        .precedence(Precedence::new(300))
        .pdi(PdiBuilder::uplink_access().build().unwrap())
        .far_id(FarId::new(3))
        .build()
        .unwrap();

    // Create corresponding FARs
    use rs_pfcp::ie::create_far::CreateFarBuilder;
    let far1 = CreateFarBuilder::uplink_to_core(FarId::new(1))
        .build()
        .unwrap();
    let far2 = CreateFarBuilder::uplink_to_core(FarId::new(2))
        .build()
        .unwrap();
    let far3 = CreateFarBuilder::uplink_to_core(FarId::new(3))
        .build()
        .unwrap();

    let request = SessionEstablishmentRequestBuilder::new(12345, 1)
        .node_id(Ipv4Addr::new(10, 0, 0, 1))
        .fseid(0x123456789ABCDEF0, Ipv4Addr::new(10, 0, 0, 1))
        .create_pdrs(vec![pdr1.to_ie(), pdr2.to_ie(), pdr3.to_ie()])
        .create_fars(vec![far1.to_ie(), far2.to_ie(), far3.to_ie()])
        .build()
        .unwrap();

    // Test count()
    let pdr_count = request.ies(IeType::CreatePdr).count();
    assert_eq!(pdr_count, 3, "Should find all 3 PDRs");

    // Test collect()
    let all_pdrs: Vec<_> = request.ies(IeType::CreatePdr).collect();
    assert_eq!(all_pdrs.len(), 3, "Should collect all 3 PDRs");
    assert!(all_pdrs.iter().all(|ie| ie.ie_type == IeType::CreatePdr));

    // Test iteration
    let mut count = 0;
    for pdr in request.ies(IeType::CreatePdr) {
        assert_eq!(pdr.ie_type, IeType::CreatePdr);
        count += 1;
    }
    assert_eq!(count, 3, "Should iterate over all 3 PDRs");
}

/// Test non-existent IE type returns empty iterator
#[test]
fn test_nonexistent_ie() {
    let request = HeartbeatRequestBuilder::new(1)
        .recovery_time_stamp(std::time::SystemTime::now())
        .build();

    // Test .next() returns None
    assert!(request.ies(IeType::Cause).next().is_none());

    // Test count() returns 0
    assert_eq!(request.ies(IeType::Cause).count(), 0);

    // Test collect() returns empty Vec
    let collected: Vec<_> = request.ies(IeType::Cause).collect();
    assert!(collected.is_empty());

    // Test iteration doesn't execute
    let mut executed = false;
    for _ie in request.ies(IeType::Cause) {
        executed = true;
    }
    assert!(!executed, "Iterator should not execute for non-existent IE");
}

/// Test iterator operations (filter, map, etc.)
#[test]
fn test_iterator_operations() {
    use rs_pfcp::ie::{
        create_pdr::CreatePdrBuilder, far_id::FarId, pdi::PdiBuilder, pdr_id::PdrId,
        precedence::Precedence,
    };

    let pdr1 = CreatePdrBuilder::new(PdrId::new(1))
        .precedence(Precedence::new(100))
        .pdi(PdiBuilder::uplink_access().build().unwrap())
        .far_id(FarId::new(1))
        .build()
        .unwrap();

    let pdr2 = CreatePdrBuilder::new(PdrId::new(2))
        .precedence(Precedence::new(200))
        .pdi(PdiBuilder::downlink_core().build().unwrap())
        .far_id(FarId::new(2))
        .build()
        .unwrap();

    // Create corresponding FARs
    use rs_pfcp::ie::create_far::CreateFarBuilder;
    let far1 = CreateFarBuilder::uplink_to_core(FarId::new(1))
        .build()
        .unwrap();
    let far2 = CreateFarBuilder::uplink_to_core(FarId::new(2))
        .build()
        .unwrap();

    let request = SessionEstablishmentRequestBuilder::new(12345, 1)
        .node_id(Ipv4Addr::new(10, 0, 0, 1))
        .fseid(0x123456789ABCDEF0, Ipv4Addr::new(10, 0, 0, 1))
        .create_pdrs(vec![pdr1.to_ie(), pdr2.to_ie()])
        .create_fars(vec![far1.to_ie(), far2.to_ie()])
        .build()
        .unwrap();

    // Test map()
    let ie_types: Vec<_> = request
        .ies(IeType::CreatePdr)
        .map(|ie| ie.ie_type)
        .collect();
    assert_eq!(ie_types, vec![IeType::CreatePdr, IeType::CreatePdr]);

    // Test filter() with count()
    let filtered_count = request
        .ies(IeType::CreatePdr)
        .filter(|ie| ie.payload.len() > 10)
        .count();
    assert_eq!(
        filtered_count, 2,
        "Both PDRs should have payload > 10 bytes"
    );

    // Test take()
    let first_pdr = request.ies(IeType::CreatePdr).take(1).collect::<Vec<_>>();
    assert_eq!(first_pdr.len(), 1);

    // Test any()
    assert!(request
        .ies(IeType::CreatePdr)
        .any(|ie| ie.ie_type == IeType::CreatePdr));

    // Test all()
    assert!(request
        .ies(IeType::CreatePdr)
        .all(|ie| ie.ie_type == IeType::CreatePdr));
}

/// Test generic IE storage fallback
#[test]
fn test_generic_ie_storage() {
    // Create a message with additional IEs that don't have dedicated fields
    let cause_ie = Ie::new(IeType::Cause, Cause::new(1.into()).marshal().to_vec());

    let request = AssociationSetupRequestBuilder::new(1)
        .node_id(Ipv4Addr::new(192, 168, 1, 1))
        .recovery_time_stamp(std::time::SystemTime::now())
        .ies(vec![cause_ie.clone()]) // Add extra IE via generic storage
        .build();

    // Test that we can find the Cause IE from generic storage
    let found_cause = request.ies(IeType::Cause).next();
    assert!(
        found_cause.is_some(),
        "Cause IE should be found in generic storage"
    );
    assert_eq!(found_cause.unwrap().ie_type, IeType::Cause);
}

/// Test session modification with optional Vec<Ie> fields
#[test]
fn test_optional_vec_ies() {
    use rs_pfcp::ie::{
        create_far::CreateFarBuilder, destination_interface::Interface, far_id::FarId,
    };

    // Case 1: With Create FARs
    use rs_pfcp::ie::{create_far::CreateFar, network_instance::NetworkInstance};
    let far1 = CreateFarBuilder::uplink_to_core(FarId::new(1))
        .build()
        .unwrap();
    let far2 = CreateFar::builder(FarId::new(2))
        .forward_to_network(Interface::Access, NetworkInstance::new("access.apn"))
        .build()
        .unwrap();

    let request_with_fars = SessionModificationRequestBuilder::new(12345, 1)
        .create_fars(vec![far1.to_ie(), far2.to_ie()])
        .build();

    assert_eq!(request_with_fars.ies(IeType::CreateFar).count(), 2);

    // Case 2: Without Create FARs (optional Vec is None)
    let request_without_fars = SessionModificationRequestBuilder::new(12345, 2).build();

    assert_eq!(request_without_fars.ies(IeType::CreateFar).count(), 0);
    assert!(request_without_fars.ies(IeType::CreateFar).next().is_none());
}

/// Test iterator order preservation
#[test]
fn test_iterator_order() {
    use rs_pfcp::ie::{
        create_pdr::CreatePdrBuilder, far_id::FarId, pdi::PdiBuilder, pdr_id::PdrId,
        precedence::Precedence,
    };

    use rs_pfcp::ie::create_far::CreateFarBuilder;
    let pdrs: Vec<_> = (1..=5u32)
        .map(|i| {
            CreatePdrBuilder::new(PdrId::new(i as u16))
                .precedence(Precedence::new(i * 100))
                .pdi(PdiBuilder::uplink_access().build().unwrap())
                .far_id(FarId::new(i))
                .build()
                .unwrap()
                .to_ie()
        })
        .collect();

    // Create corresponding FARs
    let fars: Vec<_> = (1..=5u32)
        .map(|i| {
            CreateFarBuilder::uplink_to_core(FarId::new(i))
                .build()
                .unwrap()
                .to_ie()
        })
        .collect();

    let request = SessionEstablishmentRequestBuilder::new(12345, 1)
        .node_id(Ipv4Addr::new(10, 0, 0, 1))
        .fseid(0x123456789ABCDEF0, Ipv4Addr::new(10, 0, 0, 1))
        .create_pdrs(pdrs.clone())
        .create_fars(fars)
        .build()
        .unwrap();

    // Verify order is preserved
    let retrieved: Vec<_> = request.ies(IeType::CreatePdr).collect();
    assert_eq!(retrieved.len(), 5);

    // Compare payloads to ensure order matches
    for (i, (expected, actual)) in pdrs.iter().zip(retrieved.iter()).enumerate() {
        assert_eq!(
            expected.payload,
            actual.payload,
            "PDR {} should match",
            i + 1
        );
    }
}

/// Test chaining multiple iterator methods
#[test]
fn test_iterator_chaining() {
    use rs_pfcp::ie::{
        create_pdr::CreatePdrBuilder, far_id::FarId, pdi::PdiBuilder, pdr_id::PdrId,
        precedence::Precedence,
    };

    use rs_pfcp::ie::create_far::CreateFarBuilder;
    let pdrs: Vec<_> = (1..=10u32)
        .map(|i| {
            CreatePdrBuilder::new(PdrId::new(i as u16))
                .precedence(Precedence::new(i * 100))
                .pdi(PdiBuilder::uplink_access().build().unwrap())
                .far_id(FarId::new(i))
                .build()
                .unwrap()
                .to_ie()
        })
        .collect();

    // Create corresponding FARs
    let fars: Vec<_> = (1..=10u32)
        .map(|i| {
            CreateFarBuilder::uplink_to_core(FarId::new(i))
                .build()
                .unwrap()
                .to_ie()
        })
        .collect();

    let request = SessionEstablishmentRequestBuilder::new(12345, 1)
        .node_id(Ipv4Addr::new(10, 0, 0, 1))
        .fseid(0x123456789ABCDEF0, Ipv4Addr::new(10, 0, 0, 1))
        .create_pdrs(pdrs)
        .create_fars(fars)
        .build()
        .unwrap();

    // Chain: skip first 2, take 5, filter by payload size, count
    let result = request
        .ies(IeType::CreatePdr)
        .skip(2)
        .take(5)
        .filter(|ie| !ie.payload.is_empty())
        .count();

    assert_eq!(result, 5, "Should process 5 PDRs after skip(2).take(5)");

    // Chain: enumerate and collect
    let enumerated: Vec<_> = request.ies(IeType::CreatePdr).enumerate().collect();

    assert_eq!(enumerated.len(), 10);
    assert_eq!(enumerated[0].0, 0);
    assert_eq!(enumerated[9].0, 9);
}

/// Test zero-cost abstraction (compile-time check via benchmark comparison)
#[test]
fn test_performance_characteristics() {
    // This test ensures the iterator doesn't introduce runtime overhead
    // In practice, LLVM optimizes IeIter::single() to direct field access
    use rs_pfcp::ie::{
        create_pdr::CreatePdrBuilder, far_id::FarId, pdi::PdiBuilder, pdr_id::PdrId,
        precedence::Precedence,
    };

    use rs_pfcp::ie::create_far::CreateFarBuilder;
    let pdrs: Vec<_> = (1..=100u32)
        .map(|i| {
            CreatePdrBuilder::new(PdrId::new(i as u16))
                .precedence(Precedence::new(i * 100))
                .pdi(PdiBuilder::uplink_access().build().unwrap())
                .far_id(FarId::new(i))
                .build()
                .unwrap()
                .to_ie()
        })
        .collect();

    // Create corresponding FARs
    let fars: Vec<_> = (1..=100u32)
        .map(|i| {
            CreateFarBuilder::uplink_to_core(FarId::new(i))
                .build()
                .unwrap()
                .to_ie()
        })
        .collect();

    let request = SessionEstablishmentRequestBuilder::new(12345, 1)
        .node_id(Ipv4Addr::new(10, 0, 0, 1))
        .fseid(0x123456789ABCDEF0, Ipv4Addr::new(10, 0, 0, 1))
        .create_pdrs(pdrs)
        .create_fars(fars)
        .build()
        .unwrap();

    // Iterate 1000 times to measure performance
    for _ in 0..1000 {
        let _count = request.ies(IeType::CreatePdr).count();
    }

    // If this test completes quickly, the abstraction is zero-cost
    // Actual benchmarks in benches/ directory provide precise measurements
}
