use junit_parser;
use junit_parser::Error;
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

#[test]
fn test_error_xml() {
    let xml = r#"<testsuites skipped"1" />"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_err());
    let err = r.err().unwrap();
    assert!(matches!(err, Error::XMLError(_)));
}
#[test]
fn test_error_xml_end_mismatch() {
    let xml = r#"<testsuites> <foo> </bar> </testsuites>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_err());
    let err = r.err().unwrap();
    assert!(matches!(err, Error::XMLError(_)));
}

#[test]
fn test_error_parseint() {
    let xml = r#"<testsuites skipped="foo" />"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_err());
    let err = r.err().unwrap();
    assert!(matches!(err, Error::ParseIntError(_)));
}

#[test]
fn test_error_parsefloat() {
    let xml = r#"<testsuites time="foo" />"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_err());
    let err = r.err().unwrap();
    assert!(matches!(err, Error::ParseFloatError(_)));
}

#[test]
fn test_error_duplicate_suites() {
    let xml = r#"<testsuites>
        <testsuite name="foo" />
        <testsuite name="foo" />
        </testsuites>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_err());
    let err = r.err().unwrap();
    assert!(matches!(err, Error::DuplicateError{..}));
}

#[test]
fn test_error_duplicate_cases() {
    let xml = r#"<testsuite>
        <testcase name="foo" />
        <testcase name="foo" />
        </testsuite>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_err());
    let err = r.err().unwrap();
    assert!(matches!(err, Error::DuplicateError{..}));
}

#[test]
fn test_large_test_suite() {
    let xml = r#"
<testsuite tests="3" failures="1">
  <testcase classname="foo1" name="ASuccessfulTest"/>
  <testcase classname="foo2" name="AnotherSuccessfulTest"/>
  <testcase classname="foo3" name="AFailingTest">
    <failure type="NotEnoughFoo"> details about failure </failure>
  </testcase>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let tss = r.unwrap();
    assert_eq!(tss.suites.len(), 1);
    let ts = tss.suites.get("").unwrap();
    assert_eq!(ts.cases.len(), 3);
    let tc = ts.cases.get("foo1::ASuccessfulTest").unwrap();
    assert_eq!(tc.original_name, "ASuccessfulTest");
    assert!(tc.status.is_success());
    let tc = ts.cases.get("foo2::AnotherSuccessfulTest").unwrap();
    assert_eq!(tc.original_name, "AnotherSuccessfulTest");
    assert!(tc.status.is_success());
    let tc = ts.cases.get("foo3::AFailingTest").unwrap();
    assert_eq!(tc.original_name, "AFailingTest");
    assert!(tc.status.is_failure());
    let tf = tc.status.failure_as_ref();
    assert_eq!(tf.failure_type, "NotEnoughFoo");
    assert_eq!(tf.message, "");
    assert_eq!(tf.text, "details about failure");
}

#[test]
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
    let ts = tss.suites.get("foo").unwrap();
    assert_eq!(ts.cases.len(), 3);
    let tc = ts.cases.get("foo1::ASuccessfulTest").unwrap();
    assert_eq!(tc.original_name, "ASuccessfulTest");
    assert!(tc.status.is_success());
    let tc = ts.cases.get("foo2::AnotherSuccessfulTest").unwrap();
    assert_eq!(tc.original_name, "AnotherSuccessfulTest");
    assert!(tc.status.is_success());
    let tc = ts.cases.get("foo3::AFailingTest").unwrap();
    assert_eq!(tc.original_name, "AFailingTest");
    assert!(tc.status.is_failure());
    let tf = tc.status.failure_as_ref();
    assert_eq!(tf.failure_type, "NotEnoughFoo");
    assert_eq!(tf.message, "");
    assert_eq!(tf.text, "details about failure");

    let ts = tss.suites.get("bar").unwrap();
    assert_eq!(ts.cases.len(), 3);
    let tc = ts.cases.get("bar1::ASuccessfulTest").unwrap();
    assert_eq!(tc.original_name, "ASuccessfulTest");
    assert!(tc.status.is_success());
    let tc = ts.cases.get("bar2::AnotherSuccessfulTest").unwrap();
    assert_eq!(tc.original_name, "AnotherSuccessfulTest");
    assert!(tc.status.is_success());
    let tc = ts.cases.get("bar3::AFailingTest").unwrap();
    assert_eq!(tc.original_name, "AFailingTest");
    assert!(tc.status.is_failure());
    let tf = tc.status.failure_as_ref();
    assert_eq!(tf.failure_type, "NotEnoughBar");
    assert_eq!(tf.message, "");
    assert_eq!(tf.text, "details about failure");
}

#[test]
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
    let ts = tss.suites.get("foo").unwrap();
    assert_eq!(ts.cases.len(), 5);

    let tc = ts.cases.get("foo1::ASuccessfulTest").unwrap();
    assert_eq!(tc.original_name, "ASuccessfulTest");
    assert!(tc.status.is_success());

    let tc = ts.cases.get("foo2::AnotherSuccessfulTest").unwrap();
    assert_eq!(tc.original_name, "AnotherSuccessfulTest");
    assert!(tc.status.is_success());

    let tc = ts.cases.get("foo3::AFailingTest").unwrap();
    assert_eq!(tc.original_name, "AFailingTest");
    assert!(tc.status.is_failure());
    let tf = tc.status.failure_as_ref();
    assert_eq!(tf.failure_type, "NotEnoughFoo");
    assert_eq!(tf.message, "");
    println!("{:?}", tf.text);
    assert_eq!(tf.text, "details about failure");

    let tc = ts.cases.get("foo4::ATestOnError").unwrap();
    assert_eq!(tc.original_name, "ATestOnError");
    assert!(tc.status.is_error());
    let tf = tc.status.error_as_ref();
    assert_eq!(tf.error_type, "Setup");
    assert_eq!(tf.message, "");
    assert_eq!(tf.text, "setup failure");

    let tc = ts.cases.get("foo5::ASkippedTest").unwrap();
    assert_eq!(tc.original_name, "ASkippedTest");
    assert!(tc.status.is_skipped());
    let tf = tc.status.skipped_as_ref();
    assert_eq!(tf.skipped_type, "skip");
    assert_eq!(tf.message, "");
    assert_eq!(tf.text, "details about being skipped");
}

#[test]
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
    let ts = tss.suites.get("foo").unwrap();
    assert_eq!(ts.cases.len(), 5);

    let tc = ts.cases.get("foo1::ASuccessfulTest").unwrap();
    assert_eq!(tc.original_name, "ASuccessfulTest");
    assert!(tc.status.is_success());

    let tc = ts.cases.get("foo2::AnotherSuccessfulTest").unwrap();
    assert_eq!(tc.original_name, "AnotherSuccessfulTest");
    assert!(tc.status.is_success());

    let tc = ts.cases.get("foo3::AFailingTest").unwrap();
    assert_eq!(tc.original_name, "AFailingTest");
    assert!(tc.status.is_failure());
    let tf = tc.status.failure_as_ref();
    assert_eq!(tf.failure_type, "NotEnoughFoo");
    assert_eq!(tf.message, "");
    println!("{:?}", tf.text);
    assert_eq!(tf.text, "details about failure");

    let tc = ts.cases.get("foo4::ATestOnError").unwrap();
    assert_eq!(tc.original_name, "ATestOnError");
    assert!(tc.status.is_error());
    let tf = tc.status.error_as_ref();
    assert_eq!(tf.error_type, "Setup");
    assert_eq!(tf.message, "");
    assert_eq!(tf.text, "setup failure");

    let tc = ts.cases.get("foo5::ASkippedTest").unwrap();
    assert_eq!(tc.original_name, "ASkippedTest");
    assert!(tc.status.is_skipped());
    let tf = tc.status.skipped_as_ref();
    assert_eq!(tf.skipped_type, "skip");
    assert_eq!(tf.message, "");
    assert_eq!(tf.text, "details about being skipped");
}
