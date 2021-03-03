use std::collections::HashMap;

// When the time comes to make a decision, choose the literal
// that has the most appearance in the formula and and assign the 
// value of its current polarity to it.
pub fn most_often(formula: &Vec<Vec<(u32, bool)>>) -> (u32, bool) {
    fn count(formula: &Vec<Vec<(u32, bool)>>) -> HashMap<(u32, bool), u32> {
        let mut counter = HashMap::new();
        for clause in formula {
            for literal in clause {
                if !counter.contains_key(literal) {
                    counter.insert(*literal, 1 as u32);
                } else {
                    *counter.get_mut(literal).unwrap() += 1
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
