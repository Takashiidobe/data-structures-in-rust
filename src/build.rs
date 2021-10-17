use man::prelude::*;
use std::fs;
use std::fs::File;
use std::io::{Error, Write};

fn min_stack() -> Result<(), Error> {
    let path = "man/min-stack.1";
    let mut output = File::create(path)?;

    let msg = Manual::new("MinStack")
        .about("A Min Stack in Rust.")
        .author(Author::new("Takashi I").email("mail@takashiidobe.com"))
        .render();

    write!(output, "{}", msg)
}

fn queue() -> Result<(), Error> {
    let path = "man/queue-with-stack.1";
    let mut output = File::create(path)?;

    let msg = Manual::new("Queue")
        .about("A Queue in Rust.")
        .author(Author::new("Takashi I").email("mail@takashiidobe.com"))
        .render();

    write!(output, "{}", msg)
}

fn create_dir() -> std::io::Result<()> {
    fs::create_dir("man/")?;
    Ok(())
}

fn main() {
    create_dir();
    min_stack();
    queue();
}
