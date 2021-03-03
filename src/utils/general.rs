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
}

pub struct Args {
    pub instance: String,
    pub format: String,
    pub decision_heuristic: String,
}

pub fn handle_args() -> Args {
    let args = ArgsCLI::from_args();
    Args {
        instance: args.instance,
        format: args.format,
        decision_heuristic: args.decision_heuristic,
    }
}

pub trait IterOps<T, I>: IntoIterator<Item = T>
where
    I: IntoIterator<Item = T>,
    T: PartialEq,
{
    fn intersect(self, other: I) -> Vec<T>;
    fn difference(self, other: I) -> Vec<T>;
}

impl<T, I> IterOps<T, I> for I
where
    I: IntoIterator<Item = T>,
    T: PartialEq,
{
    fn intersect(self, other: I) -> Vec<T> {
        let mut common = Vec::new();
        let mut v_other: Vec<_> = other.into_iter().collect();

        for e1 in self.into_iter() {
            if let Some(pos) = v_other.iter().position(|e2| e1 == *e2) {
                common.push(e1);
                v_other.remove(pos);
            }
        }

        common
    }

    fn difference(self, other: I) -> Vec<T> {
        let mut diff = Vec::new();
        let mut v_other: Vec<_> = other.into_iter().collect();

        for e1 in self.into_iter() {
            if let Some(pos) = v_other.iter().position(|e2| e1 == *e2) {
                v_other.remove(pos);
            } else {
                diff.push(e1);
            }
        }

        diff.append(&mut v_other);
        diff
    }
}

pub struct Stack<T> {
    limited: bool,
    maxsize: usize,
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            limited: false,
            maxsize: 0,
            items: Vec::new(),
        }
    }

    pub fn with_capacity(maxsize: usize) -> Self {
        Self {
            limited: true,
            maxsize,
            items: Vec::with_capacity(maxsize),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn push(&mut self, item: T) -> bool {
        if self.limited {
            if self.items.len() == self.maxsize {
                return false;
            }
            self.items.push(item);
            return true;
        } else {
            self.items.push(item);
            return true;
        }
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }
}
