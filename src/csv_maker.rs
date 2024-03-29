use std::{fs::File, io::BufWriter, path::PathBuf};

use csv::Writer;

use crate::CsvZipError;

/// This is a csv maker.
pub struct CsvMaker {
    pub(crate) writer: Writer<BufWriter<File>>,
    pub(crate) file_name: String,
    pub(crate) file_path: PathBuf,
}

impl CsvMaker {
    pub fn write<I>(&mut self, line: I) -> Result<(), CsvZipError>
    where
        I: IntoIterator,
        I::Item: AsRef<[u8]>,
    {
        self.writer.write_record(line).map_err(|e| e.into())
    }

    pub(crate) fn flush(&mut self) -> Result<(), CsvZipError> {
        self.writer.flush().map_err(|e| e.into())
    }
}
