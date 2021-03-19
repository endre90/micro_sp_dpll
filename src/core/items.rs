pub struct SolverResult {
    pub sat: bool,
    pub ass: Vec<(String, bool)>,
    pub time: std::time::Duration,
}

pub struct Table {
    pub table: Vec<Vec<(String, bool)>>,
    pub time: std::time::Duration,
}

#[derive(Debug, PartialEq, Clone, Eq, Ord, PartialOrd)]
pub enum Predicate {
    NOT(Box<Predicate>),
    AND(Vec<Predicate>),
    OR(Vec<Predicate>),
    VAR(String),
}

#[macro_export]
macro_rules! var {
    ($a:expr) => {
        Predicate::VAR($a.to_owned())
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
        Predicate::AND($a.to_owned())
    };
    ($( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x.to_owned());
            )*
            Predicate::AND(temp_vec)
        }
    };
}

#[macro_export]
macro_rules! or {
    ($a:expr) => {
        Predicate::OR($a.to_owned())
    };
    ($( $x:expr ),* ) => {
        {
            let mut temp_vec: Vec<Predicate> = Vec::new();
            $(
                temp_vec.push($x.to_owned());
            )*
            Predicate::OR(temp_vec)
        }
    };
}
