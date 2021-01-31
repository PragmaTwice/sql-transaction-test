mod permutation;
mod reader;
mod options;
mod process;

use clap::Clap;
use options::Options;
use mysql::Result;
use process::DatabaseProcess;

fn main() -> Result<()> {
    let options: Options = Options::parse();

    let mut db_process = DatabaseProcess::new(
        options.url.as_str(), 
        Options::parse_input(options.input_path).expect("expected two file paths")
    )?;

    while {
        db_process.work()?;
        db_process.next()
    } {}

    Ok(())
}
