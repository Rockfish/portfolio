use ron::from_str;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use sqlx::PgPool;

#[derive(Debug, Deserialize, Serialize)]
pub struct BlueChipStock {
    symbol: String,
    name: String,
    chart_link: String,
}

pub fn read_blue_chips_file(filename: &str) -> anyhow::Result<Vec<BlueChipStock>> {
    let file = File::open(filename).expect("Failed to open file");
    let buf_reader = BufReader::new(file);

    let mut stocks: Vec<BlueChipStock> = vec![];

    for line in buf_reader.lines().flatten() {
        let stock: BlueChipStock = match from_str(&line) {
            Ok(stock) => stock,
            Err(e) => return Err(e.into()),
        };
        stocks.push(stock);
    }
    Ok(stocks)
}

pub async fn load_blue_chip_stocks(pool: &PgPool, filename: String) -> anyhow::Result<u32> {
    let records = read_blue_chips_file(&filename)?;

    let cmd = r#"
        INSERT INTO Blue_Chips (
            symbol,
            name,
            chart_link
        ) VALUES ($1, $2, $3)
        "#;

    let mut count = 0;
    for record in records {
        // let encoded: Vec<u8> = bincode::serialize(&record).unwrap();
        // let hash = digest(encoded);

        sqlx::query(cmd)
            .bind(record.symbol)
            .bind(record.name)
            .bind(record.chart_link)
            // .bind(hash)
            .execute(pool)
            .await?;
        count += 1;
    }
    Ok(count)
}
