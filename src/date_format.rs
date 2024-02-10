use chrono::NaiveDate;
use log::warn;
use serde::{self, Deserialize, Deserializer, Serializer};

pub fn serialize<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        None => serializer.serialize_str(""),
        Some(date) => {
            let format = "%Y-%m-%d"; // %Y-%m-%d"
            let s = date.format(&format).to_string();
            serializer.serialize_str(&s)
        }
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    // Hack to fix 2 digit year and 1 digit month in the settlement column
    let year: String;
    let month: String;
    let mut parts: Vec<&str> = s.split('/').collect();
    if parts.len() == 3 {
        if parts[0].len() == 1 {
            month = format!("0{}", parts[0]).as_str().parse().unwrap();
            parts[0] = month.as_str();
        }
        if parts[2].len() == 2 {
            year = format!("20{}", parts[2]).as_str().parse().unwrap();
            parts[2] = year.as_str();
        }
    }
    let s = parts.join("/");

    let result = NaiveDate::parse_from_str(&s, "%m/%d/%Y").map_err(|e| {
        warn!("Error: {:?}", &e);
        e
    });

    match result {
        Ok(date) => Ok(Some(date)),
        Err(_error) => Ok(None),
    }
}
