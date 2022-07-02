use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
};

use csv::Writer;
use tempdir::TempDir;
use thiserror::Error;
use zip::ZipWriter;

// For Excel CSV
pub const BYTE_ORDER_MARK: &[u8; 3] = b"\xEF\xBB\xBF";

#[derive(Error, Debug)]
pub enum CsvZipError {
    #[error("io error {0}")]
    Io(#[from] std::io::Error),

    #[error("csv error {0}")]
    Csv(#[from] csv::Error),

    #[error("zip error {0}")]
    Zip(#[from] zip::result::ZipError),
}

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

pub struct CsvMaker {
    writer: Writer<BufWriter<File>>,
    pub file_name: String,
    pub file_path: PathBuf,
}

impl CsvMaker {
    pub fn write(&mut self, line: &Vec<&str>) -> Result<(), CsvZipError> {
        self.writer.write_record(line).map_err(|e| e.into())
    }

    fn flush(&mut self) -> Result<(), CsvZipError> {
        self.writer.flush().map_err(|e| e.into())
    }
}

pub struct CsvZipMaker {
    tempdir: TempDir,
    writer: ZipWriter<BufWriter<File>>,
    file_path: PathBuf,
}

impl CsvZipMaker {
    pub fn new(prefix: &str, name: &str) -> Result<Self, CsvZipError> {
        let tempdir = TempDir::new(prefix)?;
        let file_path = tempdir.path().join(&format!("{}.zip", name));
        let buf_writer = BufWriter::new(File::create(file_path.clone())?);
        let writer = ZipWriter::new(buf_writer);
        Ok(Self {
            tempdir,
            writer,
            file_path,
        })
    }

    pub fn make_csv_maker(
        &self,
        name: &str,
        customizer: Option<CsvExcelCustomizer>,
    ) -> Result<CsvMaker, CsvZipError> {
        let file_name = format!("{}.csv", name);
        let file_path = self.tempdir.path().join(&file_name);
        let mut buf_writer = BufWriter::new(File::create(file_path.clone())?);
        let mut writer_builder = csv::WriterBuilder::new();

        if let Some(customizer) = customizer {
            customizer.customize(&mut buf_writer, &mut writer_builder)?;
        }

        Ok(CsvMaker {
            writer: writer_builder.from_writer(buf_writer),
            file_name,
            file_path,
        })
    }

    pub fn add_csv(&mut self, csv_maker: &mut CsvMaker) -> Result<(), CsvZipError> {
        csv_maker.flush()?;

        self.writer
            .start_file(&csv_maker.file_name, Default::default())?;
        let mut f = BufReader::new(File::open(csv_maker.file_path.clone())?);
        let mut buf = [0; 1024];
        loop {
            match f.read(&mut buf)? {
                0 => break,
                n => {
                    self.writer.write(&buf[0..n])?;
                }
            }
        }
        Ok(())
    }

    pub fn make_zip_file(&mut self) -> Result<&PathBuf, CsvZipError> {
        let mut res = self.writer.finish()?;
        res.flush()?;
        Ok(&self.file_path)
    }

    pub fn make_zip_binary(&mut self) -> Result<Vec<u8>, CsvZipError> {
        let mut res = self.writer.finish()?;
        res.flush()?;
        let mut file = File::open(&self.file_path)?;
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf)?;
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use crate::{CsvExcelCustomizer, CsvZipError, CsvZipMaker};

    #[test]
    fn it_works() -> Result<(), CsvZipError> {
        let mut maker = CsvZipMaker::new("test", "summary")?;
        let mut csv_maker = maker.make_csv_maker("summary1", Some(CsvExcelCustomizer))?;
        csv_maker.write(&vec!["aaa", "bbb"])?;
        csv_maker.write(&vec!["ccc", "ddd"])?;
        maker.add_csv(&mut csv_maker)?;
        let mut csv_maker = maker.make_csv_maker("summary2", None)?;
        csv_maker.write(&vec!["111", "222"])?;
        csv_maker.write(&vec!["333", "444"])?;
        maker.add_csv(&mut csv_maker)?;

        //let path_buf = maker.make_zip_file()?;
        //std::fs::copy(path_buf, "test.zip")?;

        println!("{:?}", maker.make_zip_binary()?.len());

        Ok(())
    }
}
