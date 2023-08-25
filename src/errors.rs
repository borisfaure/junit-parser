use thiserror::Error;

/// Error enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum Error {
    /// Error while parsing XML
    #[error("Error while parsing XML")]
    XMLError(#[from] ::quick_xml::Error),
    /// Error while converting f64 attribute
    #[error("Error while converting f64 attribute")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    /// Error while converting u64 attribute
    #[error("Error while converting u64 attribute")]
    ParseIntError(#[from] std::num::ParseIntError),
    /// Error while converting bytes to Utf8
    #[error("Error while converting bytes to Utf8")]
    ParseUt8Error(#[from] std::str::Utf8Error),
}
