use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn random(
    formula: &Vec<Vec<(String, bool)>>,
    assignments: &Vec<(String, bool)>,
) -> (String, bool) {
    let mut literals = vec![]; //assignments.to_owned();
    for clause in formula.to_owned() {
        for literal in clause {
            if literals.contains(&literal) || assignments.contains(&literal) {
                ()
            } else {
                literals.push(literal)
            }
        }
    }
    let mut rng = thread_rng();
    literals.choose(&mut rng).unwrap().to_owned()
}
