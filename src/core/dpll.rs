use super::items::*;
use crate::heuristics::*;
use std::time::{Duration, Instant};

pub fn solve_all(formula: &Predicate, heuristic: &str) -> Table {
    let mut table_vec = vec![];
    let mut total_time = Duration::from_micros(0);
    let mut mut_formula = formula.clone();
    loop {
        let result = solve_predicate(&mut_formula, &heuristic);
        if result.sat {
            table_vec.push(result.ass.clone());
            total_time = total_time + result.time;
            mut_formula = Predicate::AND(vec![
                mut_formula,
                Predicate::NOT(Box::new(Predicate::AND(
                    result
                        .ass
                        .iter()
                        .map(|x| match x.1 {
                            true => Predicate::ATOM(x.0.clone()),
                            false => Predicate::NOT(Box::new(Predicate::ATOM(x.0.clone()))),
                        })
                        .collect(),
                ))),
            ])
        } else {
            break Table {
                table: table_vec,
                time: total_time,
            };
        }
    }
}

pub fn solve_predicate(formula: &Predicate, heuristic: &str) -> SolverResult {
    let now = Instant::now();
    let ts = crate::tseitin(&formula);
    let cnf = crate::predicate_cnf_to_dpll_cnf(&ts);
    let res = dpll(&cnf, heuristic);
    let clean_res = clean_result(&res);
    let time_to_solve = now.elapsed();
    SolverResult {
        sat: clean_res.sat,
        ass: clean_res.ass,
        time: time_to_solve,
    }
}

pub fn solve_dimacs(formula: &str, heuristic: &str) -> SolverResult {
    let now = Instant::now();
    let dimacs = crate::dimacs_cnf_parser(formula);
    let res = dpll(&dimacs, heuristic);
    let clean_res = clean_result(&res);
    let time_to_solve = now.elapsed();
    SolverResult {
        sat: clean_res.sat,
        ass: clean_res.ass,
        time: time_to_solve,
    }
}

pub fn clean_result(res: &SolverResult) -> SolverResult {
    SolverResult {
        sat: res.sat,
        ass: {
            let mut result = res
                .ass
                .iter()
                .filter(|x| !x.0.starts_with("$"))
                .map(|x| x.to_owned())
                .collect::<Vec<(String, bool)>>();
            result.sort();
            result.dedup();
            result
        },
        time: res.time,
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
        let new_literal = choose_literal(&new_formula, heuristic);

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
                (literals.contains(&(x.0.to_owned(), false))
                    && !literals.contains(&(x.0.to_owned(), true)))
                    || (literals.contains(&(x.0.to_owned(), true))
                        && !literals.contains(&(x.0.to_owned(), false)))
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

pub fn choose_literal(formula: &Vec<Vec<(String, bool)>>, heuristic: &str) -> (String, bool) {
    match heuristic {
        "ran" => random::random(formula),
        "mo" => most_often::most_often(formula),
        "jw" => jeroslow_wang::jeroslow_wang(formula),
        "dlis" => dlis::dlis(formula),
        _ => panic!("unknown heuristic"),
    }
}
