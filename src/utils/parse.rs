use std::fs::File;
use std::io::prelude::*;

pub fn dimacs_cnf_parser(name: &str) -> Vec<Vec<(String, bool)>> {
    let mut instance = File::open(&format!("{}", name)).unwrap();
    let mut instance_buffer = String::new();

    instance.read_to_string(&mut instance_buffer).unwrap();
    let mut instance_lines = instance_buffer.lines();

    let mut atoms: Vec<(String, bool)> = vec![];
    let mut clauses = vec![];

    let mut next_instance_line = "SOME";

    while next_instance_line != "NONE" {
        next_instance_line = match instance_lines.next() {
            Some(x) => x,
            None => "NONE",
        };
        if next_instance_line != "NONE" {
            if next_instance_line.starts_with("c ") {
                ()
            } else if next_instance_line.starts_with("p cnf") {
                let stats: Vec<String> = next_instance_line
                    .split(|c| c == ' ')
                    .filter(|x| *x != "p" && *x != "cnf")
                    .map(|x| x.to_owned())
                    .collect();
                match stats.len() == 2 {
                    true => (),
                    false => panic!("wrong dimacs cnf stats"),
                }
            } else {
                match next_instance_line.ends_with(" 0") {
                    true => {
                        let clause = next_instance_line
                            .split(|c| c == ' ')
                            .filter(|x| *x != "0")
                            .map(|y| match y.parse::<i32>().unwrap() >= 0 {
                                true => (y.parse::<i32>().unwrap().abs().to_string(), true),
                                false => (y.parse::<i32>().unwrap().abs().to_string(), false),
                            })
                            .collect::<Vec<(String, bool)>>();
                        for atom in &clause {
                            if !atoms.contains(atom) {
                                atoms.push(atom.to_owned())
                            }
                        }
                        clauses.push(clause);
                    }
                    false => {
                        println!("{}", next_instance_line);
                        panic!("wrong dimacs format")
                    }
                }
            }
        }
    }
    clauses
}
