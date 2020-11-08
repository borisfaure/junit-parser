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

#[test]
fn empty_test_suite() {
    let xml = r#"<testsuites>
    <testsuite/>
        </testsuites>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let t = t.suites.get("").unwrap();
    assert_eq!(t.cases.len(), 0);
}
#[test]
fn empty_test_suite_empty_attributes() {
    let xml = r#"<testsuites><testsuite
        tests="" name="" time="" errors="" failures="" skipped="" />
        </testsuites>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let t = t.suites.get("").unwrap();
    assert_eq!(t.time, 0f64);
    assert_eq!(t.tests, 0u64);
    assert_eq!(t.errors, 0u64);
    assert_eq!(t.failures, 0u64);
    assert_eq!(t.skipped, 0u64);
    assert_eq!(t.cases.len(), 0);
    assert_eq!(t.name, "");
}

#[test]
fn empty_test_suite_start_end() {
    let xml = r#"<testsuites>
        <testsuite></testsuite>
        </testsuites>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let t = t.suites.get("").unwrap();
    assert_eq!(t.time, 0f64);
    assert_eq!(t.tests, 0u64);
    assert_eq!(t.errors, 0u64);
    assert_eq!(t.failures, 0u64);
    assert_eq!(t.skipped, 0u64);
    assert_eq!(t.name, "");
    assert_eq!(t.cases.len(), 0);
}

#[test]
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
    let t = t.suites.get("AllTests").unwrap();
    assert_eq!(t.time, 38730.23f64);
    assert_eq!(t.tests, 22u64);
    assert_eq!(t.errors, 5u64);
    assert_eq!(t.failures, 9u64);
    assert_eq!(t.skipped, 3u64);
    assert_eq!(t.name, "AllTests");
    assert_eq!(t.cases.len(), 0);
}

#[test]
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
    let t = t.suites.get("AllTests").unwrap();
    assert_eq!(t.time, 38730.23f64);
    assert_eq!(t.tests, 22u64);
    assert_eq!(t.errors, 5u64);
    assert_eq!(t.failures, 9u64);
    assert_eq!(t.skipped, 3u64);
    assert_eq!(t.name, "AllTests");
    assert_eq!(t.cases.len(), 0);
}

#[test]
fn no_suites_empty_test_suite() {
    let xml = r#"<testsuite/>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let t = t.suites.get("").unwrap();
    assert_eq!(t.cases.len(), 0);
}
#[test]
fn no_suites_empty_test_suite_empty_attributes() {
    let xml = r#"<testsuite
        tests="" name="" time="" errors="" failures="" skipped="" />"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let t = t.suites.get("").unwrap();
    assert_eq!(t.time, 0f64);
    assert_eq!(t.tests, 0u64);
    assert_eq!(t.errors, 0u64);
    assert_eq!(t.failures, 0u64);
    assert_eq!(t.skipped, 0u64);
    assert_eq!(t.name, "");
    assert_eq!(t.cases.len(), 0);
}

#[test]
fn no_suites_empty_test_suites_start_end() {
    let xml = r#"<testsuite></testsuite>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let t = t.suites.get("").unwrap();
    assert_eq!(t.time, 0f64);
    assert_eq!(t.tests, 0u64);
    assert_eq!(t.errors, 0u64);
    assert_eq!(t.failures, 0u64);
    assert_eq!(t.skipped, 0u64);
    assert_eq!(t.name, "");
    assert_eq!(t.cases.len(), 0);
}

#[test]
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
    let t = t.suites.get("AllTests").unwrap();
    assert_eq!(t.time, 38730.23f64);
    assert_eq!(t.tests, 22u64);
    assert_eq!(t.errors, 5u64);
    assert_eq!(t.failures, 9u64);
    assert_eq!(t.skipped, 3u64);
    assert_eq!(t.name, "AllTests");
    assert_eq!(t.cases.len(), 0);
}

#[test]
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
    let t = t.suites.get("AllTests").unwrap();
    assert_eq!(t.time, 38730.23f64);
    assert_eq!(t.tests, 22u64);
    assert_eq!(t.errors, 5u64);
    assert_eq!(t.failures, 9u64);
    assert_eq!(t.skipped, 3u64);
    assert_eq!(t.name, "AllTests");
    assert_eq!(t.cases.len(), 0);
}

#[test]
fn test_case_success() {
    let xml = r#"<testsuite name="foo" tests="1" time="30.23" >
    <testcase name="bar" time="12.34" />
            </testsuite>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let ts = t.suites.get("foo").unwrap();
    assert_eq!(ts.cases.len(), 1);
    let tc = ts.cases.get("bar").unwrap();
    assert_eq!(tc.time, 12.34f64);
    assert_eq!(tc.name, "bar");
    assert!(tc.status.is_success());
}
#[test]
fn test_case_success_stdout() {
    let xml = r#"<testsuite name="foo" tests="1" time="30.23" >
    <testcase name="bar" time="12.34" >
    <sys-out>Hi :)</sys-out>
    </testcase>
            </testsuite>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let ts = t.suites.get("foo").unwrap();
    assert_eq!(ts.cases.len(), 1);
    let tc = ts.cases.get("bar").unwrap();
    assert_eq!(tc.time, 12.34f64);
    assert_eq!(tc.name, "bar");
    assert!(tc.status.is_success());
}

#[test]
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
    let ts = t.suites.get("foo").unwrap();
    assert_eq!(ts.cases.len(), 1);
    let tc = ts.cases.get("bar").unwrap();
    assert_eq!(tc.time, 12.34f64);
    assert_eq!(tc.name, "bar");
    assert!(tc.status.is_error());
}

#[test]
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
    let ts = t.suites.get("foo").unwrap();
    assert_eq!(ts.cases.len(), 1);
    let tc = ts.cases.get("bar").unwrap();
    assert_eq!(tc.time, 12.34f64);
    assert_eq!(tc.name, "bar");
    assert!(tc.status.is_error());
    let te = tc.status.error_as_ref();
    assert_eq!(te.error_type, "error");
    assert_eq!(te.message, "exception raised");
}

#[test]
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
    let ts = t.suites.get("foo").unwrap();
    assert_eq!(ts.cases.len(), 1);
    let tc = ts.cases.get("bar").unwrap();
    assert_eq!(tc.time, 12.34f64);
    assert_eq!(tc.name, "bar");
    assert!(tc.status.is_error());
    let te = tc.status.error_as_ref();
    assert_eq!(te.error_type, "error");
    assert_eq!(te.message, "exception raised");
    assert_eq!(te.text, "foo::bar asserted!");
}

#[test]
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
    let ts = t.suites.get("foo").unwrap();
    assert_eq!(ts.cases.len(), 1);
    let tc = ts.cases.get("bar").unwrap();
    assert_eq!(tc.time, 12.34f64);
    assert_eq!(tc.name, "bar");
    assert!(tc.status.is_failure());
}

#[test]
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
    let ts = t.suites.get("foo").unwrap();
    assert_eq!(ts.cases.len(), 1);
    let tc = ts.cases.get("bar").unwrap();
    assert_eq!(tc.time, 12.34f64);
    assert_eq!(tc.name, "bar");
    assert!(tc.status.is_failure());
    let te = tc.status.failure_as_ref();
    assert_eq!(te.failure_type, "failure");
    assert_eq!(te.message, "test failed");
}

#[test]
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
    let ts = t.suites.get("foo").unwrap();
    assert_eq!(ts.cases.len(), 1);
    let tc = ts.cases.get("bar").unwrap();
    assert_eq!(tc.time, 12.34f64);
    assert_eq!(tc.name, "bar");
    assert!(tc.status.is_failure());
    let te = tc.status.failure_as_ref();
    assert_eq!(te.failure_type, "failure");
    assert_eq!(te.message, "test failed");
    assert_eq!(te.text, "foo::bar failed!");
}

#[test]
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
    let ts = t.suites.get("foo").unwrap();
    assert_eq!(ts.cases.len(), 1);
    let tc = ts.cases.get("bar").unwrap();
    assert_eq!(tc.time, 12.34f64);
    assert_eq!(tc.name, "bar");
    assert!(tc.status.is_skipped());
}

#[test]
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
    let ts = t.suites.get("foo").unwrap();
    assert_eq!(ts.cases.len(), 1);
    let tc = ts.cases.get("bar").unwrap();
    assert_eq!(tc.time, 12.34f64);
    assert_eq!(tc.name, "bar");
    assert!(tc.status.is_skipped());
    let te = tc.status.skipped_as_ref();
    assert_eq!(te.skipped_type, "skipped");
    assert_eq!(te.message, "test skipped");
}

#[test]
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
    let ts = t.suites.get("foo").unwrap();
    assert_eq!(ts.cases.len(), 1);
    let tc = ts.cases.get("bar").unwrap();
    assert_eq!(tc.time, 12.34f64);
    assert_eq!(tc.name, "bar");
    assert!(tc.status.is_skipped());
    let te = tc.status.skipped_as_ref();
    assert_eq!(te.skipped_type, "skipped");
    assert_eq!(te.message, "test skipped");
    assert_eq!(te.text, "foo::bar skipped for some reason");
}
