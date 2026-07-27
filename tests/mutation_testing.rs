use rust_testing::mutation_testing::{
    MutationDecision, MutationKind, MutationOutcome, MutationSignal,
};
#[test]
fn consumer_can_identify_surviving_mutant() {
    let d = MutationDecision::new(
        "rechaza cupón vencido",
        MutationKind::Condition,
        MutationOutcome::Survived,
    )
    .expect("not empty");
    assert_eq!(d.signal(), MutationSignal::Investigate);
}
