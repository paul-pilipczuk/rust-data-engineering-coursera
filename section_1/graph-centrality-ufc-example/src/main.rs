use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::Direction;
use std::fmt;


#[derive(Debug)]
struct Fighter{
    name: String,
}

impl Fighter{
    fn new(name: &str) -> Self {
        Fighter {
            name: name.to_string(),
        }
    }
}