#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::fs::File;
use std::io::{BufReader, Write};
use csv::{ReaderBuilder, Trim};
use futures::TryStreamExt;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use time::Date;
use time::macros::format_description;
#[allow(unused_imports)]
use crate::decimal_formats::*;

#[allow(unused_imports)]
use crate::date_format;

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

// #[serde_as]
#[allow(dead_code)]
#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct IQ_Report_Table {
    #[serde(skip)]
    id: i32,
    stock: String,
    div_growth: String,
    value_rating: String,
    price: Option<Decimal>,
    dividend: Option<Decimal>,
    r#yield: Option<Decimal>,
    points_down: Option<Decimal>,
    percent_down: Option<Decimal>,
    undervalue_lo_price: Option<Decimal>,
    undervalue_hi_yield: Option<Decimal>,
    points_up: Option<Decimal>,
    percent_up: Option<Decimal>,
    overvalue_hi_price: Option<Decimal>,
    overvalue_lo_yield: Option<Decimal>,
    sp_rating: String,
    lo_52_wk: Option<Decimal>,
    hi_52_wk: Option<Decimal>,
    book_value: Option<Decimal>,
    earnings_12_mo: Option<Decimal>,
    price_to_earnings: Option<Decimal>,
    pay_out: Option<Decimal>,
    div_in_dgr: String,
    long_term_debt: Option<Decimal>,
    bluechip_criteria: Option<Decimal>,
    symbol: String,
    sector: String,
    industry: String,
    sub_sector: String,
    div_growth_3_year: Option<Decimal>,
    div_growth_5_year: Option<Decimal>,
    div_growth_10_year : Option<Decimal>,
    #[serde(with = "date_format")]
    report_date: Option<Date>,
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

pub async fn iq_report_save_all(pool: &PgPool, filename: &str) {
    let mut output = File::create(filename).unwrap();
    let mut stream = sqlx::query_as::<_, IQ_Report_Table>("select * from iq_report order by report_date, symbol")
        .fetch(pool);

    // output.write("[\n".as_bytes()).unwrap();
    while let Ok(item) = stream.try_next().await {
        match item {
            None => break,
            Some(report) => {
                // let data = ron::to_string(&report).unwrap();
                let data = serde_json::to_string(&report).unwrap();
                output.write(data.as_bytes()).unwrap();
                output.write("\n".as_bytes()).unwrap();
            }
        }
    }
    // output.write("]\n".as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]
    use sqlx::PgPool;
    use crate::config::{get_config, make_file_path};
    use crate::iq_report::iq_report_save_all;

    #[allow(unused_imports)]
    use crate::date_format;

    #[tokio::test]
    async fn test_save_all() {
        let config = get_config();
        let filename = make_file_path(&config, "data_backup/iq_report_table_save_all.json").unwrap();

        let pool = PgPool::connect(&config.db_connection_string).await;
        println!("Saving iq_report table data to: {}", filename);

        if let Ok(pool) = pool {
            iq_report_save_all(&pool, &filename).await;
        }
        println!("Done");
    }
}