//! Test CDATA parsing in various elements

use junit_parser;
use std::io::Cursor;

#[test]
fn test_cdata_failure() {
    let xml = r#"
<testsuite>
  <testcase name="AFailingTest">
    <failure>
      <![CDATA[
        <foo>
    ]]></failure>
    </testcase>
</testsuite>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let suite = &r.unwrap().suites[0];
    assert_eq!(suite.cases.len(), 1);
    let tc = &suite.cases[0];
    assert_eq!(tc.name, "AFailingTest");
    let tf = tc.status.failure_as_ref();
    assert_eq!(
        tf.text,
        r#"
        <foo>
    "#
    );
}

#[test]
fn test_cdata_error() {
    let xml = r#"
<testsuite>
  <testcase name="AFailingTest">
    <error>
      <![CDATA[
        <foo>
    ]]></error>
    </testcase>
</testsuite>"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let suite = &r.unwrap().suites[0];
    assert_eq!(suite.cases.len(), 1);
    let tc = &suite.cases[0];
    assert_eq!(tc.name, "AFailingTest");
    let te = tc.status.error_as_ref();
    assert_eq!(
        te.text,
        r#"
        <foo>
    "#
    );
}
