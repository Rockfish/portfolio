mod account_history;
mod account_positions;
mod blue_chips;
mod chart_data;
mod commands;
mod config;
mod data_filter;
mod date_format;
mod decimal_formats;
mod iq_report;
mod rate_of_returns;

use crate::account_history::load_account_history;
use crate::account_positions::{load_account_positions_dividends, load_account_positions_overview};
use crate::blue_chips::load_blue_chip_stocks;
use crate::chart_data::load_chart_data;
use crate::commands::{Args, Command};
use crate::config::{get_config, get_file_path};
use crate::iq_report::load_iq_report;
use crate::rate_of_returns::read_history::get_bought_records;
use log::info;
use sqlx::postgres::PgPool;
use structopt::StructOpt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let config = get_config();
    let args = Args::from_args_safe()?;
    let pool = PgPool::connect(&config.db_connection_string).await?;

    // let db = get_db_connection(&config.db_connection_string).await?;
    // println!("db connection is ok: {}", db.ping().await.is_ok());

    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("migration completed");

    get_bought_records(&pool, "ORCL").await;

    match args.cmd {
        Some(Command::LoadAccountHistory { filename }) => match get_file_path(&config, &filename) {
            Ok(filename) => {
                println!("Loading account history from: '{filename}'");
                let count = load_account_history(&pool, &filename).await?;
                println!("Added {count} records");
            }
            Err(e) => println!("Error: {e}"),
        },
        Some(Command::LoadAccountPositionOverview { filename, date }) => match get_file_path(&config, &filename) {
            Ok(filename) => {
                if !(filename.contains("Portfolio_Positions") && filename.contains("overview")) {
                    println!("Error: In correct file type. Name must be of format: Portfolio_Positions_<date>_overview.csv: {filename}")
                } else {
                    println!("Loading account positions_overview from: '{filename}'");
                    let count = load_account_positions_overview(&pool, filename, &date).await?;
                    println!("Added {count} records");
                }
            }
            Err(e) => println!("Error: {e}"),
        },
        Some(Command::LoadAccountPositionDividends { filename, date }) => match get_file_path(&config, &filename) {
            Ok(filename) => {
                if !(filename.contains("Portfolio_Positions") && filename.contains("dividend")) {
                    println!("Error: In correct file type. Name must be of format: Portfolio_Positions_<date>_dividend.csv: {filename}")
                } else {
                    println!("Loading account position dividends from: '{filename}'");
                    let count = load_account_positions_dividends(&pool, filename, &date).await?;
                    println!("Added {count} records");
                }
            }
            Err(e) => println!("Error: {e}"),
        },
        Some(Command::LoadBlueChipStocks { filename }) => match get_file_path(&config, &filename) {
            Ok(filename) => {
                println!("Loading blue chips stocks from: '{filename}'");
                let count = load_blue_chip_stocks(&pool, filename).await?;
                println!("Added {} records", count);
            }
            Err(e) => println!("Error: {e}"),
        },
        Some(Command::LoadChartData { filename }) => match get_file_path(&config, &filename) {
            Ok(filename) => {
                println!("Loading chart data from: '{filename}'");
                let count = load_chart_data(&pool, filename).await?;
                println!("Added {} records", count);
            }
            Err(e) => println!("Error: {e}"),
        },
        Some(Command::LoadIQReport { filename, date }) => match get_file_path(&config, &filename) {
            Ok(filename) => {
                println!("Loading chart data from: '{filename}'");
                let count = load_iq_report(&pool, filename, &date).await?;
                println!("Added {} records", count);
            }
            Err(e) => println!("Error: {e}"),
        },
        Some(Command::Done { id }) => {
            println!("Marking todo {id} as done");
            // if complete_todo(&pool, id).await? {
            //     println!("Todo {id} is marked as done");
            // } else {
            //     println!("Invalid id {id}");
            // }
        }
        None => {
            println!("Done.");
        }
    }

    Ok(())
}
