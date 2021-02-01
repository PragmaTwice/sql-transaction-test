//! ## Quick Start
//! 
//! ```shell
//! $ RUST_LOG=debug cargo run -- -u mysql://username:password@host:port/db-name asset/a.sql asset/b.sql 
//! ```
//! You can check the result of this command in step "Run example" from the latest workflow 
//! run of [Actions](https://github.com/PragmaTwice/sql-transaction-test/actions?query=workflow%3ABuildAndTest), 
//! where a TiDB service was pulled up in CI for testing.
//! 
//! ## Functions
//! The two SQL files will be split by rows, trying the case where they are all crossed 
//! while keeping the internal order of each file unchanged. 
//! 
//! The sequence of SQL statements under each alignment will be submitted to the server.
//! 
//! ## Features
//! - permutation calculation by bit vector
//! - data traversing with iterator patterns
//! - unit testing and real-world simulation in CI

/// permutation generator and iterator
mod permutation;
/// IO reader of double files for iterating in any permutation
mod reader;
/// command-line options of this program
mod options;
/// database client for iteraing SQL statements from two files
mod process;

use clap::Clap;
use options::Options;
use mysql::Result;
use process::DatabaseProcess;
use log::info;

#[doc(hidden)]
fn main() -> Result<()> {
    let options: Options = Options::parse();

    env_logger::init();

    let mut db_process = DatabaseProcess::new(
        options.url.as_str(), 
        Options::parse_input(options.input_path).expect("expected two file paths")
    )?;

    info!("connected to {}", options.url);

    let mut count = 0u128;
    let total = db_process.count().round();

    while {
        count += 1;
        info!("iterating on permutation {}/{}", count, total);

        db_process.work()?;
        db_process.next()
    } {}

    Ok(())
}
