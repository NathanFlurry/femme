//! Basic log printing.
//!
//! Basic log printing; useful when building command line applications that shouldn't have any
//! extra formatting, but still want to use `log` and the log level filtering.

use console::style;
use log::{kv, Level, Log, Metadata, Record};

/// Basic log printing.
#[derive(Debug)]
pub struct Logger {
    filter: env_logger::filter::Filter,
}

impl Logger {
    /// Create a new instance.
    pub fn new(filter: env_logger::filter::Filter) -> Self {
        Self { filter }
    }

    /// Start logging.
    pub fn start(self) -> Result<(), log::SetLoggerError> {
        let filter = self.filter.filter();
        let res = log::set_boxed_logger(Box::new(self));
        if res.is_ok() {
            log::set_max_level(filter);
        }
        res
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        self.filter.enabled(metadata)
    }

    fn log(&self, record: &Record<'_>) {
        if self.filter.matches(record) {
            let args = record.args();
            let msg = match record.level() {
                Level::Error => format!("{}", style(args).red()),
                Level::Warn => format!("{}", style(args).yellow()),
                _ => args.to_string(),
            };
            println!("{}{}", msg, KeyValues::fmt(&record));
        }
    }

    fn flush(&self) {}
}

struct KeyValues {
    output: Option<String>,
}

impl<'kvs> kv::Visitor<'kvs> for KeyValues {
    fn visit_pair(&mut self, key: kv::Key<'kvs>, val: kv::Value<'kvs>) -> Result<(), kv::Error> {
        if let None = self.output {
            self.output = Some(String::new());
        }
        let string = self.output.as_mut().unwrap();
        string.push_str(&format!(" ,{}={}", key, val));
        Ok(())
    }
}

impl KeyValues {
    fn fmt(record: &Record) -> String {
        let mut visitor = Self { output: None };
        record.key_values().visit(&mut visitor).unwrap();
        match visitor.output {
            Some(output) => output,
            None => String::new(),
        }
    }
}
