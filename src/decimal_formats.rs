use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

// Possible inputs:
//     n/a
//     --
//    $1.00
//   ($1.00)
pub fn deserialize_dollar<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    if s.contains("n/a") || s.contains("--") {
        return Ok(None);
    }

    let negative = if s.contains('(') { Decimal::from(-1) } else { Decimal::from(1) };

    let s = s.replace(['$', '(', ')'], "");

    let number = Decimal::from_str(&s);

    match number {
        Ok(num) => Ok(Some(num * negative)),
        Err(_error) => Ok(None),
    }
}

pub fn deserialize_percentage<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    if s.contains("n/a") || s.contains("--") {
        return Ok(None);
    }

    let s = s.replace('%', "");
    let number = Decimal::from_str(&s);

    match number {
        Ok(num) => Ok(Some(num)),
        Err(_error) => Ok(None),
    }
}

pub fn deserialize_symbol<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let s = s.replace('*', "");
    Ok(s)
}
