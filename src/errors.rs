#![warn(missing_docs)]
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
    /// Error parsing the `property` element: missing `name`
    #[error("Missing `name` attribute in property")]
    MissingPropertyName,
    /// Error while parsing: unexpected end of file
    #[error("Unexpected end of XML while parsing a {0} element")]
    UnexpectedEndOfFile(String),
}

impl From<::quick_xml::events::attributes::AttrError> for Error {
    #[inline]
    /// Convert [`::quick_xml::events::attributes`] into [`Error::XMLError`]
    fn from(err: ::quick_xml::events::attributes::AttrError) -> Error {
        Error::XMLError(err.into())
    }
}

impl From<::quick_xml::escape::EscapeError> for Error {
    #[inline]
    /// Convert [`::quick_xml::escape::EscapeError`] into [`Error::XMLError`]
    fn from(err: ::quick_xml::escape::EscapeError) -> Error {
        Error::XMLError(err.into())
    }
}
