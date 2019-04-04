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
}

#[cfg(test)]
mod tests {
    extern crate tempfile;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn it_should_create_log() {
    }
}
