use rusqlite::types::{FromSql, FromSqlError, ValueRef};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Format {
    Cd,
    Lp,
    Usb,
    Tape,
}

impl FromSql for Format {
    fn column_result(value: ValueRef<'_>) -> Result<Self, FromSqlError> {
        value
            .as_str()
            .and_then(|s| Format::from_str(s).ok_or(FromSqlError::InvalidType))
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Format {
    pub fn as_str(&self) -> &'static str {
        match self {
            Format::Cd => "cd",
            Format::Lp => "lp",
            Format::Usb => "usb",
            Format::Tape => "tape",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "cd" => Some(Format::Cd),
            "lp" => Some(Format::Lp),
            "usb" => Some(Format::Usb),
            "tape" => Some(Format::Tape),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Album {
    pub id: Option<i64>,
    pub artist: String,
    pub album: String,
    pub genre: String,
    pub release_date: String,
    pub format: Format,
    pub source_url: String,
    pub country: String,
    pub artwork_url: String,
}
