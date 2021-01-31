use std::fs::File;
use mysql::{Conn, Opts, Result, prelude::Queryable};
use crate::reader::CoupleReader;
use log::debug;

/// connect database and forward conntent from reader
pub struct DatabaseProcess {
    /// double connection
    conns: (Conn, Conn),

    /// file reader
    reader: CoupleReader<File>,
}

impl DatabaseProcess {
    /// construct a DatabaseProcess from database options (maybe URI) and two file paths
    pub fn new<T: Into<Opts>>(opts: T, paths: (String, String)) -> Result<Self> {
        let opts : Opts = opts.into();

        Ok(DatabaseProcess {
            conns: (Conn::new(opts.clone())?, Conn::new(opts)?),
            reader: CoupleReader::new(File::open(paths.0)?, File::open(paths.1)?)?,
        })
    }

    /// forward CoupleReader::next()
    pub fn next(&mut self) -> bool {
        self.reader.next()
    }

    /// work on current permutation
    pub fn work(&mut self) -> Result<()> {
        let mut iter = self.reader.iter()?;

        while let Some((res, x)) = iter.next() {
            let res = res?;

            debug!("run SQL query '{}'", res);
            if x { &mut self.conns.0 } else { &mut self.conns.1 }.query_drop(res)?;
        }

        Ok(())
    }
}
