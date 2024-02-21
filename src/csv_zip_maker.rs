use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
};

use csv::WriterBuilder;
use tempfile::TempDir;
use zip::ZipWriter;

use crate::{csv_maker::CsvMaker, CsvCustomizer, CsvExcelCustomizer, CsvZipError};

pub struct CsvZipMaker {
    tempdir: TempDir,
    writer: ZipWriter<BufWriter<File>>,
    file_path: PathBuf,
}

impl CsvZipMaker {
    pub fn new(prefix: &str, name: &str) -> Result<Self, CsvZipError> {
        let tempdir = TempDir::with_prefix(prefix)?;
        let file_path = tempdir.path().join(format!("{}.zip", name));
        let buf_writer = BufWriter::new(File::create(&file_path)?);
        let writer = ZipWriter::new(buf_writer);
        Ok(Self {
            tempdir,
            writer,
            file_path,
        })
    }

    pub fn make_csv_maker(&self, name: &str) -> Result<CsvMaker, CsvZipError> {
        self.make_csv_maker_with_customizer(name, ())
    }

    pub fn make_csv_maker_for_excel(&self, name: &str) -> Result<CsvMaker, CsvZipError> {
        self.make_csv_maker_with_customizer(name, CsvExcelCustomizer)
    }

    pub fn make_csv_maker_with_customizer(
        &self,
        name: &str,
        customizer: impl CsvCustomizer,
    ) -> Result<CsvMaker, CsvZipError> {
        let file_name = format!("{}.csv", name);
        let file_path = self.tempdir.path().join(&file_name);
        let mut buf_writer = BufWriter::new(File::create(&file_path)?);
        let mut writer_builder = WriterBuilder::new();

        customizer.customize(&mut buf_writer, &mut writer_builder)?;

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
        let mut f = BufReader::new(File::open(&csv_maker.file_path)?);
        let mut buf = [0; 1024];
        loop {
            match f.read(&mut buf)? {
                0 => break,
                n => {
                    self.writer.write_all(&buf[0..n])?;
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
