use crate::random;
use super::items::*;
use std::time::Instant;

pub fn solve_predicate(formula: &Predicate, heuristic: &str) -> SolverResult {
    let ts = crate::tseitin(&formula);
    let cnf = crate::predicate_cnf_to_dpll_cnf(&ts);
    let res = dpll(&cnf, heuristic);
    clean_result(&res)
}

pub fn solve_dimacs(formula: &str, heuristic: &str) -> SolverResult {
    let dimacs = crate::dimacs_cnf_parser(formula);
    let res = dpll(&dimacs, heuristic);
    clean_result(&res)
}

pub fn clean_result(res: &SolverResult) -> SolverResult {
    SolverResult {
        sat: res.sat,
        ass: res.ass.iter().filter(|x| !x.0.starts_with("$")).map(|x| x.to_owned()).collect(),
        time: res.time
    }
}

pub fn dpll(formula: &Vec<Vec<(String, bool)>>, heuristic: &str) -> SolverResult {
    let assignments = vec![];
    let now = Instant::now();
    fn recursive(
        formula: &Vec<Vec<(String, bool)>>,

        heuristic: &str,
        assignments: &Vec<(String, bool)>,
        now: std::time::Instant,
    ) -> SolverResult {
        let (new_formula_up, new_assignments_up) = unit_propagate(formula, assignments);
        let (new_formula, new_assignments) =
            pure_literal_assign(&new_formula_up, &new_assignments_up);
        if new_formula.len() == 0 {
            let time_to_solve = now.elapsed();
            return SolverResult {
                sat: true,
                ass: new_assignments.to_owned(),
                time: time_to_solve,
            };
        };
        if new_formula.contains(&vec![]) {
            let time_to_solve = now.elapsed();
            return SolverResult {
                sat: false,
                ass: vec![],
                time: time_to_solve,
            };
        };
        let new_literal = choose_literal(&new_formula, &assignments, heuristic);

        let mut updated_formula = vec![];
        updated_formula.extend(new_formula.clone());
        updated_formula.extend(vec![vec![new_literal.clone()]]);
        
        let mut updated_assignments = vec![];
        updated_assignments.extend(new_assignments.clone());        

        let sol = recursive(&updated_formula, heuristic, &updated_assignments, now);
        if sol.sat {
            sol
        } else {
            let mut updated_formula = vec![];
            updated_formula.extend(new_formula.clone());
            updated_formula.extend(vec![vec![(new_literal.0.clone(), !new_literal.1)]]);

            let mut updated_assignments = vec![];
            updated_assignments.extend(new_assignments.clone());
            recursive(&updated_formula, heuristic, &updated_assignments, now)
        }
    }
    recursive(formula, heuristic, &assignments, now)
}

// The procedure is based on unit clauses, i.e. clauses that are composed
// of a single literal. Because each clause needs to be satisfied, we know
// that this literal must be true. If a set of clauses contains the unit
// clause l, the other clauses are simplified by the application of the
// two following rules:
//     1. every clause containing l is removed (the clause is satisfied if l is);
//     2. in every clause that contains neg l, this literal is deleted from the clause
//        (neg l can not contribute to it being satisfied).
// The application of these two rules lead to a new set of clauses that
// is equisatisfiable with the original. Unit propagation can sometimed even decide
// the problem (try test 5). If the returned formula is empty, the formula is SAT
// for the given assignments. If the formula contains an empty clause, the formula
// is UNSAT.
pub fn unit_propagate(
    formula: &Vec<Vec<(String, bool)>>,
    assignments: &Vec<(String, bool)>,
) -> (Vec<Vec<(String, bool)>>, Vec<(String, bool)>) {
    let final_partial: Vec<(String, bool)> = assignments.to_owned();

    fn recursive(
        formula: &Vec<Vec<(String, bool)>>,
        final_partial: Vec<(String, bool)>,
    ) -> (Vec<Vec<(String, bool)>>, Vec<(String, bool)>) {
        let partial: Vec<(String, bool)> = formula
            .iter()
            .filter(|x| x.len() == 1)
            .map(|x| x[0].clone())
            .collect();

        let mut _new_formula = formula.to_owned();
        if partial.len() == 0 {
            return (formula.to_owned(), final_partial);
        } else {
            for unit in &partial {
                if unit.1 {
                    _new_formula = _new_formula
                        .iter()
                        .filter(|x| !x.contains(&unit))
                        .map(|x| x.to_owned())
                        .collect();
                    _new_formula = _new_formula
                        .to_owned()
                        .iter()
                        .map(|x| {
                            let mut mut_x = x.clone();
                            mut_x.retain(|x| *x != (unit.0.clone(), false));
                            mut_x
                        })
                        .collect();
                } else {
                    _new_formula = _new_formula
                        .iter()
                        .filter(|x| !x.contains(&unit))
                        .map(|x| x.to_owned())
                        .collect();
                    _new_formula = _new_formula
                        .to_owned()
                        .iter()
                        .map(|x| {
                            let mut mut_x = x.clone();
                            mut_x.retain(|x| *x != (unit.0.clone(), true));
                            mut_x
                        })
                        .collect();
                }
            }
        }
        let mut new_assignments = vec![];
        new_assignments.extend(partial);
        new_assignments.extend(final_partial.to_owned());
        recursive(&_new_formula, new_assignments)
    }
    recursive(formula, final_partial)
}

// If a propositional variable occurs with only one polarity in the formula,
// it is called pure. Pure literals can always be assigned in a way that makes
// all clauses containing them true. Thus, these clauses do not constrain the
// search anymore and can be deleted.
pub fn pure_literal_assign(
    formula: &Vec<Vec<(String, bool)>>,
    assignments: &Vec<(String, bool)>,
) -> (Vec<Vec<(String, bool)>>, Vec<(String, bool)>) {
    let final_unipolar_literals: Vec<(String, bool)> = assignments.to_owned();

    fn recursive(
        formula: &Vec<Vec<(String, bool)>>,
        final_unipolar_literals: Vec<(String, bool)>,
    ) -> (Vec<Vec<(String, bool)>>, Vec<(String, bool)>) {
        let mut literals: Vec<(String, bool)> = vec![];
        for clause in formula {
            for literal in clause {
                if !literals.contains(&literal) {
                    literals.push(literal.to_owned())
                }
            }
        }
        let unipolar_literals: Vec<(String, bool)> = literals
            .iter()
            .filter(|x| {
                (literals.contains(&(x.0.to_owned(), false)) && !literals.contains(&(x.0.to_owned(), true)))
                    || (literals.contains(&(x.0.to_owned(), true)) && !literals.contains(&(x.0.to_owned(), false)))
            })
            .map(|x| x.to_owned())
            .collect();
        let mut _new_formula = formula.to_owned();
        if unipolar_literals.len() == 0 {
            return (formula.to_owned(), final_unipolar_literals);
        } else {
            for pure in &unipolar_literals {
                _new_formula = _new_formula
                    .iter()
                    .filter(|x| !x.contains(&pure))
                    .map(|x| x.to_owned())
                    .collect();
            }
        }
        let mut assignments = vec![];
        assignments.extend(unipolar_literals);
        assignments.extend(final_unipolar_literals.to_owned());
        recursive(&_new_formula, assignments)
    }
    recursive(formula, final_unipolar_literals)
}

// When unit propagation and pure literal elimination can do no more,
// we have to choose one literal from the formula and assign a polarity.
// Then we conjunct that literal to the current cnf formula so that the
// unit propagation procedure and pure literal elimination prodedure can
// again try to decide the prodlem.
pub fn choose_literal(
    formula: &Vec<Vec<(String, bool)>>,
    assignments: &Vec<(String, bool)>,
    heuristic: &str,
) -> (String, bool) {
    match heuristic {
        "ran" => random(formula, assignments),
        // "mo" => most_often(formula),
        // "jw" => jeroslow_wang(formula),
        // "dlis" => dynamic_largest_individual_sum(formula),
        _ => panic!("unknown heuristic"),
    }
}