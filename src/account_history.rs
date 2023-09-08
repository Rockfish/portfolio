#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::date_format;

#[allow(unused_imports)]
use date_format::*;

use crate::data_filter::DataFilter;
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
    let data_filter = DataFilter::new(buf_reader);

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .trim(Trim::All)
        .flexible(true)
        .from_reader(data_filter);

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
