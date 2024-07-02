mod common;

#[test]
fn correct_fringe_solve_lak104d() {
    let context = common::full_lak104d_context("fringe");
    for problem in context.problems() {
        let result = context.solve(*problem);

        assert!(
            result.is_some(),
            "Problem {} path not found",
            problem.number
        );

        assert!(
            result.unwrap() <= (problem.length.unwrap() + 0.001),
            "Problem {} failed:\n  Expected: {}\n  Actual:   {}",
            problem.number,
            problem.length.unwrap(),
            result.unwrap(),
        );
    }
}

#[test]
fn correct_astar_solve_lak104d() {
    let context = common::full_lak104d_context("a-star");
    for problem in context.problems() {
        let result = context.solve(*problem);

        assert!(
            result.is_some(),
            "Problem {} path not found",
            problem.number
        );

        assert!(
            result.unwrap() <= (problem.length.unwrap() + 0.001),
            "Problem {} failed:\n  Expected: {}\n  Actual:   {}",
            problem.number,
            problem.length.unwrap(),
            result.unwrap(),
        );
    }
}
