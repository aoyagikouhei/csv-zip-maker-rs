use thiserror::Error;

/// This is a csv zip maker error.
#[derive(Error, Debug)]
pub enum CsvZipError {
    #[error("io error {0}")]
    Io(#[from] std::io::Error),

    #[error("csv error {0}")]
    Csv(#[from] csv::Error),

    #[error("zip error {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("Utf16 {0}")]
    Utf16(String),
}
