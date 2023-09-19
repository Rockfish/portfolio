#![allow(dead_code)]

#[allow(unused_imports)]
use crate::decimal_formats::*;
use csv::{ReaderBuilder, Trim};
use futures::TryStreamExt;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::fs::File;
use std::io::{BufReader, Write};
use time::macros::format_description;
use time::Date;

#[allow(unused_imports)]
use crate::date_format;

#[derive(Debug, Serialize, Deserialize)]
pub struct IqReport {
    #[serde(rename = "STOCK")]
    stock: String,
    #[serde(rename = "Div Growth")]
    div_growth: String,
    #[serde(rename = "Value Rating")]
    value_rating: String,
    #[serde(rename = "Price")]
    price: Option<Decimal>,
    #[serde(rename = "Dividend")]
    dividend: Option<Decimal>,
    #[serde(rename = "Yield", deserialize_with = "deserialize_percentage")]
    r#yield: Option<Decimal>,
    #[serde(rename = "Pts Dn")]
    points_down: Option<Decimal>,
    #[serde(rename = "% Down", deserialize_with = "deserialize_percentage")]
    percent_down: Option<Decimal>,
    #[serde(rename = "Undervalue LoPr")]
    undervalue_lo_price: Option<Decimal>,
    #[serde(rename = "Undervalue HiYld", deserialize_with = "deserialize_percentage")]
    undervalue_hi_yield: Option<Decimal>,
    #[serde(rename = "Pts Up")]
    points_up: Option<Decimal>,
    #[serde(rename = "% Up", deserialize_with = "deserialize_percentage")]
    percent_up: Option<Decimal>,
    #[serde(rename = "Overvalue HiPr")]
    overvalue_hi_price: Option<Decimal>,
    #[serde(rename = "Overvalue LoYld", deserialize_with = "deserialize_percentage")]
    overvalue_lo_yield: Option<Decimal>,
    #[serde(rename = "S&P")]
    sp_rating: String,
    #[serde(rename = "52 wik Lo")]
    lo_52_wk: Option<Decimal>,
    #[serde(rename = "52 wk Hi")]
    hi_52_wk: Option<Decimal>,
    #[serde(rename = "Bk Val")]
    book_value: Option<Decimal>,
    #[serde(rename = "12-Mo Earn")]
    earnings_12_mo: Option<Decimal>,
    #[serde(rename = "P/E")]
    price_to_earnings: Option<Decimal>,
    #[serde(rename = "Pay out", deserialize_with = "deserialize_percentage")]
    pay_out: Option<Decimal>,
    #[serde(rename = "Div in Dgr")]
    div_in_dgr: String,
    #[serde(rename = "L/T Debt", deserialize_with = "deserialize_percentage")]
    long_term_debt: Option<Decimal>,
    #[serde(rename = "BC")]
    bluechip_criteria: Option<Decimal>,
    #[serde(rename = "Tic")]
    symbol: String,
    #[serde(rename = "SECTOR")]
    sector: String,
    #[serde(rename = "INDUSTRY")]
    industry: String,
    #[serde(rename = "SUB-SECTOR")]
    sub_sector: String,
    #[serde(rename = "3 Year Div Growth", deserialize_with = "deserialize_percentage")]
    div_growth_3_year: Option<Decimal>,
    #[serde(rename = "5 year Div Growth", deserialize_with = "deserialize_percentage")]
    div_growth_5_year: Option<Decimal>,
    #[serde(rename = "10 Year Div Growth", deserialize_with = "deserialize_percentage")]
    div_growth_10_year: Option<Decimal>,
    #[serde(skip_deserializing)]
    report_date: Option<Date>,
}

// #[serde_as]
#[allow(dead_code)]
#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct IqReportTable {
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
    div_growth_10_year: Option<Decimal>,
    #[serde(with = "date_format")]
    report_date: Option<Date>,
}

pub fn read_iq_report(filename: String) -> anyhow::Result<Vec<IqReport>> {
    let file = File::open(filename).expect("Failed to open file");
    let buf_reader = BufReader::new(file);

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .quote(b'"')
        .trim(Trim::All)
        .flexible(true)
        .from_reader(buf_reader);

    let mut records: Vec<IqReport> = vec![];
    for result in reader.deserialize() {
        match result {
            Ok(record) => records.push(record),
            Err(e) => {
                println!("{:#?}", e);
                return Err(e.into());
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
            stock,
            div_growth,
            value_rating,
            price,
            dividend,
            yield,
            points_down,
            percent_down,
            undervalue_lo_price,
            undervalue_hi_yield,
            points_up,
            percent_up,
            overvalue_hi_price,
            overvalue_lo_yield,
            sp_rating,
            lo_52_wk,
            hi_52_wk,
            book_value,
            earnings_12_mo,
            price_to_earnings,
            pay_out,
            div_in_dgr,
            long_term_debt,
            bluechip_criteria,
            symbol,
            sector,
            industry,
            sub_sector,
            div_growth_3_year,
            div_growth_5_year,
            div_growth_10_year,
            report_date
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19,
                 $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32)
        "#;

    let mut count = 0;
    for record in records {
        // let encoded: Vec<u8> = bincode::serialize(&record).unwrap();
        // let hash = digest(encoded);

        sqlx::query(cmd)
            .bind(record.stock)
            .bind(record.div_growth)
            .bind(record.value_rating)
            .bind(record.price)
            .bind(record.dividend)
            .bind(record.r#yield)
            .bind(record.points_down)
            .bind(record.percent_down)
            .bind(record.undervalue_lo_price)
            .bind(record.undervalue_hi_yield)
            .bind(record.points_up)
            .bind(record.percent_up)
            .bind(record.overvalue_hi_price)
            .bind(record.overvalue_lo_yield)
            .bind(record.sp_rating)
            .bind(record.lo_52_wk)
            .bind(record.hi_52_wk)
            .bind(record.book_value)
            .bind(record.earnings_12_mo)
            .bind(record.price_to_earnings)
            .bind(record.pay_out)
            .bind(record.div_in_dgr)
            .bind(record.long_term_debt)
            .bind(record.bluechip_criteria)
            .bind(record.symbol)
            .bind(record.sector)
            .bind(record.industry)
            .bind(record.sub_sector)
            .bind(record.div_growth_3_year)
            .bind(record.div_growth_5_year)
            .bind(record.div_growth_10_year)
            .bind(date)
            .execute(pool)
            .await?;
        count += 1;
    }
    Ok(count)
}

pub async fn iq_report_save_all(pool: &PgPool, filename: &str) {
    let mut output = File::create(filename).unwrap();
    let mut stream = sqlx::query_as::<_, IqReportTable>("select * from iq_report order by report_date, symbol").fetch(pool);

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
    use crate::config::{get_config, make_file_path};
    use crate::iq_report::iq_report_save_all;
    use sqlx::PgPool;

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
