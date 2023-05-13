use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(u8)]
#[serde(try_from = "u8", into = "u8")]
pub enum DayOfWeek {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
}

impl From<DayOfWeek> for u8 {
    fn from(day: DayOfWeek) -> Self {
        day as u8
    }
}

#[derive(Debug, Error)]
pub enum DeserializationError {
    #[error("invalid day of week")]
    WrongNumber,
}

impl TryFrom<u8> for DayOfWeek {
    type Error = DeserializationError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DayOfWeek::Sunday),
            1 => Ok(DayOfWeek::Monday),
            2 => Ok(DayOfWeek::Tuesday),
            3 => Ok(DayOfWeek::Wednesday),
            4 => Ok(DayOfWeek::Thursday),
            5 => Ok(DayOfWeek::Friday),
            6 => Ok(DayOfWeek::Saturday),
            _ => Err(DeserializationError::WrongNumber),
        }
    }
}
