mod common;

#[test]
fn correct_fringe_solve() {
    let mut context = common::full_lak104d_context("fringe");
    let error = context.solve_full();
    assert!(error < 0.000_001);
}

#[test]
fn correct_astar_solve() {
    let mut context = common::full_lak104d_context("a-star");
    let error = context.solve_full();
    assert!(error < 0.000_001);
}
