extern crate ansi_term;
use ansi_term::Colour;
use std::fmt::Display;
use std::io::prelude::*;
use std::io::{stdin, stdout};

pub fn read_line(prompt: &str) -> String {
    // send prompt
    let mut data = String::new();
    print!("{}: ", Colour::Green.paint(prompt));
    stdout()
        .flush()
        .expect("i/o invariant: Unable to flush stdout");

    stdin()
        .read_line(&mut data)
        .expect("i/o invariant: Unable to read line");

    data.trim_end().to_owned()
}

pub trait Pickable {
    fn get_key(&self) -> String;
}

pub fn pick_from_list<T: Display + Pickable>(
    prompt: &str,
    items: &[T],
) -> Result<usize, Box<std::error::Error>> {
    for item in items {
        println!(
            "{:<10} {}",
            Colour::Blue.underline().paint(&item.get_key()),
            item
        )
    }

    let data: usize = loop {
        let string = read_line(&prompt);
        if let Some(index) = items.iter().position(|f| f.get_key() == string) {
            break index;
        }
    };

    Ok(data)
}
