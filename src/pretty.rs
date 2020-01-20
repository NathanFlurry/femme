//! Pretty print logs.

use console::style;
use log::{kv, Level, Log, Metadata, Record};

#[derive(Debug)]
pub struct Logger {
    filter: env_logger::filter::Filter,
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
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.filter.enabled(metadata)
    }

    fn log(&self, record: &Record<'_>) {
        if self.filter.matches(record) {
            pretty_print(record)
        }
    }

    fn flush(&self) {}
}

fn pretty_print(record: &Record<'_>) {
    println!(
        "{}{}{}",
        format_message(&record),
        format_line(&record),
        format_kv_pairs(&record),
    );
}

fn format_kv_pairs(record: &Record) -> String {
    struct Visitor {
        string: String,
    }

    impl<'kvs> kv::Visitor<'kvs> for Visitor {
        fn visit_pair(
            &mut self,
            key: kv::Key<'kvs>,
            val: kv::Value<'kvs>,
        ) -> Result<(), kv::Error> {
            let string = &format!("   › {}: {}\n", style(key).magenta(), val);
            self.string.push_str(string);
            Ok(())
        }
    }

    let mut visitor = Visitor {
        string: String::new(),
    };
    record.key_values().visit(&mut visitor).unwrap();
    visitor.string
}

fn format_line(record: &Record<'_>) -> String {
    match (record.file(), record.line()) {
        (Some(file), Some(line)) => format!("   {}:{}\n", file, line),
        _ => String::new(),
    }
}

fn format_message(record: &Record<'_>) -> String {
    use Level::*;
    let symbol = match record.level() {
        Trace => format!("{}", "◯"),
        Debug => format!("{}", "◎"),
        Info => format!("{}", "●"),
        Warn => format!("{}", "⌿"),
        Error => format!("{}", "✖"),
    };

    let msg = format!("{}  {}\n", symbol, style(record.args()).underlined());
    match record.level() {
        Trace | Debug | Info => format!("{}", style(msg).green()),
        Warn => format!("{}", style(msg).yellow()),
        Error => format!("{}", style(msg).red()),
    }
}
