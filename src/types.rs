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

    #[allow(dead_code)]
    pub const fn half(a: Solver) -> [Part<'a>; 2] {
        [
            Part::new("Example 1", "example.txt", a),
            Part::new("Part 1", "input.txt", a),
        ]
    }

    pub const fn full(a: Solver, b: Solver) -> [Part<'a>; 4] {
        [
            Part::new("Example 1", "example.txt", a),
            Part::new("Part 1", "input.txt", a),
            Part::new("Example 2", "example.txt", b),
            Part::new("Part 2", "input.txt", b),
        ]
    }
}
