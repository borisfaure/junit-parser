//! Test the JUnit parser
//!
//! Some tests may seem duplicate since they test start-end elements and
//! empty-element tags but the parser uses different codepaths

use junit_parser;
use junit_parser::Error;
use std::io::Cursor;

#[test]
/// Test that the report is parsed with a doctype
/// Following tests will not have one
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
/// Test with an empty-element `testsuites` tag
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
/// Test with an empty-element `testrun` tag
fn empty_test_run() {
    let xml = r#"<testrun/>"#;
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
/// Test a `testsuites` tag with empty attributes
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
/// Test a `testsuites` element with no content
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
/// Test a `testrun` element with no content
fn empty_testrun_start_end() {
    let xml = r#"<testrun></testrun>"#;
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
/// Test an empty-element `testsuites` tag with attributes
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
/// Test a `testsuites` element with attributes but no content
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

#[test]
/// Test a `testsuites` element with an empty-element `testsuite`
fn empty_test_suite() {
    let xml = r#"<testsuites>
    <testsuite/>
        </testsuites>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let t = &t.suites[0];
    assert_eq!(t.cases.len(), 0);
}

#[test]
/// Test a `testsuites` element with an empty-element `testsuite` with empty
/// attributes
fn empty_test_suite_empty_attributes() {
    let xml = r#"<testsuites><testsuite
        tests="" name="" time="" errors="" failures="" skipped="" />
        </testsuites>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let t = &t.suites[0];
    assert_eq!(t.time, 0f64);
    assert_eq!(t.tests, 0u64);
    assert_eq!(t.errors, 0u64);
    assert_eq!(t.failures, 0u64);
    assert_eq!(t.skipped, 0u64);
    assert_eq!(t.cases.len(), 0);
    assert_eq!(t.name, "");
}

#[test]
/// Test an empty-element `testsuite`
fn empty_test_suite_start_end() {
    let xml = r#"<testsuites>
        <testsuite></testsuite>
        </testsuites>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let t = &t.suites[0];
    assert_eq!(t.time, 0f64);
    assert_eq!(t.tests, 0u64);
    assert_eq!(t.errors, 0u64);
    assert_eq!(t.failures, 0u64);
    assert_eq!(t.skipped, 0u64);
    assert_eq!(t.name, "");
    assert_eq!(t.cases.len(), 0);
}

#[test]
/// Test an empty-element `testsuite` with attributes
fn empty_test_suite_with_attributes() {
    let xml = r#"<testsuites>
    <testsuite name="AllTests" tests="22" time="38730.23"
            errors="5" failures="9" skipped="3"
            /></testsuites>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let t = &t.suites[0];
    assert_eq!(t.time, 38730.23f64);
    assert_eq!(t.tests, 22u64);
    assert_eq!(t.errors, 5u64);
    assert_eq!(t.failures, 9u64);
    assert_eq!(t.skipped, 3u64);
    assert_eq!(t.name, "AllTests");
    assert_eq!(t.cases.len(), 0);
}

#[test]
/// Test a single element `testsuite` with attributes but no content
fn empty_test_suite_start_end_with_attributes() {
    let xml = r#"<testsuites>
    <testsuite name="AllTests"
            tests="22" time="38730.23"
            errors="5" failures="9" skipped="3"
            ></testsuite>
            </testsuites>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let t = &t.suites[0];
    assert_eq!(t.time, 38730.23f64);
    assert_eq!(t.tests, 22u64);
    assert_eq!(t.errors, 5u64);
    assert_eq!(t.failures, 9u64);
    assert_eq!(t.skipped, 3u64);
    assert_eq!(t.name, "AllTests");
    assert_eq!(t.cases.len(), 0);
}

#[test]
/// Test a single empty-element `testsuite`, not in a `testsuites` element
fn no_suites_empty_test_suite() {
    let xml = r#"<testsuite/>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let t = &t.suites[0];
    assert_eq!(t.cases.len(), 0);
}

#[test]
/// Test a single empty-element `testsuite` with attributes, not in a `testsuites` element
fn no_suites_empty_test_suite_empty_attributes() {
    let xml = r#"<testsuite
        tests="" name="" time="" errors="" failures="" skipped="" />"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let t = &t.suites[0];
    assert_eq!(t.time, 0f64);
    assert_eq!(t.tests, 0u64);
    assert_eq!(t.errors, 0u64);
    assert_eq!(t.failures, 0u64);
    assert_eq!(t.skipped, 0u64);
    assert_eq!(t.name, "");
    assert_eq!(t.cases.len(), 0);
}

#[test]
/// Test a single empty-element `testsuite`, not in a `testsuites` element
fn no_suites_empty_test_suites_start_end() {
    let xml = r#"<testsuite></testsuite>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let t = &t.suites[0];
    assert_eq!(t.time, 0f64);
    assert_eq!(t.tests, 0u64);
    assert_eq!(t.errors, 0u64);
    assert_eq!(t.failures, 0u64);
    assert_eq!(t.skipped, 0u64);
    assert_eq!(t.name, "");
    assert_eq!(t.cases.len(), 0);
}

#[test]
/// Test a single empty-element `testsuite`, not in a `testsuites` element,
/// with attributes
fn no_suites_empty_test_suites_with_attributes() {
    let xml = r#"<testsuite name="AllTests"
            tests="22" time="38730.23"
            errors="5" failures="9" skipped="3"
            />"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let t = &t.suites[0];
    assert_eq!(t.time, 38730.23f64);
    assert_eq!(t.tests, 22u64);
    assert_eq!(t.errors, 5u64);
    assert_eq!(t.failures, 9u64);
    assert_eq!(t.skipped, 3u64);
    assert_eq!(t.name, "AllTests");
    assert_eq!(t.cases.len(), 0);
}

#[test]
/// Test a single start-end element `testsuite`, not in a `testsuites` element,
/// with attributes
fn no_suites_empty_test_suites_start_end_with_attributes() {
    let xml = r#"<testsuite name="AllTests"
            tests="22" time="38730.23"
            errors="5" failures="9" skipped="3"
            ></testsuite>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let t = &t.suites[0];
    assert_eq!(t.time, 38730.23f64);
    assert_eq!(t.tests, 22u64);
    assert_eq!(t.errors, 5u64);
    assert_eq!(t.failures, 9u64);
    assert_eq!(t.skipped, 3u64);
    assert_eq!(t.name, "AllTests");
    assert_eq!(t.cases.len(), 0);
}

#[test]
/// Test a testcase in success
fn test_case_success() {
    let xml = r#"<testsuite name="foo" tests="1" time="30.23" >
    <testcase name="bar" time="12.34" />
            </testsuite>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let ts = &t.suites[0];
    assert_eq!(ts.cases.len(), 1);
    let tc = &ts.cases[0];
    assert_eq!(tc.time, 12.34f64);
    assert_eq!(tc.name, "bar");
    assert!(tc.status.is_success());
}

#[test]
/// Test a testcase in error with empty-element `error`
fn test_case_error_empty() {
    let xml = r#"<testsuite errors="1" name="foo" tests="1" time="30.23" >
    <testcase name="bar" time="12.34" >
    <error />
    </testcase>
            </testsuite>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let ts = &t.suites[0];
    assert_eq!(ts.cases.len(), 1);
    let tc = &ts.cases[0];
    assert_eq!(tc.time, 12.34f64);
    assert_eq!(tc.name, "bar");
    assert!(tc.status.is_error());
}

#[test]
/// Test a testcase in error with empty-element `error` with messages attribute
fn test_case_error_message() {
    let xml = r#"<testsuite errors="1" name="foo" tests="1" time="30.23" >
    <testcase name="bar" time="12.34" >
    <error type="error" message="exception raised" />
    </testcase>
            </testsuite>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let ts = &t.suites[0];
    assert_eq!(ts.cases.len(), 1);
    let tc = &ts.cases[0];
    assert_eq!(tc.time, 12.34f64);
    assert_eq!(tc.name, "bar");
    assert!(tc.status.is_error());
    let te = tc.status.error_as_ref();
    assert_eq!(te.error_type, "error");
    assert_eq!(te.message, "exception raised");
}

#[test]
/// Test a testcase in error with messages attribute and content
fn test_case_error_message_text() {
    let xml = r#"<testsuite errors="1" name="foo" tests="1" time="30.23" >
    <testcase name="bar" time="12.34" >
    <error type="error" message="exception raised" >
    foo::bar asserted!
    </error>
    </testcase>
            </testsuite>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let ts = &t.suites[0];
    assert_eq!(ts.cases.len(), 1);
    let tc = &ts.cases[0];
    assert_eq!(tc.time, 12.34f64);
    assert_eq!(tc.name, "bar");
    assert!(tc.status.is_error());
    let te = tc.status.error_as_ref();
    assert_eq!(te.error_type, "error");
    assert_eq!(te.message, "exception raised");
    assert_eq!(te.text, "foo::bar asserted!");
}

#[test]
/// Test a testcase in failure with empty-element `failure`
fn test_case_failure_empty() {
    let xml = r#"<testsuite failures="1" name="foo" tests="1" time="30.23" >
    <testcase name="bar" time="12.34" >
    <failure />
    </testcase>
            </testsuite>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let ts = &t.suites[0];
    assert_eq!(ts.cases.len(), 1);
    let tc = &ts.cases[0];
    assert_eq!(tc.time, 12.34f64);
    assert_eq!(tc.name, "bar");
    assert!(tc.status.is_failure());
}

#[test]
/// Test a testcase in failure with empty-element `failure` with messages attribute
fn test_case_failure_message() {
    let xml = r#"<testsuite failures="1" name="foo" tests="1" time="30.23" >
    <testcase name="bar" time="12.34" >
    <failure type="failure" message="test failed" />
    </testcase>
            </testsuite>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let ts = &t.suites[0];
    assert_eq!(ts.cases.len(), 1);
    let tc = &ts.cases[0];
    assert_eq!(tc.time, 12.34f64);
    assert_eq!(tc.name, "bar");
    assert!(tc.status.is_failure());
    let te = tc.status.failure_as_ref();
    assert_eq!(te.failure_type, "failure");
    assert_eq!(te.message, "test failed");
}

#[test]
/// Test a testcase in failure with messages attribute and content
fn test_case_failure_message_text() {
    let xml = r#"<testsuite failures="1" name="foo" tests="1" time="30.23" >
    <testcase name="bar" time="12.34" >
    <failure type="failure" message="test failed" >
    foo::bar failed!
    </failure>
    </testcase>
            </testsuite>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let ts = &t.suites[0];
    assert_eq!(ts.cases.len(), 1);
    let tc = &ts.cases[0];
    assert_eq!(tc.time, 12.34f64);
    assert_eq!(tc.name, "bar");
    assert!(tc.status.is_failure());
    let te = tc.status.failure_as_ref();
    assert_eq!(te.failure_type, "failure");
    assert_eq!(te.message, "test failed");
    assert_eq!(te.text, "foo::bar failed!");
}

#[test]
/// Test a skipped testcase with empty-element `skipped` with messages attribute
fn test_case_skipped_empty() {
    let xml = r#"<testsuite skipped="1" name="foo" tests="1" time="30.23" >
    <testcase name="bar" time="12.34" >
    <skipped />
    </testcase>
            </testsuite>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    debug_assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let ts = &t.suites[0];
    assert_eq!(ts.cases.len(), 1);
    let tc = &ts.cases[0];
    assert_eq!(tc.time, 12.34f64);
    assert_eq!(tc.name, "bar");
    assert!(tc.status.is_skipped());
}

#[test]
/// Test a skipped testcase with empty-element `skipped` with messages attribute
fn test_case_skipped_message() {
    let xml = r#"<testsuite skipped="1" name="foo" tests="1" time="30.23" >
    <testcase name="bar" time="12.34" >
    <skipped type="skipped" message="test skipped" />
    </testcase>
            </testsuite>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let ts = &t.suites[0];
    assert_eq!(ts.cases.len(), 1);
    let tc = &ts.cases[0];
    assert_eq!(tc.time, 12.34f64);
    assert_eq!(tc.name, "bar");
    assert!(tc.status.is_skipped());
    let te = tc.status.skipped_as_ref();
    assert_eq!(te.skipped_type, "skipped");
    assert_eq!(te.message, "test skipped");
}

#[test]
/// Test a skipped testcase with messages attribute and content
fn test_case_skipped_message_text() {
    let xml = r#"<testsuite skipped="1" name="foo" tests="1" time="30.23" >
    <testcase name="bar" time="12.34" >
    <skipped type="skipped" message="test skipped" >
    foo::bar skipped for some reason
    </skipped>
    </testcase>
            </testsuite>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let ts = &t.suites[0];
    assert_eq!(ts.cases.len(), 1);
    let tc = &ts.cases[0];
    assert_eq!(tc.time, 12.34f64);
    assert_eq!(tc.name, "bar");
    assert!(tc.status.is_skipped());
    let te = tc.status.skipped_as_ref();
    assert_eq!(te.skipped_type, "skipped");
    assert_eq!(te.message, "test skipped");
    assert_eq!(te.text, "foo::bar skipped for some reason");
}

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

#[test]
/// Duplicate test suites are all stored
fn test_duplicate_suites() {
    let xml = r#"<testsuites>
        <testsuite name="foo" errors="1" />
        <testsuite name="foo" errors="2" />
        </testsuites>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let tss = r.unwrap();
    assert_eq!(tss.suites.len(), 2);
    let ts = &tss.suites[0];
    assert_eq!(ts.name, "foo");
    assert_eq!(ts.errors, 1);
    let ts = &tss.suites[1];
    assert_eq!(ts.name, "foo");
    assert_eq!(ts.errors, 2);
}

#[test]
/// Duplicate test cases are all stored
fn test_duplicate_cases() {
    let xml = r#"<testsuite>
        <testcase name="foo" />
        <testcase name="foo" ><error/></testcase>
        <testcase name="foo" ><failure/></testcase>
        <testcase name="foo" ><skipped/></testcase>
        </testsuite>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let tss = r.unwrap();
    assert_eq!(tss.suites.len(), 1);
    let ts = &tss.suites[0];
    assert_eq!(ts.cases.len(), 4);
    let tc = &ts.cases[0];
    assert_eq!(tc.name, "foo");
    assert!(tc.status.is_success());
    let tc = &ts.cases[1];
    assert_eq!(tc.name, "foo");
    assert!(tc.status.is_error());
    let tc = &ts.cases[2];
    assert_eq!(tc.name, "foo");
    assert!(tc.status.is_failure());
    let tc = &ts.cases[3];
    assert_eq!(tc.name, "foo");
    assert!(tc.status.is_skipped());
}

#[test]
/// Test optional TestSuite attributes
fn test_optional_test_suite_attributes() {
    let xml = r#"
<testsuite
  assertions="42"
  timestamp="2023-09-14T23:43:28+02:00"
  hostname="mycomputer.local"
  id="TestSuiteId"
  package="TestPackage"
  file="test.rs"
  log="mylog"
  url="test://my.test/"
  version="3.1459"
  >
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let tss = r.unwrap();
    assert_eq!(tss.suites.len(), 1);
    let ts = &tss.suites[0];
    assert_eq!(ts.assertions, Some(42));
    assert_eq!(ts.timestamp, Some("2023-09-14T23:43:28+02:00".to_string()));
    assert_eq!(ts.hostname, Some("mycomputer.local".to_string()));
    assert_eq!(ts.id, Some("TestSuiteId".to_string()));
    assert_eq!(ts.package, Some("TestPackage".to_string()));
    assert_eq!(ts.file, Some("test.rs".to_string()));
    assert_eq!(ts.log, Some("mylog".to_string()));
    assert_eq!(ts.url, Some("test://my.test/".to_string()));
    assert_eq!(ts.version, Some("3.1459".to_string()));
}

#[test]
/// Test with multiple test cases
/// Also test parsing TestCase's attributes `classname`, `group`,
/// `file` and `line`
fn test_large_test_suite() {
    let xml = r#"
<testsuite tests="3" failures="1">
  <testcase classname="foo1" group="gr1" name="ASuccessfulTest"/>
  <testcase group="gr2" name="AnotherSuccessfulTest"/>
  <testcase classname="foo3" name="AFailingTest"
    file="foo.rs" line="63">
    <failure type="NotEnoughFoo"> details about failure </failure>
  </testcase>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let tss = r.unwrap();
    assert_eq!(tss.suites.len(), 1);
    let ts = &tss.suites[0];
    assert_eq!(ts.cases.len(), 3);
    let tc = &ts.cases[0];
    assert_eq!(tc.original_name, "ASuccessfulTest");
    assert_eq!(tc.classname, Some("foo1".to_string()));
    assert_eq!(tc.name, "foo1::ASuccessfulTest");
    assert_eq!(tc.group, Some("gr1".to_string()));
    assert!(tc.status.is_success());
    let tc = &ts.cases[1];
    assert_eq!(tc.original_name, "AnotherSuccessfulTest");
    assert_eq!(tc.classname, None);
    assert_eq!(tc.name, "gr2::AnotherSuccessfulTest");
    assert_eq!(tc.group, Some("gr2".to_string()));
    assert!(tc.status.is_success());
    let tc = &ts.cases[2];
    assert_eq!(tc.original_name, "AFailingTest");
    assert_eq!(tc.classname, Some("foo3".to_string()));
    assert_eq!(tc.name, "foo3::AFailingTest");
    assert_eq!(tc.group, None);
    assert_eq!(tc.file, Some("foo.rs".to_string()));
    assert_eq!(tc.line, Some(63));
    assert!(tc.status.is_failure());
    let tf = tc.status.failure_as_ref();
    assert_eq!(tf.failure_type, "NotEnoughFoo");
    assert_eq!(tf.message, "");
    assert_eq!(tf.text, "details about failure");
}

#[test]
/// Test with multiple `testsuite`
fn test_large_test_suites() {
    let xml = r#"<testsuites tests="6" failures="2">
<testsuite name="foo" tests="3" failures="1">
  <testcase classname="foo1" name="ASuccessfulTest"/>
  <testcase classname="foo2" name="AnotherSuccessfulTest"/>
  <testcase classname="foo3" name="AFailingTest">
    <failure type="NotEnoughFoo"> details about failure </failure>
  </testcase>
</testsuite>
<testsuite name="bar" tests="3" failures="1">
  <testcase classname="bar1" name="ASuccessfulTest"/>
  <testcase classname="bar2" name="AnotherSuccessfulTest"/>
  <testcase classname="bar3" name="AFailingTest">
    <failure type="NotEnoughBar"> details about failure </failure>
  </testcase>
</testsuite>
</testsuites>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let tss = r.unwrap();
    assert_eq!(tss.suites.len(), 2);
    let ts = &tss.suites[0];
    assert_eq!(ts.cases.len(), 3);
    let tc = &ts.cases[0];
    assert_eq!(tc.original_name, "ASuccessfulTest");
    assert!(tc.status.is_success());
    let tc = &ts.cases[1];
    assert_eq!(tc.original_name, "AnotherSuccessfulTest");
    assert!(tc.status.is_success());
    let tc = &ts.cases[2];
    assert_eq!(tc.original_name, "AFailingTest");
    assert!(tc.status.is_failure());
    let tf = tc.status.failure_as_ref();
    assert_eq!(tf.failure_type, "NotEnoughFoo");
    assert_eq!(tf.message, "");
    assert_eq!(tf.text, "details about failure");

    let ts = &tss.suites[1];
    assert_eq!(ts.cases.len(), 3);
    let tc = &ts.cases[0];
    assert_eq!(tc.original_name, "ASuccessfulTest");
    assert!(tc.status.is_success());
    let tc = &ts.cases[1];
    assert_eq!(tc.original_name, "AnotherSuccessfulTest");
    assert!(tc.status.is_success());
    let tc = &ts.cases[2];
    assert_eq!(tc.original_name, "AFailingTest");
    assert!(tc.status.is_failure());
    let tf = tc.status.failure_as_ref();
    assert_eq!(tf.failure_type, "NotEnoughBar");
    assert_eq!(tf.message, "");
    assert_eq!(tf.text, "details about failure");
}

#[test]
/// Test that unknown tags are skipped
fn test_large_test_suites_added_tags() {
    let xml = r#"<testsuites tests="6" failures="2">
    <script />
<testsuite name="foo" tests="3" failures="1">
  <script />
  <testcase classname="foo1" name="ASuccessfulTest"/>
  <script />
  <testcase classname="foo2" name="AnotherSuccessfulTest"/>
  <script />
  <testcase classname="foo3" name="AFailingTest">
    <script />
    <failure type="NotEnoughFoo">
      details about failure
    </failure>
    <script />
  </testcase>
  <script />
  <testcase classname="foo4" name="ATestOnError">
    <script />
    <error type="Setup">
      setup failure
    </error>
    <script />
  </testcase>
  <script />
  <testcase classname="foo5" name="ASkippedTest">
    <script />
    <skipped type="skip">
      details about being skipped
    </skipped>
    <script />
  </testcase>
</testsuite>
</testsuites>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let tss = r.unwrap();
    assert_eq!(tss.suites.len(), 1);
    let ts = &tss.suites[0];
    assert_eq!(ts.cases.len(), 5);

    let tc = &ts.cases[0];
    assert_eq!(tc.original_name, "ASuccessfulTest");
    assert!(tc.status.is_success());

    let tc = &ts.cases[1];
    assert_eq!(tc.original_name, "AnotherSuccessfulTest");
    assert!(tc.status.is_success());

    let tc = &ts.cases[2];
    assert_eq!(tc.original_name, "AFailingTest");
    assert!(tc.status.is_failure());
    let tf = tc.status.failure_as_ref();
    assert_eq!(tf.failure_type, "NotEnoughFoo");
    assert_eq!(tf.message, "");
    println!("{:?}", tf.text);
    assert_eq!(tf.text, "details about failure");

    let tc = &ts.cases[3];
    assert_eq!(tc.original_name, "ATestOnError");
    assert!(tc.status.is_error());
    let tf = tc.status.error_as_ref();
    assert_eq!(tf.error_type, "Setup");
    assert_eq!(tf.message, "");
    assert_eq!(tf.text, "setup failure");

    let tc = &ts.cases[4];
    assert_eq!(tc.original_name, "ASkippedTest");
    assert!(tc.status.is_skipped());
    let tf = tc.status.skipped_as_ref();
    assert_eq!(tf.skipped_type, "skip");
    assert_eq!(tf.message, "");
    assert_eq!(tf.text, "details about being skipped");
}

#[test]
/// Test that comments are skipped
fn test_large_test_suites_with_comments() {
    let xml = r#"<testsuites tests="6" failures="2">
    <!-- -->
<testsuite name="foo" tests="3" failures="1">
  <!-- -->
  <testcase classname="foo1" name="ASuccessfulTest"/>
  <!-- -->
  <testcase classname="foo2" name="AnotherSuccessfulTest"/>
  <!-- -->
  <testcase classname="foo3" name="AFailingTest">
    <!-- -->
    <failure type="NotEnoughFoo">
      details about failure
    </failure>
    <!-- -->
  </testcase>
  <!-- -->
  <testcase classname="foo4" name="ATestOnError">
    <!-- -->
    <error type="Setup">
      setup failure
    </error>
    <!-- -->
  </testcase>
  <!-- -->
  <testcase classname="foo5" name="ASkippedTest">
    <!-- -->
    <skipped type="skip">
      details about being skipped
    </skipped>
    <!-- -->
  </testcase>
</testsuite>
</testsuites>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let tss = r.unwrap();
    assert_eq!(tss.suites.len(), 1);
    let ts = &tss.suites[0];
    assert_eq!(ts.cases.len(), 5);

    let tc = &ts.cases[0];
    assert_eq!(tc.original_name, "ASuccessfulTest");
    assert!(tc.status.is_success());

    let tc = &ts.cases[1];
    assert_eq!(tc.original_name, "AnotherSuccessfulTest");
    assert!(tc.status.is_success());

    let tc = &ts.cases[2];
    assert_eq!(tc.original_name, "AFailingTest");
    assert!(tc.status.is_failure());
    let tf = tc.status.failure_as_ref();
    assert_eq!(tf.failure_type, "NotEnoughFoo");
    assert_eq!(tf.message, "");
    println!("{:?}", tf.text);
    assert_eq!(tf.text, "details about failure");

    let tc = &ts.cases[3];
    assert_eq!(tc.original_name, "ATestOnError");
    assert!(tc.status.is_error());
    let tf = tc.status.error_as_ref();
    assert_eq!(tf.error_type, "Setup");
    assert_eq!(tf.message, "");
    assert_eq!(tf.text, "setup failure");

    let tc = &ts.cases[4];
    assert_eq!(tc.original_name, "ASkippedTest");
    assert!(tc.status.is_skipped());
    let tf = tc.status.skipped_as_ref();
    assert_eq!(tf.skipped_type, "skip");
    assert_eq!(tf.message, "");
    assert_eq!(tf.text, "details about being skipped");
}

#[test]
/// Test unescaping attributes
fn test_attr_unescaped() {
    let xml = r#"<testsuites tests="1" name="&lt;suites&gt;">
<testsuite name="&lt;suite&gt;" tests="1">
  <testcase classname="&lt;class&gt;" name="&lt;name&gt;"/>
  <testcase classname="&lt;class&gt;" name="&lt;AFailingTest&gt;">
    <failure type="&lt;NotEnoughFoo&gt;">
      details about failure
    </failure>
  </testcase>
  <testcase classname="&lt;class&gt;" name="&lt;ATestOnError&gt;">
    <error type="&lt;Setup&gt;">
      setup failure
    </error>
  </testcase>
  <testcase classname="&lt;class&gt;" name="&lt;ASkippedTest&gt;">
    <skipped type="&lt;skip&gt;">
      details about being skipped
    </skipped>
  </testcase>
</testsuite>
</testsuites>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.name, "<suites>");
    assert_eq!(t.suites.len(), 1);
    let ts = &t.suites[0];
    assert_eq!(ts.cases.len(), 4);
    let tc = &ts.cases[0];
    assert_eq!(tc.name, "<class>::<name>");
    let tc = &ts.cases[1];
    assert_eq!(tc.name, "<class>::<AFailingTest>");
    let te = tc.status.failure_as_ref();
    assert_eq!(te.failure_type, "<NotEnoughFoo>");
    let tc = &ts.cases[2];
    assert_eq!(tc.name, "<class>::<ATestOnError>");
    let te = tc.status.error_as_ref();
    assert_eq!(te.error_type, "<Setup>");
    let tc = &ts.cases[3];
    assert_eq!(tc.name, "<class>::<ASkippedTest>");
    let te = tc.status.skipped_as_ref();
    assert_eq!(te.skipped_type, "<skip>");
}

#[test]
/// Testing parsing system-out/err
/// Ensure whitespace is kept
fn test_system_out_err() {
    let xml1 = r#"
    <testsuite name="foo" tests="1" time="30.23">
      <testcase name="bar" time="12.34" >
        <system-out>
  tc-out
  tc-out
        </system-out>
        <system-err>
  tc-err
  tc-err
        </system-err>
      </testcase>
      <system-out>
  ts-out
  ts-out
      </system-out>
      <system-err>
  ts-err
  ts-err
      </system-err>
    </testsuite>"#;
    let xml2 = r#"
    <testsuite name="foo" tests="1" time="30.23">
      <testcase name="bar" time="12.34" >
        <system-out><![CDATA[
  tc-out
  tc-out
        ]]></system-out>
        <system-err><![CDATA[
  tc-err
  tc-err
        ]]></system-err>
      </testcase>
      <system-out><![CDATA[
  ts-out
  ts-out
      ]]></system-out>
      <system-err><![CDATA[
  ts-err
  ts-err
      ]]></system-err>
    </testsuite>"#;
    for xml in [xml1, xml2] {
        let cursor = Cursor::new(xml);
        let r = junit_parser::from_reader(cursor);
        assert!(r.is_ok());
        let t = r.unwrap();
        assert_eq!(t.suites.len(), 1);
        let ts = &t.suites[0];
        assert!(ts.system_out.is_some());
        assert_eq!(
            ts.system_out,
            Some(
                r#"
  ts-out
  ts-out
      "#
                .to_string()
            )
        );
        assert!(ts.system_err.is_some());
        assert_eq!(
            ts.system_err,
            Some(
                r#"
  ts-err
  ts-err
      "#
                .to_string()
            )
        );
        assert_eq!(ts.cases.len(), 1);
        let tc = &ts.cases[0];
        assert!(tc.system_out.is_some());
        assert!(tc.system_out.is_some());
        assert_eq!(
            tc.system_out,
            Some(
                r#"
  tc-out
  tc-out
        "#
                .to_string()
            )
        );
        assert!(tc.system_err.is_some());
        assert_eq!(
            tc.system_err,
            Some(
                r#"
  tc-err
  tc-err
        "#
                .to_string()
            )
        );
    }
    let xml3 = r#"
    <testsuite name="foo" tests="1" time="30.23">
      <testcase name="bar" time="12.34" >
        <system-out><![CDATA[
  <tc-out>
  </tc-out>
        ]]></system-out>
        <system-err><![CDATA[
  <tc-err>
  </tc-err>
        ]]></system-err>
      </testcase>
      <system-out><![CDATA[
  <ts-out>
  </ts-out>
      ]]></system-out>
      <system-err><![CDATA[
  <ts-err>
  </ts-err>
      ]]></system-err>
    </testsuite>"#;
    let cursor = Cursor::new(xml3);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let ts = &t.suites[0];
    assert!(ts.system_out.is_some());
    assert_eq!(
        ts.system_out,
        Some(
            r#"
  <ts-out>
  </ts-out>
      "#
            .to_string()
        )
    );
    assert!(ts.system_err.is_some());
    assert_eq!(
        ts.system_err,
        Some(
            r#"
  <ts-err>
  </ts-err>
      "#
            .to_string()
        )
    );
    assert_eq!(ts.cases.len(), 1);
    let tc = &ts.cases[0];
    assert!(tc.system_out.is_some());
    assert!(tc.system_out.is_some());
    assert_eq!(
        tc.system_out,
        Some(
            r#"
  <tc-out>
  </tc-out>
        "#
            .to_string()
        )
    );
    assert!(tc.system_err.is_some());
    assert_eq!(
        tc.system_err,
        Some(
            r#"
  <tc-err>
  </tc-err>
        "#
            .to_string()
        )
    );
}

#[test]
fn test_system_out_err_empty() {
    let xml = r#"
<testsuite>
  <system-out />
  <system-err />
  <testcase classname="foo1" group="gr1" name="ASuccessfulTest">
    <system-out />
    <system-err />
  </testcase>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
}

#[test]
fn test_system_out_err_no_content() {
    let xml = r#"
<testsuite>
  <system-out> </system-out>
  <system-err> </system-err>
  <testcase classname="foo1" group="gr1" name="ASuccessfulTest">
    <system-out> </system-out>
    <system-err> </system-err>
  </testcase>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
}

#[test]
/// Test that test suite can have nested test suites
fn test_sub_testsuites() {
    let xml = r#"
<testsuites name="mytestsuites" tests="3" failures="1">
    <testsuite name="firstsuite">
        <testsuite name="mygroup">
            <testcase classname="foo1" name="a"/>
            <testcase classname="foo2" name="b"/>
        </testsuite>
        <testcase classname="foo3" name="c"/>
        <testcase classname="foo4" name="d"/>
        <testcase classname="foo5" name="e"/>

        <testsuite name="mygroup2">
            <testcase classname="foo6" name="f"/>
            <testcase classname="foo7" name="g"/>
        </testsuite>
    </testsuite>
</testsuites>
"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let tss = r.unwrap();
    assert_eq!(tss.suites.len(), 1);
    let ts1 = &tss.suites[0];
    assert_eq!(ts1.name, "firstsuite");
    assert_eq!(ts1.suites.len(), 2);
    assert_eq!(ts1.suites[0].name, "mygroup");
    assert_eq!(ts1.suites[0].cases.len(), 2);
    assert_eq!(ts1.suites[0].cases[0].name, "foo1::a");
    assert_eq!(ts1.suites[0].cases[1].name, "foo2::b");
    assert_eq!(ts1.cases.len(), 3);
    assert_eq!(ts1.cases[0].name, "foo3::c");
    assert_eq!(ts1.cases[1].name, "foo4::d");
    assert_eq!(ts1.cases[2].name, "foo5::e");
    assert_eq!(ts1.suites[1].name, "mygroup2");
    assert_eq!(ts1.suites[1].cases[0].name, "foo6::f");
    assert_eq!(ts1.suites[1].cases[1].name, "foo7::g");
}

#[test]
/// Test that test suite can have nested test suites
fn test_many_nested_testsuites() {
    let xml = r#"
<testsuites name="mytestsuites">
  <testsuite name="suite1">
    <testsuite name="suite2">
      <testsuite name="suite3">
        <testsuite name="suite4">
          <testsuite name="suite5">
            <testcase classname="foo1" name="a"/>
            <testcase classname="foo2" name="b"/>
          </testsuite>
        </testsuite>
      </testsuite>
    </testsuite>
  </testsuite>
</testsuites>
"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let tss = r.unwrap();
    assert_eq!(tss.name, "mytestsuites");
    assert_eq!(tss.suites.len(), 1);
    assert_eq!(tss.suites[0].name, "suite1");
    assert_eq!(tss.suites[0].suites.len(), 1);
    assert_eq!(tss.suites[0].suites[0].name, "suite2");
    assert_eq!(tss.suites[0].suites[0].suites.len(), 1);
    assert_eq!(tss.suites[0].suites[0].suites[0].name, "suite3");
    assert_eq!(tss.suites[0].suites[0].suites[0].suites.len(), 1);
    assert_eq!(tss.suites[0].suites[0].suites[0].suites[0].name, "suite4");
    assert_eq!(tss.suites[0].suites[0].suites[0].suites[0].suites.len(), 1);
    assert_eq!(
        tss.suites[0].suites[0].suites[0].suites[0].suites[0].name,
        "suite5"
    );
    assert_eq!(
        tss.suites[0].suites[0].suites[0].suites[0].suites[0]
            .suites
            .len(),
        0
    );
    assert_eq!(
        tss.suites[0].suites[0].suites[0].suites[0].suites[0]
            .cases
            .len(),
        2
    );

    assert_eq!(
        tss.suites[0].suites[0].suites[0].suites[0].suites[0].cases[0].name,
        "foo1::a"
    );
    assert_eq!(
        tss.suites[0].suites[0].suites[0].suites[0].suites[0].cases[1].name,
        "foo2::b"
    );
}

#[test]
/// Test example with a `testrun` tag
fn test_testrun() {
    let xml = r#"
<testrun>
 <testsuite tests="3" failures="1">
   <testcase classname="foo1" name="ASuccessfulTest"/>
   <testcase classname="foo2" name="AnotherSuccessfulTest"/>
   <testcase classname="foo3" name="AFailingTest">
     <failure type="NotEnoughFoo"> details about failure </failure>
   </testcase>
 </testsuite>
</testrun>
 "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let tss = r.unwrap();
    assert_eq!(tss.suites.len(), 1);
    let ts = &tss.suites[0];
    assert_eq!(ts.cases.len(), 3);
    assert!(ts.cases[0].status.is_success());
    assert!(ts.cases[1].status.is_success());
    assert!(ts.cases[2].status.is_failure());
}
