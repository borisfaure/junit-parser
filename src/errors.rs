/// JunitParserError enumerates all possible errors returned by this library.
#[derive(Debug)]
pub enum JunitParserError {
    /// Error while parsing XML
    XMLError(::quick_xml::Error),
    /// Error while converting f64 attribute
    ParseFloatError(std::num::ParseFloatError),
    /// Error while converting u64 attribute
    ParseIntError(std::num::ParseIntError),
    /// Duplicate test suite / test case
    DuplicateError { kind: String, name: String },
}

impl std::error::Error for JunitParserError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            JunitParserError::XMLError(e) => Some(e),
            JunitParserError::ParseFloatError(e) => Some(e),
            JunitParserError::ParseIntError(e) => Some(e),
            _ => None,
        }
    }
}

impl std::fmt::Display for JunitParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            JunitParserError::XMLError(err) => write!(f, "XML error: {}", err),
            JunitParserError::ParseFloatError(err) => write!(f, "ParseFloat error: {}", err),
            JunitParserError::ParseIntError(err) => write!(f, "ParseInt error: {}", err),
            JunitParserError::DuplicateError { kind, name } => {
                write!(f, "Duplicate {} named {}", kind, name)
            }
        }
    }
}

impl From<::std::num::ParseFloatError> for JunitParserError {
    #[inline]
    fn from(error: ::std::num::ParseFloatError) -> JunitParserError {
        JunitParserError::ParseFloatError(error)
    }
}

impl From<::std::num::ParseIntError> for JunitParserError {
    #[inline]
    fn from(error: ::std::num::ParseIntError) -> JunitParserError {
        JunitParserError::ParseIntError(error)
    }
}

impl From<::std::str::Utf8Error> for JunitParserError {
    #[inline]
    fn from(error: ::std::str::Utf8Error) -> JunitParserError {
        JunitParserError::XMLError(::quick_xml::Error::Utf8(error))
    }
}

impl From<::quick_xml::Error> for JunitParserError {
    #[inline]
    fn from(err: ::quick_xml::Error) -> JunitParserError {
        JunitParserError::XMLError(err)
    }
}
