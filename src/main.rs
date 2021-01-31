mod permutation;
mod reader;
mod options;
mod process;

use clap::Clap;
use options::Options;
use mysql::Result;
use process::DatabaseProcess;
use log::info;

fn main() -> Result<()> {
    let options: Options = Options::parse();

    env_logger::init();

    let mut db_process = DatabaseProcess::new(
        options.url.as_str(), 
        Options::parse_input(options.input_path).expect("expected two file paths")
    )?;

    info!("connected to {}", options.url);

    let mut count = 0;
    let total = db_process.count().round();

    while {
        count += 1;
        info!("iterating on permutation {}/{}", count, total);

        db_process.work()?;
        db_process.next()
    } {}

    Ok(())
}
