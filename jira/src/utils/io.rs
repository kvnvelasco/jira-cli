extern crate ansi_term;
use ansi_term::Colour;
use simple_error::SimpleError;
use skim::{Skim, SkimOptions, SkimOptionsBuilder};
use std::fmt::Display;
use std::io::prelude::*;
use std::io::{stdin, stdout, Cursor};
lazy_static! {
    static ref SKIM_OPTIONS: SkimOptions<'static> = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .ansi(true)
        .build()
        .unwrap();
}

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
    fn display_key(&self) -> String;
}

pub fn pick_from_list<T: Display + Pickable>(items: &[T]) -> Result<usize, Box<std::error::Error>> {
    let data = {
        let mut temp = String::new();
        for item in items {
            temp.push_str(&format!("{} {}\n", item.get_key(), item))
        }
        temp
    };

    let selected_items = Skim::run_with(&SKIM_OPTIONS, Some(Box::new(Cursor::new(data))))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    Ok(selected_items
        .first()
        .ok_or(SimpleError::new("Unable to select an item."))?
        .get_index())
}

pub fn confirm(prompt: &str) -> bool {
    read_line(&format!("{} (y/N)?", prompt)) == "y"
}
