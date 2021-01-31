mod permutation;
mod reader;
mod options;

use clap::Clap;
use options::Options;
use mysql::{Opts, Conn, Result};

fn main() -> Result<()> {
    let options: Options = Options::parse();

    let (first_path, second_path) = Options::parse_input(options.input_path).expect("expected two file paths");
    
    let db_conn = Conn::new(Opts::from_url(options.url.as_str())?)?;

    Ok(())
}
