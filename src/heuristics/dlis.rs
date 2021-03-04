use std::collections::HashMap;

pub fn dlis(formula: &Vec<Vec<(String, bool)>>) -> (String, bool) {
    let mut pos_counter = HashMap::new();
    let mut neg_counter = HashMap::new();
    for clause in formula {
        for literal in clause {
            if literal.1 {
                if !pos_counter.contains_key(literal) {
                    pos_counter.insert(literal.to_owned(), 1 as u32);
                } else {
                    *pos_counter.get_mut(literal).unwrap() += 1
                }
            } else {
                if !neg_counter.contains_key(literal) {
                    neg_counter.insert(literal.to_owned(), 1 as u32);
                } else {
                    *neg_counter.get_mut(literal).unwrap() += 1
                }
            }
        }
    }

    let max_pos = pos_counter.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

    let max_neg = neg_counter.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

    if max_pos.1 > max_neg.1 {
        max_pos.0.to_owned()
    } else {
        max_neg.0.to_owned()
    }
}
