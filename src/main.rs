use lib::*;
use std::time::Duration;

fn main() -> () {

    // Write your test predicate here:

    let x = Atom::new("x", Value::UNNASIGNED);
    let y = Atom::new("y", Value::UNNASIGNED);
    let z = Atom::new("z", Value::UNNASIGNED);
    let w = Atom::new("w", Value::UNNASIGNED);

    let test_predicate = Predicate::OR(
        vec!(
            Predicate::AND(
                vec!(
                    Predicate::ATOM(x.clone()),
                    Predicate::NOT(
                        Box::new(
                            Predicate::ATOM(y)
                        )
                    )
                )
            ),
            Predicate::OR(
                vec!(
                    Predicate::ATOM(z),
                    Predicate::AND(
                        vec!(
                            Predicate::ATOM(x),
                            Predicate::NOT(
                                Box::new(
                                    Predicate::ATOM(w)
                                )
                            )
                        )
                    )
                )
            )
        )
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
