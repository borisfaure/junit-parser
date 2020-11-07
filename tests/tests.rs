use junit_parser;
use std::io::Cursor;

#[test]
fn skip_doctype() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?><testsuites/>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.time, 0f64);
    assert_eq!(t.tests, 0u64);
    assert_eq!(t.errors, 0u64);
    assert_eq!(t.failures, 0u64);
    assert_eq!(t.skipped, 0u64);
    assert_eq!(t.name, "");
    assert_eq!(t.suites.len(), 0);
}

#[test]
fn empty_test_suites() {
    let xml = r#"<testsuites/>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.time, 0f64);
    assert_eq!(t.tests, 0u64);
    assert_eq!(t.errors, 0u64);
    assert_eq!(t.failures, 0u64);
    assert_eq!(t.skipped, 0u64);
    assert_eq!(t.name, "");
    assert_eq!(t.suites.len(), 0);
}
#[test]
fn empty_test_suites_empty_attributes() {
    let xml = r#"<testsuites
        tests="" name="" time="" errors="" failures="" skipped="" />"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.time, 0f64);
    assert_eq!(t.tests, 0u64);
    assert_eq!(t.errors, 0u64);
    assert_eq!(t.failures, 0u64);
    assert_eq!(t.skipped, 0u64);
    assert_eq!(t.name, "");
    assert_eq!(t.suites.len(), 0);
}

#[test]
fn empty_test_suites_start_end() {
    let xml = r#"<testsuites></testsuites>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.time, 0f64);
    assert_eq!(t.tests, 0u64);
    assert_eq!(t.errors, 0u64);
    assert_eq!(t.failures, 0u64);
    assert_eq!(t.skipped, 0u64);
    assert_eq!(t.name, "");
    assert_eq!(t.suites.len(), 0);
}

#[test]
fn empty_test_suites_with_attributes() {
    let xml = r#"<testsuites name="AllTests"
            tests="22" time="38730.23"
            errors="5" failures="9" skipped="3"
            />"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.time, 38730.23f64);
    assert_eq!(t.tests, 22u64);
    assert_eq!(t.errors, 5u64);
    assert_eq!(t.failures, 9u64);
    assert_eq!(t.skipped, 3u64);
    assert_eq!(t.name, "AllTests");
    assert_eq!(t.suites.len(), 0);
}

#[test]
fn empty_test_suites_start_end_with_attributes() {
    let xml = r#"<testsuites name="AllTests"
            tests="22" time="38730.23"
            errors="5" failures="9" skipped="3"
            ></testsuites>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.time, 38730.23f64);
    assert_eq!(t.tests, 22u64);
    assert_eq!(t.errors, 5u64);
    assert_eq!(t.failures, 9u64);
    assert_eq!(t.skipped, 3u64);
    assert_eq!(t.name, "AllTests");
    assert_eq!(t.suites.len(), 0);
}
