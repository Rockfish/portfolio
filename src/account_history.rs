#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::date_format;

#[allow(unused_imports)]
use date_format::*;

use crate::csv_filter::CsvFilter;
use csv::{ReaderBuilder, Trim};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sha256::digest;
use sqlx::MySqlPool;
use std::fs::File;
use std::io::BufReader;
use time::Date;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account_History {
    #[serde(rename = "Run Date", with = "date_format")]
    Run_Date: Option<Date>,
    #[serde(rename = "Account")]
    Account: String,
    #[serde(rename = "Action")]
    Action: String,
    #[serde(rename = "Symbol")]
    Symbol: String,
    #[serde(rename = "Security Description")]
    Security_Description: String,
    #[serde(rename = "Security Type")]
    Security_Type: String,
    #[serde(rename = "Exchange Quantity")]
    Exchange_Quantity: Option<Decimal>,
    #[serde(rename = "Exchange Currency")]
    Exchange_Currency: String,
    #[serde(rename = "Quantity")]
    Quantity: Option<Decimal>,
    #[serde(rename = "Currency")]
    Currency: String,
    #[serde(rename = "Price")]
    Price: Option<Decimal>,
    #[serde(rename = "Exchange Rate")]
    Exchange_Rate: String,
    #[serde(rename = "Commission")]
    Commission: Option<Decimal>,
    #[serde(rename = "Fees")]
    Fees: Option<Decimal>,
    #[serde(rename = "Accrued Interest")]
    Accrued_Interest: Option<Decimal>,
    #[serde(rename = "Amount")]
    Amount: Option<Decimal>,
    #[serde(rename = "Settlement Date", with = "date_format")]
    Settlement_Date: Option<Date>,
    #[serde(skip)]
    Hash: String,
}

pub fn read_records(filename: String) -> anyhow::Result<Vec<Account_History>> {
    let file = File::open(filename).expect("Failed to open file");
    let buf_reader = BufReader::new(file);
    let csv_reader = CsvFilter::new(buf_reader);

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .trim(Trim::All)
        .from_reader(csv_reader);

    let mut records: Vec<Account_History> = vec![];
    for result in reader.deserialize() {
        let record: Account_History = result?;
        records.push(record);
    }
    Ok(records)
}
pub async fn load_account_history(pool: &MySqlPool, filename: String) -> anyhow::Result<u32> {
    let records = read_records(filename)?;

    let cmd = r#"
        INSERT INTO Accounts_History (
            Run_Date,
            Account,
            Action,
            Symbol,
            Security_Description,
            Security_Type,
            Exchange_Quantity,
            Exchange_Currency,
            Quantity,
            Currency,
            Price,
            Exchange_Rate,
            Commission,
            Fees,
            Accrued_Interest,
            Amount,
            Settlement_Date,
            Hash
        ) VALUES ( ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;

    let mut count = 0;
    for record in records {
        let encoded: Vec<u8> = bincode::serialize(&record).unwrap();
        let hash = digest(encoded);

        sqlx::query(cmd)
            .bind(record.Run_Date)
            .bind(record.Account)
            .bind(record.Action)
            .bind(record.Symbol)
            .bind(record.Security_Description)
            .bind(record.Security_Type)
            .bind(record.Exchange_Quantity)
            .bind(record.Exchange_Currency)
            .bind(record.Quantity)
            .bind(record.Currency)
            .bind(record.Price)
            .bind(record.Exchange_Rate)
            .bind(record.Commission)
            .bind(record.Fees)
            .bind(record.Accrued_Interest)
            .bind(record.Amount)
            .bind(record.Settlement_Date)
            .bind(hash)
            .execute(pool)
            .await?;
        count += 1;
    }

    Ok(count)
}

// mod date_format {
//     use serde::{self, Deserialize, Deserializer, Serializer};
//     use time::macros::format_description;
//     use time::Date;
//
//     pub fn serialize<S>(date: &Option<Date>, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         return match date {
//             None => unreachable!(),
//             Some(date) => {
//                 let format = format_description!("[year]-[month]-[day]");
//                 let s = format!("{:?}", date.format(&format));
//                 serializer.serialize_str(&s)
//             }
//         };
//     }
//
//     pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Date>, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let format = format_description!("[month]/[day]/[year]");
//         let s = String::deserialize(deserializer)?;
//
//         // Hack to fix 2 digit year and 1 digit month in the settlement column
//         let year: String;
//         let month: String;
//         let mut parts: Vec<&str> = s.split('/').collect();
//         if parts.len() == 3 {
//             if parts[0].len() == 1 {
//                 month = format!("0{}", parts[0]).as_str().parse().unwrap();
//                 parts[0] = month.as_str();
//             }
//             if parts[2].len() == 2 {
//                 year = format!("20{}", parts[2]).as_str().parse().unwrap();
//                 parts[2] = year.as_str();
//             }
//         }
//         let s = parts.join("/");
//
//         let result = Date::parse(&s, &format);
//
//         return match result {
//             Ok(date) => Ok(Some(date)),
//             Err(_error) => Ok(None),
//         };
//     }
// }
