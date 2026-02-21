use std::fmt::Display;
use std::process;
use std::thread::sleep;
use std::time::Duration;
use chrono::{Local, SecondsFormat};
pub enum LogType {
    Error,
    Info,
}

pub fn log<T:Display >(value: T, lt: LogType) {
    let now = Local::now();
    match lt {
        LogType::Error=> {
            println!("{}: {}", now.to_rfc3339_opts(SecondsFormat::Secs, false), value);
            sleep(Duration::from_millis(5000));
            process::exit(-1)
        },
        LogType::Info => println!("{}: {}", now.to_rfc3339_opts(SecondsFormat::Secs, false), value),
    }
}