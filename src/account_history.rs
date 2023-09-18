#![allow(dead_code)]

use crate::date_format;

#[allow(unused_imports)]
use date_format::*;

use crate::data_filter::DataFilter;
use csv::{ReaderBuilder, Trim};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, PgPool};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use time::Date;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountHistory {
    #[serde(rename = "Run Date", with = "date_format")]
    run_date: Option<Date>,
    #[serde(rename = "Account")]
    account: String,
    #[serde(rename = "Action")]
    action: String,
    #[serde(rename = "Symbol")]
    symbol: String,
    #[serde(rename = "Security Description")]
    security_description: String,
    #[serde(rename = "Security Type")]
    security_type: String,
    #[serde(rename = "Exchange Quantity")]
    exchange_quantity: Option<Decimal>,
    #[serde(rename = "Exchange Currency")]
    exchange_currency: String,
    #[serde(rename = "Quantity")]
    quantity: Option<Decimal>,
    #[serde(rename = "Currency")]
    currency: String,
    #[serde(rename = "Price")]
    price: Option<Decimal>,
    #[serde(rename = "Exchange Rate")]
    exchange_rate: String,
    #[serde(rename = "Commission")]
    commission: Option<Decimal>,
    #[serde(rename = "Fees")]
    fees: Option<Decimal>,
    #[serde(rename = "Accrued Interest")]
    accrued_interest: Option<Decimal>,
    #[serde(rename = "Amount")]
    amount: Option<Decimal>,
    #[serde(rename = "Settlement Date", with = "date_format")]
    settlement_date: Option<Date>,
    #[serde(skip)]
    source_file_id: i32,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct AccountHistorySourceFile {
    id: i32,
    filename: String,
    start_date: Date,
    end_date: Date,
}

pub fn read_account_history_records(filename: &str) -> anyhow::Result<Vec<AccountHistory>> {
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

    let mut records: Vec<AccountHistory> = vec![];
    for result in reader.deserialize() {
        let record: AccountHistory = result?;
        records.push(record);
    }
    Ok(records)
}
pub async fn load_account_history(pool: &PgPool, filename: &str) -> anyhow::Result<u32> {
    let records = read_account_history_records(&filename)?;
    let (min_date, _max_date) = get_records_date_range(&records);

    let last_source_file = get_last_account_history_source_file(pool).await;

    if let Ok(last_source_file) = last_source_file {
        if min_date < last_source_file.end_date {
            let filename = Path::new(filename).file_name().unwrap().to_str().unwrap().to_string();
            panic!("\nError: account history source file date ranges over lap.\nNew file: '{}' overlaps with: '{}'\nRecords need to be newer than: {}\n",
            filename, last_source_file.filename, last_source_file.end_date);
        }
    }

    let source_file_id = add_account_history_source_file(pool, &filename, &records).await.unwrap();

    insert_account_history_records(pool, &records, source_file_id).await
}

async fn insert_account_history_records(pool: &PgPool, records: &Vec<AccountHistory>, source_file_id: i32) -> anyhow::Result<u32> {

    let cmd = r#"
        INSERT INTO Account_History (
            run_date,
            account,
            action,
            symbol,
            security_description,
            security_type,
            exchange_quantity,
            exchange_currency,
            quantity,
            currency,
            price,
            exchange_rate,
            commission,
            fees,
            accrued_interest,
            amount,
            settlement_date,
            source_file_id
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
        "#;

    let mut count = 0;
    for record in records {
        sqlx::query(cmd)
            .bind(&record.run_date)
            .bind(&record.account)
            .bind(&record.action)
            .bind(&record.symbol)
            .bind(&record.security_description)
            .bind(&record.security_type)
            .bind(&record.exchange_quantity)
            .bind(&record.exchange_currency)
            .bind(&record.quantity)
            .bind(&record.currency)
            .bind(&record.price)
            .bind(&record.exchange_rate)
            .bind(&record.commission)
            .bind(&record.fees)
            .bind(&record.accrued_interest)
            .bind(&record.amount)
            .bind(&record.settlement_date)
            .bind(source_file_id)
            .execute(pool)
            .await?;
        count += 1;
    }

    Ok(count)
}

fn get_records_date_range(records: &Vec<AccountHistory>) -> (Date, Date) {
    let mut min_date = Date::MAX;
    let mut max_date = Date::MIN;
    for record in records {
        if let Some(run_date) = record.run_date {
            if run_date < min_date {
                min_date = run_date;
            }
            if run_date > max_date {
                max_date = run_date;
            }
        }
    }
    (min_date, max_date)
}

async fn get_last_account_history_source_file(pool: &PgPool) -> Result<AccountHistorySourceFile, Error> {

    let result = sqlx::query_as::<_, AccountHistorySourceFile>(
        r#"
        select id, filename, start_date, end_date
        from account_history_source_file
        where end_date = (select max(end_date) from account_history_source_file)
    "#
    ).fetch_one(pool).await;

    result
}

async fn add_account_history_source_file(pool: &PgPool, filename: &str, records: &Vec<AccountHistory>) ->anyhow::Result<i32> {
    let name = Path::new(filename).file_name().unwrap().to_str().unwrap().to_string();
    let (min_date, max_date) = get_records_date_range(records);

    let rec = sqlx::query!(
        r#"
        INSERT INTO Account_History_Source_File (
            filename,
            start_date,
            end_date
        ) VALUES ($1, $2, $3)
        RETURNING id;
        "#,
        name,
        min_date,
        max_date,
    )
        .fetch_one(pool)
        .await?;

   Ok(rec.id)
}
