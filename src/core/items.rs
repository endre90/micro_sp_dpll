pub struct SolverResult {
    pub sat: bool,
    pub ass: Vec<(String, bool)>,
    pub time: std::time::Duration,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
pub enum Value {
    TRUE,
    FALSE,
    UNNASIGNED
}

#[derive(PartialEq, Clone, Debug, Eq, PartialOrd, Ord)]
pub struct Atom {
    pub name: String,
    pub value: Value
}

impl Atom {
    pub fn new(name: &str, value: Value) -> Atom {
        Atom {
            name: name.to_owned(),
            value : value
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Ord, PartialOrd)]
pub enum Predicate {
    NOT(Box<Predicate>),
    AND(Vec<Predicate>),
    OR(Vec<Predicate>),
    ATOM(Atom)
}