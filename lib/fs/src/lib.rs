use std::error::Error;
use std::fs::{self, create_dir_all, read_dir, read_to_string, write, ReadDir};
pub use std::path::{Path, PathBuf};
use std::result::Result as NativeResult;

type Result<T> = NativeResult<T, Box<Error>>;

pub struct Workspace {
    path: PathBuf,
}

impl Workspace {
    pub fn new(path: &Path) -> Result<Self> {
        // create the directory;
        let ws = Workspace {
            path: PathBuf::from(path),
        };
        ws.create_directories()?;
        Ok(ws)
    }

    pub fn file_exists(&self, path: &Path) -> bool {
        let buffer = self.get_path(&path);
        if let false = buffer.is_file() { return false };
        match fs::metadata(&buffer) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    fn create_directories(&self) -> Result<()> {
        create_dir_all(&self.path)?;
        Ok(())
    }

    pub fn child_workspace<T>(&self, path: &T) -> Result<Workspace>
    where
        T: std::convert::AsRef<std::path::Path>,
    {
        let mut new_buffer = PathBuf::from(&self.path);
        new_buffer.push(path);
        let ws = Workspace::new(&new_buffer)?;
        ws.create_directories()?;
        Ok(ws)
    }

    pub fn get_path(&self, path: &Path) -> PathBuf {
        let mut pathbuf = self.path.to_owned();
        pathbuf.push(path);
        pathbuf
    }

    pub fn write_file(&self, path: &Path, content: &str) -> Result<PathBuf> {
        let pathbuf = self.get_path(path);
        self.create_file_if_not_exists(&pathbuf);
        write(&pathbuf, content)?;
        Ok(pathbuf)
    }

    pub fn read_file(&self, path: &Path) -> Result<String> {
        self.create_file_if_not_exists(&path);
        let buffer = self.get_path(&path);
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

    fn create_file_if_not_exists(&self, path: &Path) {
        let buffer = self.get_path(&path);
        let _file_name = buffer.file_name().expect("File path is not valid");
        if let false = self.file_exists(&path) {
            fs::File::create(&buffer).expect("Unable to create file");
        };
    }
}
