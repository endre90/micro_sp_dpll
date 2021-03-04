use std::collections::HashMap;

pub fn jeroslow_wang(formula: &Vec<Vec<(String, bool)>>) -> (String, bool) {
    fn count(formula: &Vec<Vec<(String, bool)>>) -> HashMap<(String, bool), u32> {
        let mut counter = HashMap::new();
        for clause in formula {
            for literal in clause {
                if !counter.contains_key(literal) {
                    counter.insert(
                        literal.to_owned(),
                        (2 as i32).pow(-(clause.len() as i32) as u32) as u32,
                    );
                } else {
                    *counter.get_mut(literal).unwrap() +=
                        (2 as i32).pow(-(clause.len() as i32) as u32) as u32
                }
            }
        }
        counter
    }

    let counter = count(formula);
    counter
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0
        .to_owned()
}
