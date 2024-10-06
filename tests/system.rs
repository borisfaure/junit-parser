//! System-out/err parsing tests

use junit_parser;
use std::io::Cursor;

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
