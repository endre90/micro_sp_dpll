use std::collections::HashMap;

pub fn most_often(formula: &Vec<Vec<(String, bool)>>) -> (String, bool) {
    fn count(formula: &Vec<Vec<(String, bool)>>) -> HashMap<(String, bool), u32> {
        let mut counter = HashMap::new();
        for clause in formula {
            for literal in clause {
                if !counter.contains_key(literal) {
                    counter.insert(literal.to_owned(), 1 as u32);
                } else {
                    *counter.get_mut(literal).unwrap() += 1
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
