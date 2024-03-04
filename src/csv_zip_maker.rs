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

    fn execute_csv(&mut self, file_name: &str, file_path: &PathBuf) -> Result<(), CsvZipError> {
        self.writer.start_file(file_name, Default::default())?;
        let mut f = BufReader::new(File::open(file_path)?);
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

    pub fn add_csv(&mut self, csv_maker: &mut CsvMaker) -> Result<(), CsvZipError> {
        csv_maker.flush()?;
        self.execute_csv(&csv_maker.file_name, &csv_maker.file_path)
    }

    pub fn add_csv_utf16(&mut self, csv_maker: &mut CsvMaker) -> Result<(), CsvZipError> {
        csv_maker.flush()?;
        let reader_file_path = match csv_maker.file_path.to_str() {
            Some(res) => res,
            None => return Err(CsvZipError::Utf16("file_path.to_str".to_owned())),
        };
        let writer_file_path = self.tempdir.path().join("utf16.csv");
        let mut reader = BufReader::new(File::open(reader_file_path)?);
        let mut writer = BufWriter::new(File::create(writer_file_path.clone())?);
        writer.write_all(b"\xFF\xFE")?;
        let mut buf = [0; 1];
        let mut buffer: Vec<u8> = Vec::new();
        let mut cr_flag = false;
        loop {
            match reader.read(&mut buf)? {
                0 => break,
                _n => {
                    buffer.push(buf[0]);
                    if cr_flag {
                        if buf[0] == b'\n' {
                            // 完了
                            let src = match String::from_utf8(buffer) {
                                Ok(res) => res,
                                Err(e) => return Err(CsvZipError::Utf16(e.to_string())),
                            };
                            buffer = Vec::new();
                            let dst: Vec<u8> =
                                src.encode_utf16().flat_map(|it| it.to_le_bytes()).collect();
                            writer.write_all(&dst)?;
                            cr_flag = false;
                        } else {
                            cr_flag = false;
                        }
                    } else if buf[0] == b'\r' {
                        cr_flag = true;
                    }
                }
            }
        }
        writer.flush()?;

        self.execute_csv(&csv_maker.file_name, &writer_file_path)
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
