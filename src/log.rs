extern crate append_log;

use std::path::{Path, PathBuf};

use append_log::Log;

use crate::{Options, Error};

pub struct MultiLog {
    path: PathBuf,
    options: Options,

    // NOTE: last element of the list is the log we write to
    logs: Vec<Log>,

    log_count: usize,
}

impl MultiLog {
    pub fn open_with_default(path: &Path) -> Result<Self, Error> {
        Self::open(path, Options::default())
    }

    pub fn open(path: &Path, options: Options) -> Result<Self, Error> {
        Ok(MultiLog { path: path.to_path_buf(), options, logs: vec![], log_count: 0 })
    }

    fn log_path(path: &Path, index: usize) -> PathBuf {
        if index == 0 {
            return path.to_path_buf();
        }

        let base = path.file_name()
            .map(|val| val.to_str().expect("base to convert to String"))
            .unwrap_or("default");
        path.with_file_name(format!("{}.{}", base, index))
    }
}

#[cfg(test)]
mod tests {
    extern crate tempfile;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn it_should_construct_proper_log_path() {
        assert_eq!(MultiLog::log_path(Path::new("/tmp/my-db"), 0).to_str().unwrap(),
            "/tmp/my-db");
        assert_eq!(MultiLog::log_path(Path::new("/tmp/my-db"), 42).to_str().unwrap(),
            "/tmp/my-db.42");
    }
}
