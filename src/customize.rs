use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::CsvZipError;

// For Excel CSV
const BYTE_ORDER_MARK: &[u8; 3] = b"\xEF\xBB\xBF";

/// This is a csv file and csv format customizer.
pub trait CsvCustomizer {
    fn customize(
        &self,
        buf_writer: &mut BufWriter<File>,
        writer_builder: &mut csv::WriterBuilder,
    ) -> Result<(), CsvZipError>;
}

pub struct CsvExcelCustomizer;

impl CsvCustomizer for CsvExcelCustomizer {
    fn customize(
        &self,
        buf_writer: &mut BufWriter<File>,
        writer_builder: &mut csv::WriterBuilder,
    ) -> Result<(), CsvZipError> {
        buf_writer.write_all(BYTE_ORDER_MARK)?;
        writer_builder.terminator(csv::Terminator::CRLF);
        Ok(())
    }
}

impl CsvCustomizer for () {
    fn customize(
        &self,
        _buf_writer: &mut BufWriter<File>,
        _writer_builder: &mut csv::WriterBuilder,
    ) -> Result<(), CsvZipError> {
        Ok(())
    }
}
