mod account_history;
mod data_filter;
mod date_format;
mod dbconfig;
mod positions;

use crate::account_history::load_account_history;
use crate::dbconfig::get_db_config;
use sqlx::mysql::MySqlPool;
use std::path::Path;
use structopt::StructOpt;

const DATA_FOLDER: &str = "/Users/john/Portfolio_Data";

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    LoadAccountHistory { filename: String },
    Done { id: u64 },
}

fn test_path(filename: &str) -> Result<String, String> {
    let path = Path::new(filename);
    if path.exists() {
        if path.is_file() {
            match path.to_str() {
                None => panic!("path is not valid utc-8"),
                Some(s) => return Ok(s.to_string()),
            }
        } else {
            return Err(format!("path: '{filename}' exits but is not a file"));
        }
    }
    Err(format!("path: '{filename}' does not exist"))
}

fn get_file_path(filename: &str) -> Result<String, String> {
    match test_path(filename) {
        Ok(filename) => Ok(filename),
        Err(_) => match Path::new(DATA_FOLDER).join(filename).to_str() {
            None => panic!("path.to_str() error"),
            Some(filepath) => match test_path(filepath) {
                Ok(filename) => Ok(filename),
                Err(e) => Err(e),
            },
        },
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let dbconfig = get_db_config();

    let args = Args::from_args_safe()?;
    let pool = MySqlPool::connect(&dbconfig.url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    match args.cmd {
        Some(Command::LoadAccountHistory { filename }) => match get_file_path(&filename) {
            Ok(filename) => {
                println!("Loading account history from: '{filename}'");
                let count = load_account_history(&pool, filename).await?;
                println!("Added {count} account records");
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
