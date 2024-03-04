# csv-zip-maker-rs

CSV and ZIP maker

[Documentation](https://docs.rs/csv-zip-maker)

## Features
- CSV and Zip file make in temp dir.
- Cleanup files at drop.
- Use file instead of memory.

## Changes
[CHANGELOG.md](https://github.com/aoyagikouhei/csv-zip-maker-rs/blob/main/CHANGELOG.md)

## Examples
```rust
use csv_zip_maker::{CsvExcelCustomizer, CsvZipError, CsvZipMaker};

fn main() -> Result<(), CsvZipError> {
    let mut maker = CsvZipMaker::new("test", "summary")?;
    let mut csv_maker = maker.make_csv_maker_for_excel("summary1")?;
    csv_maker.write(&vec!["aaa", "bbb"])?;
    csv_maker.write(&vec!["ccc", "ddd"])?;
    maker.add_csv(&mut csv_maker)?;
    let mut csv_maker = maker.make_csv_maker("summary2")?;
    csv_maker.write(&vec!["111", "222"])?;
    csv_maker.write(&vec!["333", "444"])?;
    maker.add_csv(&mut csv_maker)?;

    // UTF16
    let mut csv_maker = maker.make_csv_maker_with_customizer("summary3", CsvUtf16Customizer)?;
    csv_maker.write(&vec!["予定表～①\n💖ﾊﾝｶｸだ", "予定表～②💖ﾊﾝｶｸだ"])?;
    csv_maker.write(&vec!["予定表～③💖ﾊﾝｶｸだ", "予定表～④💖ﾊﾝｶｸだ"])?;
    
    maker.add_csv_utf16(&mut csv_maker)?;

    let path_buf = maker.make_zip_file()?;
    std::fs::copy(path_buf, "test.zip")?;
    Ok(())
}
```