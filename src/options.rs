extern crate append_log;

pub use append_log::{Options as LogOptions};

#[derive(Debug)]
pub struct Options {
    pub log: LogOptions,
}

impl Default for Options {
    fn default() -> Self {
        Options { log: LogOptions::default() }
    }
}
