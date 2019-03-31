extern crate ansi_term;
use ansi_term::Colour;
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_absolute_path(path: PathBuf) -> PathBuf {
  match path.is_relative() {
    true => {
      let current_dir =
        std::env::current_dir().expect("Unable to get current working directory context.");
      let mut local = PathBuf::from(current_dir);
      local.push(path);
      local
    }
    false => path,
  }
}

pub fn construct_paths(base_path: &Path, paths: Vec<&str>, validate: bool) -> PathBuf {
  let mut pathbuf = PathBuf::from(base_path);
  for path in paths {
    pathbuf.push(path);
  }

  match pathbuf.canonicalize() {
    Ok(_) => pathbuf,
    Err(_) => {
      if validate == false {
        return pathbuf
      };
      println!(
        "{} \
         Please check that this is a valid cli context or run jira init",
        Colour::Red.paint("Attempted to load file or directory that does not exist.")
      );
      std::process::exit(2);
    }
  }
}

pub fn check_file_exists(path: &Path) -> bool {
  match fs::metadata(path) {
    Ok(_) => true,
    Err(_) => false,
  }
}

pub fn create_file(path: &Path) {
  let file_name = path.file_name().expect("File path is not valid");
  match fs::metadata(&path) {
    Ok(_) => println!("File {:?} Already exists, skipping", &file_name),
    Err(_) => {
      fs::File::create(&path).expect("Unable to create file");
      println!(
        "{} {}",
        Colour::Blue.paint("Created File"),
        path
          .canonicalize()
          .expect("Unable to decode Path")
          .to_str()
          .expect("Path is invalid unicode")
      );
      ()
    }
  };
}

pub fn create_dir(path: &Path) {
  match fs::create_dir_all(&path) {
    Ok(_) => println!(
      "{} {}",
      Colour::Blue.paint("Created Directory"),
      path
        .canonicalize()
        .expect("Unable to decode Path")
        .to_str()
        .expect("Path is not valid unicode")
    ),
    Err(_) => panic!("Unable to create directory {}. Please make sure you have access"),
  }
}

pub fn write_file(path: &Path, contents: String) {
  fs::write(path, contents).expect("Unable to write to file");
}

pub fn file_to_string(path: &Path) -> String {
  fs::read_to_string(path).expect("Unable to read file")
}
