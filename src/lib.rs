pub use crate::csv_zip_maker::CsvZipMaker;
pub use customize::CsvCustomizer;
pub use customize::CsvExcelCustomizer;
pub use error::CsvZipError;

pub mod csv_maker;
pub mod csv_zip_maker;
pub mod customize;
pub mod error;
pub use csv;

#[cfg(test)]
mod tests {
    use crate::{customize::CsvExcelUtf16Customizer, CsvZipError, CsvZipMaker};

    #[test]
    fn it_works() -> Result<(), CsvZipError> {
        let mut maker = CsvZipMaker::new("test", "summary")?;
        let mut csv_maker = maker.make_csv_maker_for_excel("summary1")?;
        csv_maker.write(&vec!["aaa", "bbb"])?;
        csv_maker.write(&vec!["ccc", "ddd"])?;
        maker.add_csv(&mut csv_maker)?;

        let mut csv_maker = maker.make_csv_maker("summary2")?;
        csv_maker.write(&vec!["111", "222"])?;
        csv_maker.write(&vec!["333", "444"])?;
        maker.add_csv(&mut csv_maker)?;

        let mut csv_maker =
            maker.make_csv_maker_with_customizer("summary3", CsvExcelUtf16Customizer)?;
        csv_maker.write(&vec!["予定表～①\n💖ﾊﾝｶｸだ", "予定表～②💖ﾊﾝｶｸだ"])?;
        csv_maker.write(&vec!["予定表～③💖ﾊﾝｶｸだ", "予定表～④💖ﾊﾝｶｸだ"])?;
        maker.add_csv_utf16(&mut csv_maker)?;

        let path_buf = maker.make_zip_file()?;
        std::fs::copy(path_buf, "test.zip")?;

        //assert_eq!(263, maker.make_zip_binary()?.len());

        Ok(())
    }
}
