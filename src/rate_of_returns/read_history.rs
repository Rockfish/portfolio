use chrono::NaiveDate;
use sqlx::types::Decimal;
use sqlx::PgPool;

#[derive(Debug)]
#[allow(dead_code)]
pub struct ActivityResult {
    symbol: String,
    activity: String,
    run_date: NaiveDate,
    quantity: Decimal,
    amount: Decimal,
}

pub async fn get_bought_records(pool: &PgPool, symbol: &str) {
    let records = sqlx::query_as!(
        ActivityResult,
        r#"SELECT symbol as "symbol!",
                  activity as "activity!",
                  run_date as "run_date!",
                  ABS(SUM(amount)) as "amount!",
                  ABS(SUM(quantity)) as "quantity!"
        FROM account_history_normalized
        WHERE symbol not in ('FDRXX', 'SPAXX')
            and activity in ('Bought', 'Reinvestment', 'Sold')
            and symbol = $1
        GROUP BY symbol, activity, run_date;"#,
        symbol
    )
    .fetch_all(pool)
    .await
    .unwrap();

    for rec in records {
        println!("{:?}", rec)
    }
}
