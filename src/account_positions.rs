#![allow(dead_code)]

use crate::data_filter::DataFilter;
use csv::{ReaderBuilder, Trim};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sha256::digest;
use sqlx::PgPool;
use std::fs::File;
use std::io::{BufRead, BufReader};
use chrono::NaiveDate;
use log::warn;

#[allow(unused_imports)]
use crate::decimal_formats::*;

use crate::date_format;

#[allow(unused_imports)]
use date_format::*;

// Fidelity Account Position - Overview and Dividends views
// These are the struct for reading in the raw Fidelity data.
// Can be joined on {account_number, symbol, quantity)
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountPositionsOverview {
    #[serde(rename = "Account Number")]
    account_number: String,
    #[serde(rename = "Account Name")]
    account_name: String,
    #[serde(rename = "Symbol", deserialize_with = "deserialize_symbol")]
    symbol: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Quantity")]
    quantity: Option<Decimal>,
    #[serde(rename = "Last Price", deserialize_with = "deserialize_dollar")]
    last_price: Option<Decimal>,
    #[serde(rename = "Last Price Change", deserialize_with = "deserialize_dollar")]
    last_price_change: Option<Decimal>,
    #[serde(rename = "Current Value", deserialize_with = "deserialize_dollar")]
    current_value: Option<Decimal>,
    #[serde(rename = "Today's Gain/Loss Dollar", deserialize_with = "deserialize_dollar")]
    today_gain_loss_dollar: Option<Decimal>,
    #[serde(rename = "Today's Gain/Loss Percent", deserialize_with = "deserialize_percentage")]
    today_gain_loss_percent: Option<Decimal>,
    #[serde(rename = "Total Gain/Loss Dollar", deserialize_with = "deserialize_dollar")]
    total_gain_loss_dollar: Option<Decimal>,
    #[serde(rename = "Total Gain/Loss Percent", deserialize_with = "deserialize_percentage")]
    total_gain_loss_percent: Option<Decimal>,
    #[serde(rename = "Percent Of Account", deserialize_with = "deserialize_dollar")]
    percent_of_account: Option<Decimal>,
    #[serde(rename = "Cost Basis Total", deserialize_with = "deserialize_dollar")]
    cost_basis_total: Option<Decimal>,
    #[serde(rename = "Average Cost Basis", deserialize_with = "deserialize_dollar")]
    average_cost_basis: Option<Decimal>,
    #[serde(rename = "Type")]
    r#type: String,
    #[serde(skip_deserializing, with = "date_format")]
    as_of_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountPositionsDividends {
    #[serde(rename = "Account Number")]
    account_number: String,
    #[serde(rename = "Account Name")]
    account_name: String,
    #[serde(rename = "Symbol", deserialize_with = "deserialize_symbol")]
    symbol: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Quantity")]
    quantity: Option<Decimal>,
    #[serde(rename = "Last Price", deserialize_with = "deserialize_dollar")]
    last_price: Option<Decimal>,
    #[serde(rename = "Last Price Change", deserialize_with = "deserialize_dollar")]
    last_price_change: Option<Decimal>,
    #[serde(rename = "Current Value", deserialize_with = "deserialize_dollar")]
    current_value: Option<Decimal>,
    #[serde(rename = "Percent Of Account", deserialize_with = "deserialize_percentage")]
    percent_of_account: Option<Decimal>,
    #[serde(rename = "Ex-Date", with = "date_format")]
    ex_date: Option<NaiveDate>,
    #[serde(rename = "Amount Per Share", deserialize_with = "deserialize_dollar")]
    amount_per_share: Option<Decimal>,
    #[serde(rename = "Pay Date", with = "date_format")]
    pay_date: Option<NaiveDate>,
    #[serde(rename = "Yield", deserialize_with = "deserialize_percentage")]
    r#yield: Option<Decimal>,
    #[serde(rename = "Est. Annual Income", deserialize_with = "deserialize_dollar")]
    est_annual_income: Option<Decimal>,
    #[serde(rename = "Type")]
    r#type: String,
    #[serde(skip_deserializing, with = "date_format")]
    as_of_date: Option<NaiveDate>,
}

pub fn read_account_position_overview(filename: String) -> anyhow::Result<Vec<AccountPositionsOverview>> {
    let file = File::open(filename).expect("Failed to open file");
    let buf_reader = BufReader::new(file);

    // Only for fidelity csv with junk lines
    let data_filter = DataFilter::new(buf_reader);

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .trim(Trim::All)
        .flexible(true)
        .from_reader(data_filter);

    let mut records: Vec<AccountPositionsOverview> = vec![];
    for result in reader.deserialize() {
        let record: AccountPositionsOverview = result?;
        records.push(record);
    }
    Ok(records)
}

pub fn read_account_position_dividends(filename: String) -> anyhow::Result<Vec<AccountPositionsDividends>> {
    let file = File::open(filename).expect("Failed to open file");
    let buf_reader = BufReader::new(file);

    // Only for fidelity csv with junk lines
    let data_filter = DataFilter::new(buf_reader);

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .trim(Trim::All)
        .flexible(true)
        .from_reader(data_filter);

    let mut records: Vec<AccountPositionsDividends> = vec![];
    for result in reader.deserialize() {
        let record: AccountPositionsDividends = result?;
        records.push(record);
    }
    Ok(records)
}

pub async fn load_account_positions_overview(pool: &PgPool, filename: String, date_str: &str) -> anyhow::Result<u32> {
    let records = read_account_position_overview(filename)?;

    // let format = format_description!("[month]-[day]-[year]");
    // let date = Date::parse(date_str, &format).unwrap();

    let date = NaiveDate::parse_from_str(date_str, "%m-%d-%Y").map_err(|e| {
        warn!("Error: {:?}", &e);
        e
    })?;

    let cmd = r#"
        INSERT INTO Account_Positions_Overview (
            account_number,
            account_name,
            symbol,
            description,
            quantity,
            last_price,
            last_price_change,
            current_value,
            today_gain_loss_dollar,
            today_gain_loss_percent,
            total_gain_loss_dollar,
            total_gain_loss_percent,
            percent_of_account,
            cost_basis_total,
            average_cost_basis,
            type,
            as_of_date,
            Hash
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
        "#;

    let mut count = 0;
    for record in records {
        let encoded: Vec<u8> = bincode::serialize(&record).unwrap();
        let hash = digest(encoded);

        sqlx::query(cmd)
            .bind(record.account_number)
            .bind(record.account_name)
            .bind(record.symbol)
            .bind(record.description)
            .bind(record.quantity)
            .bind(record.last_price)
            .bind(record.last_price_change)
            .bind(record.current_value)
            .bind(record.today_gain_loss_dollar)
            .bind(record.today_gain_loss_percent)
            .bind(record.total_gain_loss_dollar)
            .bind(record.total_gain_loss_percent)
            .bind(record.percent_of_account)
            .bind(record.cost_basis_total)
            .bind(record.average_cost_basis)
            .bind(record.r#type)
            .bind(date)
            .bind(hash)
            .execute(pool)
            .await?;
        count += 1;
    }
    Ok(count)
}

pub async fn load_account_positions_dividends(pool: &PgPool, filename: String, date_str: &str) -> anyhow::Result<u32> {
    let records = read_account_position_dividends(filename)?;

    // let format = format_description!("[month]-[day]-[year]");
    // let date = Date::parse(date_str, &format).unwrap();

    let date = NaiveDate::parse_from_str(date_str, "%m-%d-%Y").map_err(|e| {
        warn!("Error: {:?}", &e);
        e
    })?;

    let cmd = r#"
        INSERT INTO Account_Positions_Dividends (
            account_number,
            account_name,
            symbol,
            description,
            quantity,
            last_price,
            last_price_change,
            current_value,
            percent_of_account,
            ex_date,
            amount_per_share,
            pay_date,
            yield,
            est_annual_income,
            type,
            as_of_date,
            Hash
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
        "#;

    let mut count = 0;
    for record in records {
        let encoded: Vec<u8> = bincode::serialize(&record).unwrap();
        let hash = digest(encoded);

        sqlx::query(cmd)
            .bind(record.account_number)
            .bind(record.account_name)
            .bind(record.symbol)
            .bind(record.description)
            .bind(record.quantity)
            .bind(record.last_price)
            .bind(record.last_price_change)
            .bind(record.current_value)
            .bind(record.percent_of_account)
            .bind(record.ex_date)
            .bind(record.amount_per_share)
            .bind(record.pay_date)
            .bind(record.r#yield)
            .bind(record.est_annual_income)
            .bind(record.r#type)
            .bind(date)
            .bind(hash)
            .execute(pool)
            .await?;
        count += 1;
    }
    Ok(count)
}

pub fn extract_date(filename: &str) -> Result<NaiveDate, String> {
    let file = File::open(filename).expect("Failed to open file");
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines().flatten() {
        if line.starts_with("\"Date downloaded") {
            // let format = format_description!("[month]-[day]-[year]");
            let date_str = &line[17..27];
            let result = NaiveDate::parse_from_str(date_str, "%m-%d-%Y").map_err(|e| {
                warn!("Error: {:?}", &e);
                e
            });
            return match result {
                Ok(date) => Ok(date),
                Err(e) => Err(e.to_string()),
            };
        }
    }
    Err("Date not found".to_string())
}

#[cfg(test)]
mod tests {
    use crate::account_positions::extract_date;

    #[test]
    fn test_extract_date() {
        let date = extract_date("/Users/john/Portfolio_Data/Portfolio_Positions_Dividend_Sep-08-2023.csv");
        println!("Date: {:?}", date);
    }
}
