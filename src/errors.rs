/// Error enumerates all possible errors returned by this library.
#[derive(Debug)]
pub enum Error {
    /// Error while parsing XML
    XMLError(::quick_xml::Error),
    /// Error while converting f64 attribute
    ParseFloatError(std::num::ParseFloatError),
    /// Error while converting u64 attribute
    ParseIntError(std::num::ParseIntError),
    /// Duplicate test suite / test case
    DuplicateError { kind: String, name: String },
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::XMLError(e) => Some(e),
            Error::ParseFloatError(e) => Some(e),
            Error::ParseIntError(e) => Some(e),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::XMLError(err) => write!(f, "XML error: {}", err),
            Error::ParseFloatError(err) => write!(f, "ParseFloat error: {}", err),
            Error::ParseIntError(err) => write!(f, "ParseInt error: {}", err),
            Error::DuplicateError { kind, name } => write!(f, "Duplicate {} named {}", kind, name),
        }
    }
}

impl From<::std::num::ParseFloatError> for Error {
    #[inline]
    fn from(error: ::std::num::ParseFloatError) -> Error {
        Error::ParseFloatError(error)
    }
}

impl From<::std::num::ParseIntError> for Error {
    #[inline]
    fn from(error: ::std::num::ParseIntError) -> Error {
        Error::ParseIntError(error)
    }
}

impl From<::std::str::Utf8Error> for Error {
    #[inline]
    fn from(error: ::std::str::Utf8Error) -> Error {
        Error::XMLError(::quick_xml::Error::Utf8(error))
    }
}

impl From<::quick_xml::Error> for Error {
    #[inline]
    fn from(err: ::quick_xml::Error) -> Error {
        Error::XMLError(err)
    }
}
