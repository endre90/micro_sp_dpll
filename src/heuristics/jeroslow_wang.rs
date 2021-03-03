use std::collections::HashMap;

// This strategy gives higher priority to literals that appear
// frequently in short clauses.
pub fn jeroslow_wang(formula: &Vec<Vec<(u32, bool)>>) -> (u32, bool) {
    fn count(formula: &Vec<Vec<(u32, bool)>>) -> HashMap<(u32, bool), u32> {
        let mut counter = HashMap::new();
        for clause in formula {
            for literal in clause {
                if !counter.contains_key(literal) {
                    counter.insert(
                        *literal,
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
    *counter
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap_or((&(0 as u32, false), &0))
        .0
}
