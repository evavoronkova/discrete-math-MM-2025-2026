use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DirectedOrUndirected {
    Directed,
    Undirected,
}

impl fmt::Display for DirectedOrUndirected {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Directed => "Directed",
            Self::Undirected => "Undirected",
        };
        write!(f, "{}", s)
    }
}
