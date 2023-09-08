
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]


use std::fs::File;
use std::io::BufReader;
use csv::{ReaderBuilder, Trim};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sha256::digest;
use sqlx::MySqlPool;
use time::Date;
use crate::data_filter::DataFilter;

#[allow(unused_imports)]
use crate::decimal_formats::*;

use crate::date_format;

#[allow(unused_imports)]
use date_format::*;

// Fidelity Account Position - Overview and Dividends views
// These are the struct for reading in the raw Fidelity data.
// Can be join on {account_number, symbol, quantity)
#[derive(Debug, Serialize, Deserialize)]
pub struct Account_Positions_Overview {
    #[serde(rename = "Account Number")]
    Account_Number: String,
    #[serde(rename = "Account Name")]
    Account_Name: String,
    #[serde(rename = "Symbol", deserialize_with = "deserialize_symbol")]
    Symbol: String,
    #[serde(rename = "Description")]
    Description: String,
    #[serde(rename = "Quantity")]
    Quantity: Option<Decimal>,
    #[serde(rename = "Last Price", deserialize_with = "deserialize_dollar")]
    Last_Price: Option<Decimal>,
    #[serde(rename = "Last Price Change", deserialize_with = "deserialize_dollar")]
    Last_Price_Change: Option<Decimal>,
    #[serde(rename = "Current Value", deserialize_with = "deserialize_dollar")]
    Current_Value: Option<Decimal>,
    #[serde(rename = "Today's Gain/Loss Dollar", deserialize_with = "deserialize_dollar")]
    Today_Gain_Loss_Dollar: Option<Decimal>,
    #[serde(rename = "Today's Gain/Loss Percent", deserialize_with = "deserialize_percentage")]
    Today_Gain_Loss_Percent: Option<Decimal>,
    #[serde(rename = "Total Gain/Loss Dollar", deserialize_with = "deserialize_dollar")]
    Total_Gain_Loss_Dollar: Option<Decimal>,
    #[serde(rename = "Total Gain/Loss Percent", deserialize_with = "deserialize_percentage")]
    Total_Gain_Loss_Percent: Option<Decimal>,
    #[serde(rename = "Percent Of Account", deserialize_with = "deserialize_dollar")]
    Percent_Of_Account: Option<Decimal>,
    #[serde(rename = "Cost Basis Total", deserialize_with = "deserialize_dollar")]
    Cost_Basis_Total: Option<Decimal>,
    #[serde(rename = "Average Cost Basis", deserialize_with = "deserialize_dollar")]
    Average_Cost_Basis: Option<Decimal>,
    #[serde(rename = "Type")]
    Type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account_Positions_Dividends {
    #[serde(rename = "Account Number")]
    Account_Number: String,
    #[serde(rename = "Account Name")]
    Account_Name: String,
    #[serde(rename = "Symbol", deserialize_with = "deserialize_symbol")]
    Symbol: String,
    #[serde(rename = "Description")]
    Description: String,
    #[serde(rename = "Quantity")]
    Quantity: Option<Decimal>,
    #[serde(rename = "Last Price", deserialize_with = "deserialize_dollar")]
    Last_Price: Option<Decimal>,
    #[serde(rename = "Last Price Change", deserialize_with = "deserialize_dollar")]
    Last_Price_Change: Option<Decimal>,
    #[serde(rename = "Current Value", deserialize_with = "deserialize_dollar")]
    Current_Value: Option<Decimal>,
    #[serde(rename = "Percent Of Account", deserialize_with = "deserialize_percentage")]
    Percent_Of_Account: Option<Decimal>,
    #[serde(rename = "Ex-Date", with = "date_format")]
    Ex_Date: Option<Date>,
    #[serde(rename = "Amount Per Share", deserialize_with = "deserialize_percentage")]
    Amount_Per_Share: Option<Decimal>,
    #[serde(rename = "Pay Date", with = "date_format")]
    Pay_Date: Option<Date>,
    #[serde(rename = "Yield", deserialize_with = "deserialize_percentage")]
    Yield: Option<Decimal>,
    #[serde(rename = "Est. Annual Income", deserialize_with = "deserialize_dollar")]
    Est_Annual_Income: Option<Decimal>,
    #[serde(rename = "Type")]
    Type: String,
}

pub fn read_account_position_overview(filename: String) -> anyhow::Result<Vec<Account_Positions_Overview>> {
    let file = File::open(filename).expect("Failed to open file");
    let buf_reader = BufReader::new(file);
    let data_filter = DataFilter::new(buf_reader);

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .trim(Trim::All)
        .flexible(true)
        .from_reader(data_filter);

    let mut records: Vec<Account_Positions_Overview> = vec![];
    for result in reader.deserialize() {
        let record: Account_Positions_Overview = result?;
        records.push(record);
    }
    Ok(records)
}

pub fn read_account_position_dividends(filename: String) -> anyhow::Result<Vec<Account_Positions_Dividends>> {
    let file = File::open(filename).expect("Failed to open file");
    let buf_reader = BufReader::new(file);
    let data_filter = DataFilter::new(buf_reader);

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .trim(Trim::All)
        .flexible(true)
        .from_reader(data_filter);

    let mut records: Vec<Account_Positions_Dividends> = vec![];
    for result in reader.deserialize() {
        let record: Account_Positions_Dividends = result?;
        records.push(record);
    }
    Ok(records)
}

pub async fn load_account_positions_overview(pool: &MySqlPool, filename: String) -> anyhow::Result<u32> {
    let records = read_account_position_overview(filename)?;

    let cmd = r#"
        INSERT INTO Account_Positions_Overview (
            Account_Number,
            Account_Name,
            Symbol,
            Description,
            Quantity,
            Last_Price,
            Last_Price_Change,
            Current_Value,
            Today_Gain_Loss_Dollar,
            Today_Gain_Loss_Percent,
            Total_Gain_Loss_Dollar,
            Total_Gain_Loss_Percent,
            Percent_Of_Account,
            Cost_Basis_Total,
            Average_Cost_Basis,
            Type,
            Hash
        ) VALUES ( ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;

    let mut count = 0;
    for record in records {
        let encoded: Vec<u8> = bincode::serialize(&record).unwrap();
        let hash = digest(encoded);

        sqlx::query(cmd)
            .bind(record.Account_Number)
            .bind(record.Account_Name)
            .bind(record.Symbol)
            .bind(record.Description)
            .bind(record.Quantity)
            .bind(record.Last_Price)
            .bind(record.Last_Price_Change)
            .bind(record.Current_Value)
            .bind(record.Today_Gain_Loss_Dollar)
            .bind(record.Today_Gain_Loss_Percent)
            .bind(record.Total_Gain_Loss_Dollar)
            .bind(record.Total_Gain_Loss_Percent)
            .bind(record.Percent_Of_Account)
            .bind(record.Cost_Basis_Total)
            .bind(record.Average_Cost_Basis)
            .bind(record.Type)
            .bind(hash)
            .execute(pool)
            .await?;
        count += 1;
    }
    Ok(count)
}

pub async fn load_account_positions_dividends(pool: &MySqlPool, filename: String) -> anyhow::Result<u32> {
    let records = read_account_position_dividends(filename)?;

    let cmd = r#"
        INSERT INTO Account_Positions_Dividends (
            Account_Number,
            Account_Name,
            Symbol,
            Description,
            Quantity,
            Last_Price,
            Last_Price_Change,
            Current_Value,
            Percent_Of_Account,
            Ex_Date,
            Amount_Per_Share,
            Pay_Date,
            Yield,
            Est_Annual_Income,
            Type,
            Hash
        ) VALUES ( ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;

    let mut count = 0;
    for record in records {
        let encoded: Vec<u8> = bincode::serialize(&record).unwrap();
        let hash = digest(encoded);

        sqlx::query(cmd)
            .bind(record.Account_Number)
            .bind(record.Account_Name)
            .bind(record.Symbol)
            .bind(record.Description)
            .bind(record.Quantity)
            .bind(record.Last_Price)
            .bind(record.Last_Price_Change)
            .bind(record.Current_Value)
            .bind(record.Percent_Of_Account)
            .bind(record.Ex_Date)
            .bind(record.Amount_Per_Share)
            .bind(record.Pay_Date)
            .bind(record.Yield)
            .bind(record.Est_Annual_Income)
            .bind(record.Type)
            .bind(hash)
            .execute(pool)
            .await?;
        count += 1;
    }
    Ok(count)
}
