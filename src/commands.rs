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
    Done { id: u64 },
}
