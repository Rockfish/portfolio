#![allow(dead_code)]

use crate::entities::account_history_normalized as history;
use chrono::NaiveDate;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use sea_orm::sea_query::{Alias, Expr, Func, PostgresQueryBuilder, Query};
use sea_orm::Condition;
use sea_orm::DatabaseBackend::Postgres;
use sea_orm::EntityTrait;
use sea_orm::{ColumnTrait, DatabaseConnection, QueryFilter};
use sea_orm::{ConnectionTrait, Order, QueryOrder, QueryResult, QuerySelect, QueryTrait, Statement};
use sqlx::postgres::PgRow;
use sqlx::types::BigDecimal;
use sqlx::{PgPool, Row};
use std::str::FromStr;

#[derive(Debug)]
pub struct ActivityResult {
    symbol: String,
    run_date: NaiveDate,
    quantity: BigDecimal,
    amount: BigDecimal,
}

#[derive(Debug)]
pub struct ActivityResultOption {
    symbol: Option<String>,
    run_date: Option<NaiveDate>,
    quantity: Option<BigDecimal>,
    amount: Option<BigDecimal>,
}

pub async fn get_bought_records(db: &DatabaseConnection, pool: &PgPool, symbol: &str) {
    let bought_records: Vec<history::Model> = history::Entity::find()
        .filter(
            Condition::any()
                .add(history::Column::Activity.eq("Bought"))
                .add(history::Column::Activity.eq("Reinvestment")),
        )
        .filter(history::Column::Symbol.is_not_in(["FDRXX", "SPAXX"]))
        .order_by(history::Column::Symbol, Order::Asc)
        .order_by(history::Column::RunDate, Order::Asc)
        .all(db)
        .await
        .unwrap();

    let sold_records: Vec<history::Model> = history::Entity::find()
        .filter(history::Column::Activity.eq("Sold"))
        .filter(history::Column::Symbol.is_not_in(["FDRXX", "SPAXX"]))
        // .group_by(history::Column::Symbol)
        // .group_by(history::Column::RunDate)
        .order_by(history::Column::Symbol, Order::Asc)
        .order_by(history::Column::RunDate, Order::Asc)
        .all(db)
        .await
        .unwrap();

    let query_string = Query::select()
        .column(history::Column::Symbol)
        .column(history::Column::RunDate)
        .expr_as(Func::sum(Expr::col(history::Column::Amount)), Alias::new("amount"))
        .expr_as(Func::sum(Expr::col(history::Column::Quantity)), Alias::new("quantity"))
        .from(history::Entity)
        .and_where(history::Column::Activity.eq("Sold"))
        .and_where(history::Column::Symbol.is_not_in(["FDRXX", "SPAXX"]))
        .group_by_columns([history::Column::Symbol, history::Column::RunDate])
        .to_owned()
        .to_string(PostgresQueryBuilder);

    for sold in sold_records {
        println!(
            "Sold - {}, {}, {:?}, {:?}, {:?}",
            sold.symbol.as_ref().unwrap(),
            sold.activity.as_ref().unwrap(),
            sold.quantity.as_ref().unwrap().abs(),
            sold.amount.as_ref().unwrap().abs(),
            sold.run_date.as_ref().unwrap(),
        );

        for bought in bought_records.iter().find(|b| b.symbol == sold.symbol) {
            println!(
                "    Bought - {}, {}, {:?}, {:?}, {:?}",
                &bought.symbol.as_ref().unwrap(),
                &bought.activity.as_ref().unwrap(),
                &bought.quantity.as_ref().unwrap().abs(),
                &bought.amount.as_ref().unwrap().abs(),
                &bought.run_date.as_ref().unwrap(),
            );
        }

        println!();
    }

    println!("{:?}", &query_string);

    let result: Vec<QueryResult> = db.query_all(Statement::from_string(Postgres, query_string)).await.unwrap();

    for r in result {
        println!(
            "{:?} {:?}, {:?}, {:?}",
            r.try_get::<String>("", "symbol"),
            r.try_get::<NaiveDate>("", "run_date"),
            r.try_get::<Decimal>("", "amount"),
            r.try_get::<Decimal>("", "quantity"),
        );
    }

    println!();

    // type safe but still needs mapping. Record type is generated by macro.
    let records = sqlx::query!(
        r#"
        SELECT symbol,
           run_date,
           SUM(amount) as amount,
           SUM(quantity) as quantity
        FROM account_history_normalized
        WHERE symbol not in ('FDRXX', 'SPAXX')
        and activity in ('Sold')
        GROUP BY symbol, run_date;
    "#
    )
    .fetch_all(pool)
    .await
    .unwrap();

    let recs: Vec<ActivityResult> = records
        .iter()
        .map(|row| ActivityResult {
            symbol: row.symbol.as_ref().unwrap().clone(),
            run_date: row.run_date.as_ref().unwrap().clone(),
            quantity: row.quantity.as_ref().unwrap().clone().abs(),
            amount: row.amount.as_ref().unwrap().clone().abs(),
        })
        .collect();

    for rec in recs {
        println!("{:?}", rec)
    }

    println!();

    // A bit more code, but lots of control over mapping. Not forced to use options. No cloning needed.
    let records = sqlx::query(
        r#"
        SELECT symbol,
            run_date,
            SUM(amount) as amount,
            SUM(quantity) as quantity
            FROM account_history_normalized
        WHERE symbol not in ('FDRXX', 'SPAXX')
            and activity in ('Sold')
        GROUP BY symbol, run_date;
    "#,
    )
    .map(|row: PgRow| ActivityResult {
        symbol: row.try_get("symbol").unwrap(),
        run_date: row.try_get("run_date").unwrap(),
        quantity: row.try_get::<BigDecimal, &str>("quantity").unwrap().abs(),
        amount: row.try_get::<BigDecimal, &str>("amount").unwrap().abs(),
    })
    .fetch_all(pool)
    .await
    .unwrap();

    for rec in records {
        println!("{:?}", rec)
    }

    // auto mapping meaning must use option<> for nullable fields. No control over mapping.
    let records = sqlx::query_as!(
        ActivityResultOption,
        r#"SELECT symbol,
                  run_date,
                  SUM(amount) as amount,
                  SUM(quantity) as quantity
        FROM account_history_normalized
        WHERE symbol not in ('FDRXX', 'SPAXX')
            and activity in ('Sold')
        GROUP BY symbol, run_date;
    "#,
    )
    .fetch_all(pool)
    .await
    .unwrap();

    for rec in records {
        println!("{:?}", rec)
    }

    // auto mapping defaults to for nullable fields.
    // Use "<name>!" to force assuming fields are not nullable.
    // No control over mapping, but convenient. Use query for adjusting data.
    let records = sqlx::query_as!(
        ActivityResult,
        r#"SELECT symbol as "symbol!",
                  run_date as "run_date!",
                  ABS(SUM(amount)) as "amount!",
                  ABS(SUM(quantity)) as "quantity!"
        FROM account_history_normalized
        WHERE symbol not in ('FDRXX', 'SPAXX')
            and activity in ('Sold')
        GROUP BY symbol, run_date;"#,
    )
    .fetch_all(pool)
    .await
    .unwrap();

    for rec in records {
        println!("{:?}", rec)
    }

    let big: Option<BigDecimal> = BigDecimal::from_f32(15.2);

    let num = big.unwrap().to_f32().unwrap();

    println!("{num}");

    let input = "0.12345678901234567890";
    let dec = BigDecimal::from_str(&input).unwrap();
    let little = Decimal::from_str(input).unwrap();
    let float = f32::from_str(&input).unwrap();

    println!(
        "Input ({}) with 10 decimals: big decimal: {}  decimal: {}  float: {})",
        input,
        &dec * &dec,
        little,
        float
    );
}
