//! ## Manual Page
//! ```shell
//! $ ./sql-transaction-test --help
//! sql-transaction-test 0.1.0
//! PragmaTwice <i@twice.moe>
//! test all merged permutation of SQL statements from two files
//! 
//! USAGE:
//!     sql-transaction-test.exe --url <url> [input-path]...
//! 
//! ARGS:
//!     <input-path>...    paths of two SQL files
//! 
//! FLAGS:
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//! 
//! OPTIONS:
//!     -u, --url <url>    URI of database server
//! ```

use clap::{Clap, crate_version, crate_authors};

/// test all merged permutation of SQL statements from two files
#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct Options {
    /// URI of database server
    #[clap(short, long)]
    pub url: String,

    /// paths of two SQL files
    pub input_path: Vec<String>,
}

impl Options {
    /// convert the vector of paths to a tuple of two paths
    pub fn parse_input(mut input: Vec<String>) -> Option<(String, String)> {
        if input.len() == 2 {
            Some((input.remove(0), input.remove(0)))
        } else if input.len() > 2 {
            Some((input.swap_remove(0), input.swap_remove(1)))
        } else {
            None
        }
    }
}
