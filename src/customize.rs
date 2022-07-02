use std::{io::{BufWriter, Write}, fs::File};

use crate::CsvZipError;

// For Excel CSV
const BYTE_ORDER_MARK: &[u8; 3] = b"\xEF\xBB\xBF";

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
        buf_writer.write(BYTE_ORDER_MARK)?;
        writer_builder.terminator(csv::Terminator::CRLF);
        Ok(())
    }
}