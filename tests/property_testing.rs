use rust_testing::property_testing::{
    GeneratorDomain, PropertyDecision, PropertyGap, PropertyKind, PropertySignal,
};
#[test]
fn consumer_can_record_a_shrinkable_edge_risk() {
    let d = PropertyDecision::new(
        "ordenar conserva longitud",
        PropertyKind::Invariant,
        GeneratorDomain::EdgeFocused,
    )
    .expect("not empty")
    .with_gap(PropertyGap::MissingEdge);
    assert_eq!(d.signal(), PropertySignal::Focused);
}
