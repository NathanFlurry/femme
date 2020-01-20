//! Print logs as ndjson.

use log::{kv, Log, Metadata, Record};
use serde_json::Value;
use std::collections::HashMap;
use std::time;

#[derive(Debug)]
pub struct Logger {
    filter: env_logger::filter::Filter,
}

#[derive(serde_derive::Serialize)]
struct Msg {
    level: u8,
    time: u128,
    msg: String,
    #[serde(flatten)]
    key_values: Option<HashMap<String, Value>>,
}

impl Logger {
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
            print_ndjson(record)
        }
    }

    fn flush(&self) {}
}

// TODO: implement key_values mapping
fn print_ndjson(record: &Record<'_>) {
    let msg = Msg {
        level: get_level(record.level()),
        key_values: format_kv_pairs(&record),
        time: time::UNIX_EPOCH.elapsed().unwrap().as_millis(),
        msg: record.args().to_string(),
    };
    println!("{}", serde_json::to_string(&msg).unwrap())
}

fn get_level(level: log::Level) -> u8 {
    use log::Level::*;
    match level {
        Trace => 10,
        Debug => 20,
        Info => 30,
        Warn => 40,
        Error => 50,
    }
}

fn format_kv_pairs(record: &Record) -> Option<HashMap<String, Value>> {
    struct Visitor {
        key_values: Option<HashMap<String, Value>>,
    }

    impl<'kvs> kv::Visitor<'kvs> for Visitor {
        fn visit_pair(
            &mut self,
            key: kv::Key<'kvs>,
            val: kv::Value<'kvs>,
        ) -> Result<(), kv::Error> {
            if let None = self.key_values {
                self.key_values = Some(HashMap::new());
            }
            let kv = self.key_values.as_mut().unwrap();
            kv.insert(key.to_string(), val.to_string().into());
            Ok(())
        }
    }

    let mut visitor = Visitor { key_values: None };
    record.key_values().visit(&mut visitor).unwrap();
    visitor.key_values
}
