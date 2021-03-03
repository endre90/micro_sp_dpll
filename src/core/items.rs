pub struct SolverResult {
    pub sat: bool,
    pub ass: Vec<(String, bool)>,
    pub time: std::time::Duration,
}

pub struct Table {
    pub table: Vec<Vec<(String, bool)>>,
    pub time: std::time::Duration,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
pub enum Value {
    TRUE,
    FALSE,
    UNNASIGNED,
}

#[derive(PartialEq, Clone, Debug, Eq, PartialOrd, Ord)]
pub struct Atom {
    pub name: String,
    pub value: Value,
}

impl Atom {
    pub fn new(name: &str, value: Value) -> Atom {
        Atom {
            name: name.to_owned(),
            value: value,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Ord, PartialOrd)]
pub enum Predicate {
    NOT(Box<Predicate>),
    AND(Vec<Predicate>),
    OR(Vec<Predicate>),
    ATOM(Atom),
}

#[macro_export]
macro_rules! atom {
    ($a:expr) => {
        Predicate::ATOM(Atom::new($a, Value::UNNASIGNED))
    };
}

#[macro_export]
macro_rules! not {
    ($a:expr) => {
        Predicate::NOT(Box::new($a))
    };
}

#[macro_export]
macro_rules! and {
    ($a:expr) => {
        Predicate::AND($a)
    };
    ($( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            Predicate::AND(temp_vec)
        }
    };
}

#[macro_export]
macro_rules! or {
    ($a:expr) => {
        Predicate::OR($a)
    };
    ($( $x:expr ),* ) => {
        {
            let mut temp_vec: Vec<Predicate> = Vec::new();
            $(
                temp_vec.push($x);
            )*
            Predicate::OR(temp_vec)
        }
    };
}
