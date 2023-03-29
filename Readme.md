# csv-zip-maker-rs

CSV and ZIP maker

[Documentation](https://docs.rs/csv-zip-maker)

## Features
- CSV and Zip file make in temp dir.
- Cleanup files at drop.
- Use file instead of memory.

## Changes

### v0.2.2 (2023/03/29)
* change CsvMaker#write signature

### v0.2.1 (2022/08/03)
* modify Readme sample
* change ambiguous error
* change write to write_all

### v0.2.0 (2022/08/03)
* hide customizer(breaking change)

### v0.1.2 (2022/07/04)
* fix Readme document path

### v0.1.1 (2022/07/02)
* fix Readme
* fix Cargo.toml categories

### v0.1.0 (2022/07/02)
* first release

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

    let path_buf = maker.make_zip_file()?;
    std::fs::copy(path_buf, "test.zip")?;
    Ok(())
}
```