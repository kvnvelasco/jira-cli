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

pub fn pick_from_list<T: Display>(
  prompt: &str,
  items: &Vec<T>,
) -> Result<usize, Box<std::error::Error>> {
  for (index, item) in items.iter().enumerate() {
    println!("{})\t{}", index, item)
  }

  let data: usize = loop {
    if let Ok(num) = read_line(&prompt).parse::<usize>() {
      if num < items.len() {
        break num;
      }
    };
  };

  Ok(data)
}
