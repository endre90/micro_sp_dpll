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
    ATOM(String),
}

#[macro_export]
macro_rules! atom {
    ($a:expr) => {
        Predicate::ATOM($a)
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
