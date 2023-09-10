mod account_history;
mod account_positions;
mod commands;
mod config;
mod data_filter;
mod date_format;
mod decimal_formats;

use crate::account_history::load_account_history;
use crate::account_positions::{load_account_positions_dividends, load_account_positions_overview};
use crate::commands::{Args, Command};
use crate::config::{get_config, get_file_path};
use sqlx::postgres::PgPool;
use structopt::StructOpt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let config = get_config();

    let args = Args::from_args_safe()?;
    let pool = PgPool::connect(&config.db_connection_string).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("migration completed");

    match args.cmd {
        Some(Command::LoadAccountHistory { filename }) => match get_file_path(&config, &filename) {
            Ok(filename) => {
                println!("Loading account history from: '{filename}'");
                let count = load_account_history(&pool, filename).await?;
                println!("Added {count} account records");
            }
            Err(e) => println!("Error: {e}"),
        },
        Some(Command::LoadAccountPositionOverview { filename }) => match get_file_path(&config, &filename) {
            Ok(filename) => {
                if !filename.contains("Portfolio_Positions_Overview") {
                    println!("Error: not a Portfolio_Positions_Overview file: {filename}")
                } else {
                    println!("Loading account positions_overview from: '{filename}'");
                    let count = load_account_positions_overview(&pool, filename).await?;
                    println!("Added {count} account records");
                }
            }
            Err(e) => println!("Error: {e}"),
        },
        Some(Command::LoadAccountPositionDividends { filename }) => match get_file_path(&config, &filename) {
            Ok(filename) => {
                if !filename.contains("Portfolio_Positions_Dividend") {
                    println!("Error: not a Portfolio_Positions_Dividend file: {filename}")
                } else {
                    println!("Loading account position dividends from: '{filename}'");
                    let count = load_account_positions_dividends(&pool, filename).await?;
                    println!("Added {count} account records");
                }
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
            println!("Printing list of all todos");
            // list_todos(&pool).await?;
        }
    }

    Ok(())
}
