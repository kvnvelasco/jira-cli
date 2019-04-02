use std::error::Error;
use std::fs::{self, create_dir_all, read_dir, write, ReadDir, read_to_string};
pub use std::path::{Path, PathBuf};
use std::result::Result as NativeResult;

type Result<T> = NativeResult<T, Box<Error>>;

pub struct Workspace {
  path: PathBuf,
}

impl Workspace {
  pub fn new(path: &Path) -> Self {
    // create the directory;
    Workspace {
      path: PathBuf::from(path),
    }
  }

  pub fn create_directories(&self) -> Result<()> {
    create_dir_all(&self.path)?;
    Ok(())
  }

  pub fn child_workspace<T>(&self, path: &T) -> Workspace
  where
    T: std::convert::AsRef<std::path::Path>,
  {
    let mut new_buffer = PathBuf::from(&self.path);
    new_buffer.push(path);
    Workspace::new(&new_buffer)
  }

  pub fn get_path(&self, path: &Path) -> PathBuf {
    let mut pathbuf = self.path.to_owned();
    pathbuf.push(path);
    pathbuf
  }

  pub fn write_file(&self, path: &Path, content: &str) -> Result<PathBuf> {
    let pathbuf = self.get_path(path);
    write(&pathbuf, content)?;
    Ok(pathbuf)
  }

  pub fn read_file(&self, path: &Path) -> Result<String> {
    let buffer = self.get_path(path);
    Ok(read_to_string(&buffer)?)
  }

  pub fn get_files_for_dir(&self, path: &Path) -> Option<ReadDir> {
    let dir = self.get_path(&path);
    if dir.is_file() {
      None
    } else {
      match read_dir(&dir) {
        Ok(iterator) => Some(iterator),
        Err(_) => None,
      }
    }
  }

  pub fn create_file(&self, path: &Path) {
    let buffer = self.get_path(&path);
    let _file_name = buffer.file_name().expect("File path is not valid");
    match fs::metadata(&buffer) {
      Ok(_) => {},
      Err(_) => {
        fs::File::create(&buffer).expect("Unable to create file");
        // println!(
        //   "{} {}",
        //   Colour::Blue.paint("Created File"),
        //   path
        //     .canonicalize()
        //     .expect("Unable to decode Path")
        //     .to_str()
        //     .expect("Path is invalid unicode")
        // );
        ()
      }
    };
  }
}
