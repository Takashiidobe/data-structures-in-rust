use man::prelude::*;
use std::fs::File;
use std::io::{Error, Write};

fn min_stack() -> Result<(), Error> {
    let path = "min-stack.1";
    let mut output = File::create(path)?;

    let msg = Manual::new("MinStack")
        .about("A Min Stack in Rust.")
        .author(Author::new("Takashi I").email("mail@takashiidobe.com"))
        .render();

    write!(output, "{}", msg)
}

fn main() {
    min_stack();
}
