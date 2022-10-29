#![cfg(incsolver)]

use rustsat::{
    instances::CNF,
    lit,
    solvers::{new_default_inc_solver, SolverResult},
    types::Lit,
};

#[test]
fn cnf_implications() {
    let a0 = lit![0];
    let a1 = lit![1];
    let b0 = lit![3];
    let b1 = lit![4];

    let mut cnf = CNF::new();
    cnf.add_lit_impl_lit(a0, b0);
    let mut solver = new_default_inc_solver();
    solver.add_cnf(cnf).unwrap();
    let ret = solver.solve_assumps(vec![a0, b0]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, !b0]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![!a0, b0]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, !b0]).unwrap();
    assert_eq!(ret, SolverResult::SAT);

    let mut cnf = CNF::new();
    cnf.add_lit_impl_clause(a0, vec![b0, b1]);
    let mut solver = new_default_inc_solver();
    solver.add_cnf(cnf).unwrap();
    let ret = solver.solve_assumps(vec![a0, b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, !b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, !b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![!a0, b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, !b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, !b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);

    let mut cnf = CNF::new();
    cnf.add_lit_impl_cube(a0, vec![b0, b1]);
    let mut solver = new_default_inc_solver();
    solver.add_cnf(cnf).unwrap();
    let ret = solver.solve_assumps(vec![a0, b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![a0, !b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![a0, !b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![!a0, b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, !b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, !b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);

    let mut cnf = CNF::new();
    cnf.add_cube_impl_lit(vec![a0, a1], b0);
    let mut solver = new_default_inc_solver();
    solver.add_cnf(cnf).unwrap();
    let ret = solver.solve_assumps(vec![a0, a1, b0]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, a1, !b0]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![a0, !a1, b0]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, !a1, !b0]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, a1, b0]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, a1, !b0]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, !a1, b0]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, !a1, !b0]).unwrap();
    assert_eq!(ret, SolverResult::SAT);

    let mut cnf = CNF::new();
    cnf.add_clause_impl_lit(vec![a0, a1], b0);
    let mut solver = new_default_inc_solver();
    solver.add_cnf(cnf).unwrap();
    let ret = solver.solve_assumps(vec![a0, a1, b0]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, a1, !b0]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![a0, !a1, b0]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, !a1, !b0]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![!a0, a1, b0]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, a1, !b0]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![!a0, !a1, b0]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, !a1, !b0]).unwrap();
    assert_eq!(ret, SolverResult::SAT);

    let mut cnf = CNF::new();
    cnf.add_cube_impl_clause(vec![a0, a1], vec![b0, b1]);
    let mut solver = new_default_inc_solver();
    solver.add_cnf(cnf).unwrap();
    let ret = solver.solve_assumps(vec![a0, a1, b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, a1, b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, a1, !b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, a1, !b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![a0, !a1, b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, !a1, b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, !a1, !b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, !a1, !b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, a1, b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, a1, b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, a1, !b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, a1, !b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, !a1, b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, !a1, b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, !a1, !b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, !a1, !b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);

    let mut cnf = CNF::new();
    cnf.add_clause_impl_clause(vec![a0, a1], vec![b0, b1]);
    let mut solver = new_default_inc_solver();
    solver.add_cnf(cnf).unwrap();
    let ret = solver.solve_assumps(vec![a0, a1, b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, a1, b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, a1, !b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, a1, !b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![a0, !a1, b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, !a1, b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, !a1, !b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, !a1, !b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![!a0, a1, b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, a1, b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, a1, !b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, a1, !b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![!a0, !a1, b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, !a1, b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, !a1, !b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, !a1, !b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);

    let mut cnf = CNF::new();
    cnf.add_clause_impl_cube(vec![a0, a1], vec![b0, b1]);
    let mut solver = new_default_inc_solver();
    solver.add_cnf(cnf).unwrap();
    let ret = solver.solve_assumps(vec![a0, a1, b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, a1, b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![a0, a1, !b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![a0, a1, !b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![a0, !a1, b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, !a1, b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![a0, !a1, !b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![a0, !a1, !b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![!a0, a1, b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, a1, b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![!a0, a1, !b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![!a0, a1, !b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![!a0, !a1, b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, !a1, b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, !a1, !b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, !a1, !b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);

    let mut cnf = CNF::new();
    cnf.add_cube_impl_cube(vec![a0, a1], vec![b0, b1]);
    let mut solver = new_default_inc_solver();
    solver.add_cnf(cnf).unwrap();
    let ret = solver.solve_assumps(vec![a0, a1, b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, a1, b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![a0, a1, !b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![a0, a1, !b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::UNSAT);
    let ret = solver.solve_assumps(vec![a0, !a1, b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, !a1, b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, !a1, !b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![a0, !a1, !b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, a1, b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, a1, b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, a1, !b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, a1, !b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, !a1, b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, !a1, b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, !a1, !b0, b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
    let ret = solver.solve_assumps(vec![!a0, !a1, !b0, !b1]).unwrap();
    assert_eq!(ret, SolverResult::SAT);
}
