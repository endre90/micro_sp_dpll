use micro_sp_dpll::*;
use std::time::Duration;

fn main() -> () {

    let x = atom!("x".to_string());
    let y = atom!("y".to_string());
    let z = atom!("z".to_string());
    let w = atom!("w".to_string());

    let test_predicate = or!(
        and!(x.clone(), not!(y)),
        or!(z, and!(x, not!(w)))
    );

    let ha = handle_args();
    let inst = ha.instance.as_str();
    let dh = ha.decision_heuristic.as_str();
    let format = ha.format.as_str();
    let all = ha.all;
    let mut result = SolverResult {sat: false, ass: vec!(), time: Duration::from_millis(0)};
    let mut table = Table {table: vec!(), time: Duration::from_millis(0)};
    match format {
        "dimacs" => result = solve_dimacs(inst, dh),
        "predicate" => match all {
            false => result = solve_predicate(&test_predicate, dh),
            true => table = solve_all(&test_predicate, dh)
        },
        _ => panic!("unknown format"),
    };
    match all {
        true => {
            println!("TABLE");
            for t in table.table {
                println!("{:?}", t);
            }
            println!("TOTAL TIME: {:?}", table.time);
        },
        false => {
            println!("SAT: {:?}", result.sat);
            println!("ASSIGNMENT: {:?}", result.ass);
            println!("TIME: {:?}", result.time);
        }
    }
}