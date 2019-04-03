extern crate append_log;

use append_log::Log;

pub struct MultiLog {
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
