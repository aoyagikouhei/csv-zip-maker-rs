pub use csv_zip_maker::CsvZipMaker;
pub use customize::CsvCustomizer;
pub use error::CsvZipError;

pub mod csv_maker;
pub mod csv_zip_maker;
pub mod customize;
pub mod error;

pub use csv;

#[cfg(test)]
mod tests {
    use crate::{customize::CsvExcelCustomizer, CsvZipError, CsvZipMaker};

    #[test]
    fn it_works() -> Result<(), CsvZipError> {
        let mut maker = CsvZipMaker::new("test", "summary")?;
        let mut csv_maker = maker.make_csv_maker("summary1", Some(Box::new(CsvExcelCustomizer)))?;
        csv_maker.write(&vec!["aaa", "bbb"])?;
        csv_maker.write(&vec!["ccc", "ddd"])?;
        maker.add_csv(&mut csv_maker)?;
        let mut csv_maker = maker.make_csv_maker("summary2", None)?;
        csv_maker.write(&vec!["111", "222"])?;
        csv_maker.write(&vec!["333", "444"])?;
        maker.add_csv(&mut csv_maker)?;

        //let path_buf = maker.make_zip_file()?;
        //std::fs::copy(path_buf, "test.zip")?;

        assert_eq!(263, maker.make_zip_binary()?.len());

        Ok(())
    }
}
