type Solver = fn(&str) -> String;

pub struct Part<'a> {
    pub name: &'a str,
    pub file: &'a str,
    pub solver: Solver,
}

impl<'a> Part<'a> {
    pub const fn new(name: &'a str, file: &'a str, solver: Solver) -> Self {
        Self {
            name: name,
            file: file,
            solver: solver,
        }
    }
}

#[macro_export]
macro_rules! common_parts {
    ( $( $solver:expr ),* ) => {
        [ $( Part::new("Example", "example.txt", $solver), Part::new("Input", "input.txt", $solver) ),* ]
    };
}
