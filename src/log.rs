#![allow(dead_code)]
use chrono::{DateTime, Utc};

pub const DEBUG: u8 = 0; // to set verbosity levels for log messages
pub const NIL: &'static str = "\x1b[0m"; // reset/remove colour

pub const RED: &'static str = "\x1b[0;31m"; // red
pub const ORG: &'static str = "\x1b[0;33m"; // orange
pub const VIO: &'static str = "\x1b[0;35m"; // violet

pub const ERR: &'static str = "\x1b[1;31m"; // error
pub const INF: &'static str = "\x1b[1;32m"; // info
pub const WRN: &'static str = "\x1b[1;33m"; // HLT/warn
pub const LOG: &'static str = "\x1b[1;34m"; // log
pub const MAG: &'static str = "\x1b[1;35m"; // magenta
pub const LGA: &'static str = "\x1b[1;36m"; // Log Aqua
pub const LGT: &'static str = "\x1b[1;36m"; // log time
pub const CYN: &'static str = "\x1b[1;36m"; // cyan
pub const LME: &'static str = "\x1b[1;38m"; // lime

//pub fn log(msg: &str) -> () {
pub fn log<T: std::fmt::Display>(msg: T) -> () {
    let dt: DateTime<Utc> = Utc::now();
    #[rustfmt::skip]
    println!("{}[{}{}{}]{} {}{}",
        MAG,CYN, dt.format("%Y-%m-%d_%H:%M:%S_%Z").to_string(),MAG,LOG,msg,NIL);
}

pub fn warn<T: std::fmt::Display>(msg: T) -> () {
    eprintln!("{}[{}w{}]{} {}{}", INF, LGA, INF, WRN, msg, NIL);
}

pub fn info<T: std::fmt::Display>(msg: T) -> () {
    eprintln!("{}[{}i{}]{} {}{}", WRN, INF, WRN, CYN, msg, NIL);
}
pub fn info_n<T: std::fmt::Display>(msg: T) -> () {
    eprint!("{}[{}i{}]{} {}{}", WRN, INF, WRN, CYN, msg, NIL);
}

pub fn err<T: std::fmt::Display>(msg: T) -> () {
    eprintln!("{}[{}e{}]{} {}{}", RED, ERR, RED, LOG, msg, NIL);
}

pub fn pass<T: std::fmt::Display>(msg: T) -> () {
    println!("{}[{}pass{}]{} {}{}", WRN, INF, WRN, LOG, msg, NIL);
}

pub fn fail<T: std::fmt::Display>(msg: T) -> () {
    eprintln!("{}[{}fail{}]{} {}{}", RED, ERR, RED, CYN, msg, NIL);
}

pub fn usage<T: std::fmt::Display>(msg: T) -> () {
    eprintln!("{}usage{}:{} {}{}", INF, LGA, WRN, msg, NIL);
}

/// Error Print Line
pub fn epl<T: std::fmt::Display>(err: T, msg: T) -> () {
    // error_println!
    eprintln!(
        "{}[{}err{}]{} {}: {}{}{}",
        RED, ERR, RED, NIL, msg, WRN, err, NIL
    );
}
