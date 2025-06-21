use serde::{Deserialize, Serialize};

/// Logging severities roughly modelled after the [RFC 5424](https://datatracker.ietf.org/doc/html/rfc5424)
/// syslog levels.  The numeric values (10 … 60) match the typical `TRACE ≤ DEBUG ≤ INFO`…
/// ordering so they can be compared directly (`level >= self.level`).
///
/// When compiled with `serde`, the enum serialises to/from its canonical upper‑case string
/// representation (e.g. `"INFO"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum Level {
    Trace = 10,
    Debug = 20,
    Info = 30,
    Warn = 40,
    Error = 50,
    Fatal = 60,
}

impl Level {
    pub fn as_str(&self) -> &'static str {
        match self {
            Level::Trace => "TRACE",
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
            Level::Fatal => "FATAL",
        }
    }

    pub fn from_str(s: &str) -> Option<Level> {
        match s.to_uppercase().as_str() {
            "TRACE" => Some(Level::Trace),
            "DEBUG" => Some(Level::Debug),
            "INFO" => Some(Level::Info),
            "WARN" => Some(Level::Warn),
            "ERROR" => Some(Level::Error),
            "FATAL" => Some(Level::Fatal),
            _ => None,
        }
    }

    pub fn value(&self) -> u8 {
        *self as u8
    }
}