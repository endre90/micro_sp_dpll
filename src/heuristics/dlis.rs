use std::collections::HashMap;

// When the time comes to make a decision, choose the literal
// that satisfies the largest number of currently unsatisfied
// clauses. For a given variable x:
//     C(x,p) = number of clauses in which x appears positively
//     C(x,n) = number of clauses in which x appears negatively
// find a variable a such that C(a,p) is max, a variable b such that C(b,n) is max
// if C(a,p) > C(b,n), 
//     assign a to TRUE and return, else assign b to FALSE and return
pub fn dynamic_largest_individual_sum(formula: &Vec<Vec<(u32, bool)>>) -> (u32, bool) {
    let mut pos_counter = HashMap::new();
    let mut neg_counter = HashMap::new();
    for clause in formula {
        for literal in clause {
            if literal.1 {
                if !pos_counter.contains_key(literal) {
                    pos_counter.insert(*literal, 1 as u32);
                } else {
                    *pos_counter.get_mut(literal).unwrap() += 1
                }
            } else {
                if !neg_counter.contains_key(literal) {
                    neg_counter.insert(*literal, 1 as u32);
                } else {
                    *neg_counter.get_mut(literal).unwrap() += 1
                }
            }
        }
    }

    let max_pos = pos_counter
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap_or((&(0 as u32, false), &0));
    
    let max_neg = neg_counter
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap_or((&(0 as u32, false), &0));

    if max_pos.1 > max_neg.1 {
        *max_pos.0
    } else {
        *max_neg.0
    }
}