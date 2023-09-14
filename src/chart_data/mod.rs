use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
// use sha256::digest;
use sqlx::PgPool;

mod extract_data;

#[derive(Debug, Serialize, Deserialize)]
struct ChartData {
    symbol: String,
    chart: String,
    update_date: String,
    quality_rank: String,
    shares_outstanding: String,
    institution_own: String,
    div_paid_since: String,
    profit_margin: String,
    ttm_earnings: String,
    pe_ratio: String,
    book_value: String,
    div_payout: String,
    current_price: String,
    current_yield: String,
    overvalue_price: String,
    overvalue_pts_up: String,
    overvalue_yield: String,
    overvalue_percent_up: String,
    undervalue_price: String,
    undervalue_pts_dn: String,
    undervalue_yield: String,
    undervalue_percent_dn: String,
}

impl Default for ChartData {
    fn default() -> Self {
        ChartData {
            symbol: "".to_string(),
            chart: "".to_string(),
            update_date: "".to_string(),
            quality_rank: "".to_string(),
            shares_outstanding: "".to_string(),
            institution_own: "".to_string(),
            div_paid_since: "".to_string(),
            profit_margin: "".to_string(),
            ttm_earnings: "".to_string(),
            pe_ratio: "".to_string(),
            book_value: "".to_string(),
            div_payout: "".to_string(),
            current_price: "".to_string(),
            current_yield: "".to_string(),
            overvalue_price: "".to_string(),
            overvalue_pts_up: "".to_string(),
            overvalue_yield: "".to_string(),
            overvalue_percent_up: "".to_string(),
            undervalue_price: "".to_string(),
            undervalue_pts_dn: "".to_string(),
            undervalue_yield: "".to_string(),
            undervalue_percent_dn: "".to_string(),
        }
    }
}

enum Flag {
    None,
    Current,
    Overvalue,
    Undervalue,
    // Div,
    // Eps,
}

pub async fn load_chart_data(pool: &PgPool, filename: String) -> anyhow::Result<u32> {
    let records = read_chart_data(filename);

    let cmd = r#"
        INSERT INTO chart_data (
            symbol,
            chart,
            update_date,
            quality_rank,
            shares_outstanding,
            institution_own,
            div_paid_since,
            profit_margin,
            ttm_earnings,
            pe_ratio,
            book_value,
            div_payout,
            current_price,
            current_yield,
            overvalue_price,
            overvalue_pts_up,
            overvalue_yield,
            overvalue_percent_up,
            undervalue_price,
            undervalue_pts_dn,
            undervalue_yield,
            undervalue_percent_dn
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
        "#;

    let mut count = 0;
    for record in records {
        // let encoded: Vec<u8> = bincode::serialize(&record).unwrap();
        // let hash = digest(encoded);

        sqlx::query(cmd)
            .bind(record.symbol)
            .bind(record.chart)
            .bind(record.update_date)
            .bind(record.quality_rank)
            .bind(record.shares_outstanding)
            .bind(record.institution_own)
            .bind(record.div_paid_since)
            .bind(record.profit_margin)
            .bind(record.ttm_earnings)
            .bind(record.pe_ratio)
            .bind(record.book_value)
            .bind(record.div_payout)
            .bind(record.current_price)
            .bind(record.current_yield)
            .bind(record.overvalue_price)
            .bind(record.overvalue_pts_up)
            .bind(record.overvalue_yield)
            .bind(record.overvalue_percent_up)
            .bind(record.undervalue_price)
            .bind(record.undervalue_pts_dn)
            .bind(record.undervalue_yield)
            .bind(record.undervalue_percent_dn)
            // .bind(hash)
            .execute(pool)
            .await?;
        count += 1;
    }

    Ok(count)
}

fn read_chart_data(filename: String) -> Vec<ChartData> {
    let mut file = File::open(filename).unwrap();
    let mut data: String = String::new();
    file.read_to_string(&mut data).unwrap();
    serde_json::from_str::<Vec<ChartData>>(&data).unwrap()
}
