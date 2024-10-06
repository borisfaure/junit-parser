//! Tests for invalid XML

use junit_parser;
use junit_parser::Error;
use std::io::Cursor;

#[test]
/// Test invalid xml due to malformed attribute
fn test_error_xml() {
    let xml = r#"<testsuites skipped"1" />"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_err());
    let err = r.err().unwrap();
    assert!(matches!(err, Error::XMLError(_)));
}

#[test]
/// Test invalid xml due to unclosed tag
fn test_error_xml_end_mismatch() {
    let xml = r#"<testsuites> <foo> </testsuites>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_err());
    let err = r.err().unwrap();
    assert!(matches!(err, Error::XMLError(_)));
}

#[test]
/// Test invalid xml due to end tag with no start tag
fn test_error_xml_end_no_start() {
    let xml = r#"<testsuites> </bar> </testsuites>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_err());
    let err = r.err().unwrap();
    assert!(matches!(err, Error::XMLError(_)));
}

#[test]
/// Test failing to parse due to invalid int
fn test_error_parseint() {
    let xml = r#"<testsuites skipped="foo" />"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_err());
    let err = r.err().unwrap();
    assert!(matches!(err, Error::ParseIntError(_)));
}

#[test]
/// Test failing to parse due to invalid float
fn test_error_parsefloat() {
    let xml = r#"<testsuites time="foo" />"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_err());
    let err = r.err().unwrap();
    assert!(matches!(err, Error::ParseFloatError(_)));
}
