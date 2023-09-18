use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(subcommand)]
    pub(crate) cmd: Option<Command>,
}

#[derive(StructOpt)]
pub enum Command {
    LoadAccountHistory { filename: String },
    LoadAccountPositionOverview { filename: String },
    LoadAccountPositionDividends { filename: String },
    LoadBlueChipStocks { filename: String },
    LoadChartData { filename: String },
    // data format: [month]/[day]/[year]
    LoadIQReport { filename: String, date: String },
    Done { id: u64 },
}
