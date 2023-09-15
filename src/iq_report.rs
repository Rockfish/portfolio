use std::fs::File;
use std::io::BufReader;
// use chrono::{DateTime, NaiveDate, TimeZone};
use csv::{ReaderBuilder, Trim};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::Date;
use time::macros::format_description;
#[allow(unused_imports)]
use crate::decimal_formats::*;

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct IQ_Report {
    #[serde(rename = "STOCK")]
    Stock: String,
    #[serde(rename = "Div Growth")]
    Div_Growth: String,
    #[serde(rename = "Value Rating")]
    Value_Rating: String,
    #[serde(rename = "Price")]
    Price: Option<Decimal>,
    #[serde(rename = "Dividend")]
    Dividend: Option<Decimal>,
    #[serde(rename = "Yield", deserialize_with = "deserialize_percentage")]
    Yield: Option<Decimal>,
    #[serde(rename = "Pts Dn")]
    Points_Down: Option<Decimal>,
    #[serde(rename = "% Down", deserialize_with = "deserialize_percentage")]
    Percent_Down: Option<Decimal>,
    #[serde(rename = "Undervalue LoPr")]
    Undervalue_Lo_Price: Option<Decimal>,
    #[serde(rename = "Undervalue HiYld", deserialize_with = "deserialize_percentage")]
    Undervalue_Hi_Yield: Option<Decimal>,
    #[serde(rename = "Pts Up")]
    Points_Up: Option<Decimal>,
    #[serde(rename = "% Up", deserialize_with = "deserialize_percentage")]
    Percent_Up: Option<Decimal>,
    #[serde(rename = "Overvalue HiPr")]
    Overvalue_Hi_Price: Option<Decimal>,
    #[serde(rename = "Overvalue LoYld", deserialize_with = "deserialize_percentage")]
    Overvalue_Lo_Yield: Option<Decimal>,
    #[serde(rename = "S&P")]
    SP_Rating: String,
    #[serde(rename = "52 wik Lo")]
    Lo_52_Wk: Option<Decimal>,
    #[serde(rename = "52 wk Hi")]
    Hi_52_Wk: Option<Decimal>,
    #[serde(rename = "Bk Val")]
    Book_Value: Option<Decimal>,
    #[serde(rename = "12-Mo Earn")]
    Earnings_12_Mo: Option<Decimal>,
    #[serde(rename = "P/E")]
    Price_to_Earnings: Option<Decimal>,
    #[serde(rename = "Pay out", deserialize_with = "deserialize_percentage")]
    Pay_Out: Option<Decimal>,
    #[serde(rename = "Div in Dgr")]
    Div_In_Dgr: String,
    #[serde(rename = "L/T Debt", deserialize_with = "deserialize_percentage")]
    Long_Term_Debt: Option<Decimal>,
    #[serde(rename = "BC")]
    Bluechip_Criteria: Option<Decimal>,
    #[serde(rename = "Tic")]
    Symbol: String,
    #[serde(rename = "SECTOR")]
    Sector: String,
    #[serde(rename = "INDUSTRY")]
    Industry: String,
    #[serde(rename = "SUB-SECTOR")]
    Sub_Sector: String,
    #[serde(rename = "3 Year Div Growth", deserialize_with = "deserialize_percentage")]
    Div_Growth_3_Year: Option<Decimal>,
    #[serde(rename = "5 year Div Growth", deserialize_with = "deserialize_percentage")]
    Div_Growth_5_Year: Option<Decimal>,
    #[serde(rename = "10 Year Div Growth", deserialize_with = "deserialize_percentage")]
    Div_Growth_10_Year : Option<Decimal>,
    #[serde(skip_deserializing)]
    Report_Date: Option<Date>,
}

pub fn read_iq_report(filename: String) -> anyhow::Result<Vec<IQ_Report>> {
    let file = File::open(filename).expect("Failed to open file");
    let buf_reader = BufReader::new(file);

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .quote(b'"')
        .trim(Trim::All)
        .flexible(true)
        .from_reader(buf_reader);

    let mut records: Vec<IQ_Report> = vec![];
    for result in reader.deserialize() {
        match result {
            Ok(record) => records.push(record),
            Err(e) => {
                println!("{:#?}", e);
                return Err(e.into())
            }
        }
    }
    Ok(records)
}

pub async fn load_iq_report(pool: &PgPool, filename: String, date_str: &str) -> anyhow::Result<u32> {
    let records = read_iq_report(filename)?;

    let format = format_description!("[month]/[day]/[year]");
    let date = Date::parse(date_str, &format).unwrap();

    let cmd = r#"
        INSERT INTO IQ_Report (
            Stock,
            Div_Growth,
            Value_Rating,
            Price,
            Dividend,
            Yield,
            Points_Down,
            Percent_Down,
            Undervalue_Lo_Price,
            Undervalue_Hi_Yield,
            Points_Up,
            Percent_Up,
            Overvalue_Hi_Price,
            Overvalue_Lo_Yield,
            SP_Rating,
            Lo_52_Wk,
            Hi_52_Wk,
            Book_Value,
            Earnings_12_Mo,
            Price_to_Earnings,
            Pay_Out,
            Div_In_Dgr,
            Long_Term_Debt,
            Bluechip_Criteria,
            Symbol,
            Sector,
            Industry,
            Sub_Sector,
            Div_Growth_3_Year,
            Div_Growth_5_Year,
            Div_Growth_10_Year,
            Report_Date
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19,
                 $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32)
        "#;

    let mut count = 0;
    for record in records {
        // let encoded: Vec<u8> = bincode::serialize(&record).unwrap();
        // let hash = digest(encoded);

        sqlx::query(cmd)
            .bind(record.Stock)
            .bind(record.Div_Growth)
            .bind(record.Value_Rating)
            .bind(record.Price)
            .bind(record.Dividend)
            .bind(record.Yield)
            .bind(record.Points_Down)
            .bind(record.Percent_Down)
            .bind(record.Undervalue_Lo_Price)
            .bind(record.Undervalue_Hi_Yield)
            .bind(record.Points_Up)
            .bind(record.Percent_Up)
            .bind(record.Overvalue_Hi_Price)
            .bind(record.Overvalue_Lo_Yield)
            .bind(record.SP_Rating)
            .bind(record.Lo_52_Wk)
            .bind(record.Hi_52_Wk)
            .bind(record.Book_Value)
            .bind(record.Earnings_12_Mo)
            .bind(record.Price_to_Earnings)
            .bind(record.Pay_Out)
            .bind(record.Div_In_Dgr)
            .bind(record.Long_Term_Debt)
            .bind(record.Bluechip_Criteria)
            .bind(record.Symbol)
            .bind(record.Sector)
            .bind(record.Industry)
            .bind(record.Sub_Sector)
            .bind(record.Div_Growth_3_Year)
            .bind(record.Div_Growth_5_Year)
            .bind(record.Div_Growth_10_Year)
            .bind(date)
            .execute(pool)
            .await?;
        count += 1;
    }
    Ok(count)
}