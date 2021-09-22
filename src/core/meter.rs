use std::collections::HashMap;
use std::fmt::Display;

use anyhow::Result;
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{ser::SerializeTuple, Deserialize, Serialize, Serializer};

#[derive(Debug)]
pub enum Instrument {
    PomodoroDaily,
}

impl Display for Instrument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            &Self::PomodoroDaily => "pomodoro.daily",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub struct Measurement(pub DateTime<Utc>, pub f64);

impl Measurement {
    pub fn new(time: DateTime<Utc>, value: f64) -> Measurement {
        Measurement(time, value)
    }
}

impl Serialize for Measurement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tuple = serializer.serialize_tuple(2)?;
        tuple.serialize_element(&self.0.timestamp_millis())?;
        tuple.serialize_element(&self.1)?;
        tuple.end()
    }
}

fn serialize_display<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Display,
    S: Serializer,
{
    serializer.collect_str(value)
}

// 返却用データなのでserだけで充分
#[derive(Debug, Serialize)]
pub struct Measurements {
    #[serde(serialize_with = "serialize_display")]
    pub instrument: Instrument,
    pub labels: HashMap<String, String>,
    pub data: Vec<Measurement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeRange {
    #[serde(with = "ts_milliseconds")]
    pub start: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub end: DateTime<Utc>,
}

pub trait MeterQuery {
    fn query_pomodoro_daily(&mut self, range: &TimeRange) -> Result<Measurements>;
}

#[cfg(test)]
mod test {
    use super::Measurement;
    use chrono::{TimeZone, Utc};
    use serde_json;

    #[test]
    fn test_sertialize_measurement() {
        let time = Utc.ymd(2015, 3, 14).and_hms(0, 0, 0);
        let measurement = Measurement(time, 1.0);
        let serialized = serde_json::to_string(&measurement).expect("json encode failed");
        assert_eq!(serialized, "[1426291200000,1.0]")
    }
}
