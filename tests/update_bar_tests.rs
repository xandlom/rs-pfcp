// tests/update_bar_tests.rs - Tests for Update BAR IE (currently 0% coverage)

use rs_pfcp::error::PfcpError;
use rs_pfcp::ie::bar_id::BarId;
use rs_pfcp::ie::suggested_buffering_packets_count::SuggestedBufferingPacketsCount;
use rs_pfcp::ie::update_bar::UpdateBar;
use rs_pfcp::ie::IeType;

#[test]
fn test_update_bar_new_basic() {
    let bar_id = BarId::new(1);
    let update_bar = UpdateBar::new(bar_id.clone(), None);

    assert_eq!(update_bar.bar_id, bar_id);
    assert!(update_bar.suggested_buffering_packets_count.is_none());
}

#[test]
fn test_update_bar_new_with_buffering_count() {
    let bar_id = BarId::new(2);
    let packet_count = SuggestedBufferingPacketsCount::new(100);

    let update_bar = UpdateBar::new(bar_id.clone(), Some(packet_count.clone()));

    assert_eq!(update_bar.bar_id, bar_id);
    assert_eq!(
        update_bar.suggested_buffering_packets_count,
        Some(packet_count)
    );
}

#[test]
fn test_update_bar_marshal_minimal() {
    let bar_id = BarId::new(5);
    let update_bar = UpdateBar::new(bar_id, None);

    let marshaled = update_bar.marshal();

    // Should at least contain the BAR ID IE
    assert!(!marshaled.is_empty());
    assert!(marshaled.len() >= 5); // IE header (4 bytes) + BAR ID value (1 byte min)
}

#[test]
fn test_update_bar_marshal_with_count() {
    let bar_id = BarId::new(10);
    let packet_count = SuggestedBufferingPacketsCount::new(200);
    let update_bar = UpdateBar::new(bar_id, Some(packet_count));

    let marshaled = update_bar.marshal();

    // Should contain both IEs
    assert!(!marshaled.is_empty());
}

#[test]
fn test_update_bar_unmarshal_minimal() {
    let bar_id = BarId::new(7);
    let update_bar = UpdateBar::new(bar_id.clone(), None);

    let marshaled = update_bar.marshal();
    let unmarshaled = UpdateBar::unmarshal(&marshaled).unwrap();

    assert_eq!(unmarshaled.bar_id, bar_id);
    assert!(unmarshaled.suggested_buffering_packets_count.is_none());
}

#[test]
fn test_update_bar_unmarshal_with_count() {
    let bar_id = BarId::new(15);
    let packet_count = SuggestedBufferingPacketsCount::new(50);
    let update_bar = UpdateBar::new(bar_id.clone(), Some(packet_count.clone()));

    let marshaled = update_bar.marshal();
    let unmarshaled = UpdateBar::unmarshal(&marshaled).unwrap();

    assert_eq!(unmarshaled.bar_id, bar_id);
    assert_eq!(
        unmarshaled.suggested_buffering_packets_count,
        Some(packet_count)
    );
}

#[test]
fn test_update_bar_round_trip() {
    let bar_id = BarId::new(42);
    let packet_count = SuggestedBufferingPacketsCount::new(100);
    let update_bar = UpdateBar::new(bar_id, Some(packet_count));

    let marshaled = update_bar.marshal();
    let unmarshaled = UpdateBar::unmarshal(&marshaled).unwrap();

    assert_eq!(update_bar, unmarshaled);
}

#[test]
fn test_update_bar_to_ie() {
    let bar_id = BarId::new(3);
    let update_bar = UpdateBar::new(bar_id, None);

    let ie = update_bar.to_ie();

    assert_eq!(ie.ie_type, IeType::UpdateBar);
    assert!(!ie.payload.is_empty());
}

#[test]
fn test_update_bar_unmarshal_missing_bar_id() {
    // Create empty payload (no BAR ID IE)
    let empty_payload = vec![];

    let result = UpdateBar::unmarshal(&empty_payload);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, PfcpError::MissingMandatoryIe { .. }));
}

#[test]
fn test_update_bar_multiple_values() {
    // Test with various BAR ID values
    for bar_id_value in [1, 10, 100, 255].iter() {
        let bar_id = BarId::new(*bar_id_value);
        let update_bar = UpdateBar::new(bar_id.clone(), None);

        let marshaled = update_bar.marshal();
        let unmarshaled = UpdateBar::unmarshal(&marshaled).unwrap();

        assert_eq!(unmarshaled.bar_id.id, *bar_id_value);
    }
}

#[test]
fn test_update_bar_equality() {
    let bar_id1 = BarId::new(1);
    let bar_id2 = BarId::new(1);

    let update_bar1 = UpdateBar::new(bar_id1, None);
    let update_bar2 = UpdateBar::new(bar_id2, None);

    assert_eq!(update_bar1, update_bar2);
}

#[test]
fn test_update_bar_clone() {
    let bar_id = BarId::new(20);
    let packet_count = SuggestedBufferingPacketsCount::new(75);
    let update_bar = UpdateBar::new(bar_id, Some(packet_count));

    let cloned = update_bar.clone();

    assert_eq!(update_bar, cloned);
}

#[test]
fn test_update_bar_debug() {
    let bar_id = BarId::new(5);
    let update_bar = UpdateBar::new(bar_id, None);

    let debug_string = format!("{:?}", update_bar);

    assert!(debug_string.contains("UpdateBar"));
}
