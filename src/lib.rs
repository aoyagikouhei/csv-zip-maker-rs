pub use crate::csv_zip_maker::CsvZipMaker;
pub use customize::CsvCustomizer;
pub use customize::CsvExcelCustomizer;
pub use error::CsvZipError;

pub mod csv_maker;
pub mod csv_zip_maker;
pub mod customize;
pub mod error;
pub use csv;
pub use time;
pub use zip;

#[cfg(test)]
mod tests {
    use crate::{customize::CsvExcelUtf16Customizer, CsvZipError, CsvZipMaker};
    use time::{Duration, OffsetDateTime};
    use zip::{write::FileOptions, DateTime};

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

    #[test]
    fn it_works_with_timestamp_offset() -> Result<(), CsvZipError> {
        let offset = OffsetDateTime::now_utc() + Duration::hours(9);
        let option = FileOptions::default().last_modified_time(DateTime::try_from(offset).unwrap());
        let mut maker = CsvZipMaker::new_with_file_option("test", "summary", option)?;
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
        std::fs::copy(path_buf, "test_with_offset.zip")?;

        //assert_eq!(263, maker.make_zip_binary()?.len());

        Ok(())
    }
}
