use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct ArgsCLI {
    /// Dimacs instance name
    #[structopt(long, short = "i", default_value = "NONE")]
    pub instance: String,
    /// Format (dimacs or predicate)
    #[structopt(long, short = "f", default_value = "predicate")]
    pub format: String,
    /// Decision heuristic
    #[structopt(long, short = "d", default_value = "ran")]
    pub decision_heuristic: String,
    /// Find all assignments (predicate format only)
    #[structopt(long, short = "a", parse(try_from_str), default_value = "false")]
    pub all: bool,
}

pub struct Args {
    pub instance: String,
    pub format: String,
    pub decision_heuristic: String,
    pub all: bool
}

pub fn handle_args() -> Args {
    let args = ArgsCLI::from_args();
    Args {
        instance: args.instance,
        format: args.format,
        decision_heuristic: args.decision_heuristic,
        all: args.all
    }
}