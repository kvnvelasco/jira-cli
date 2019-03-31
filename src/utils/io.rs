extern crate ansi_term;
use ansi_term::Colour;

use std::io::prelude::*;
use std::io::{stdin, stdout};

pub fn read_line(prompt: &'static str) -> String {
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
