use soplex_rs::{Model, Status};

fn main() {
    let mut lp = Model::new();
    let col1 = lp.add_col([], 1.0, 0.0, 5.0);
    let _col2 = lp.add_col([], 1.0, 0.0, 10.0);
    let row = lp.add_row([1.0, 1.0], 1.0, 5.0);
    assert_eq!(lp.num_cols(), 2);
    assert_eq!(lp.num_rows(), 1);

    let lp = lp.optimize();
    let result = lp.status();
    assert_eq!(result, Status::Optimal);
    assert!((lp.obj_val() - 5.0).abs() < 1e-6);
    let dual_sol = lp.dual_solution();
    assert_eq!(dual_sol.len(), 1);
    assert!((dual_sol[0] - 1.0).abs() < 1e-6);

    let mut lp = Model::from(lp);
    lp.remove_row(row);
    assert_eq!(lp.num_rows(), 0);
    let lp = lp.optimize();
    let new_result = lp.status();
    assert_eq!(new_result, Status::Optimal);
    assert!((lp.obj_val() - 15.0).abs() < 1e-6);
    let primal_sol = lp.primal_solution();
    assert_eq!(primal_sol.len(), 2);
    assert!((primal_sol[0] - 5.0).abs() < 1e-6);
    assert!((primal_sol[1] - 10.0).abs() < 1e-6);

    let mut lp = Model::from(lp);
    lp.remove_col(col1);
    assert_eq!(lp.num_cols(), 1);
    let lp = lp.optimize();
    let new_result = lp.status();
    assert_eq!(new_result, Status::Optimal);
    assert!((lp.obj_val() - 10.0).abs() < 1e-6);

    assert!(lp.solving_time() >= 0.0);
}