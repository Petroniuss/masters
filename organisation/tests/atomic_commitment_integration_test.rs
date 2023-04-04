use organisation::core::peer;

/// Integration test verifying atomic commitment:
/// - spawn two peers in each peerset,
/// - create peerset_1,
/// - create peerset_2,
/// - subscribe for cross_peerset_changes,
/// - propose a cross_peerset_change,
/// - approve changes in both peersets,
/// - this could simply be an integration test now that I think about it
#[tokio::test]
async fn atomic_commitment() {
    // run first peer.
}
