use std::io::Cursor;

#[test]
#[cfg(not(any(feature = "chrono", feature = "jiff")))]
/// Test parsing the `timestamp` attribute of the `testsuites`, `testsuite`
/// and `testcase` elements
fn test_timestamps_as_string() {
    let xml = r#"
<testsuites timestamp="2025-09-28T11:11:11+00:00">
 <testsuite name="suite1" timestamp="2025-09-28T12:34:56+00:00">
   <testcase classname="foo1" name="test1" timestamp="2025-09-28T22:33:44+00:00"/>
   <testcase classname="foo2" name="test2"/>
 </testsuite>
</testsuites>
 "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let tss = r.unwrap();
    assert_eq!(tss.timestamp, Some("2025-09-28T11:11:11+00:00".to_string()));
    assert_eq!(tss.suites.len(), 1);
    let ts = &tss.suites[0];
    assert_eq!(ts.name, "suite1");
    assert_eq!(ts.timestamp, Some("2025-09-28T12:34:56+00:00".to_string()));
    assert_eq!(ts.cases.len(), 2);
    let tc = &ts.cases[0];
    assert_eq!(tc.name, "foo1::test1");
    assert_eq!(tc.timestamp, Some("2025-09-28T22:33:44+00:00".to_string()));
    let tc = &ts.cases[1];
    assert_eq!(tc.name, "foo2::test2");
    assert_eq!(tc.timestamp, None);
}

#[test]
#[cfg(all(feature = "chrono", not(feature = "jiff")))]
/// Test parsing the `timestamp` attribute of the `testsuites`, `testsuite`
/// and `testcase` elements
fn test_timestamps_as_datetime() {
    use chrono::{TimeZone, Utc};
    let xml = r#"
<testsuites timestamp="2025-09-28T11:11:11+00:00">
 <testsuite name="suite1" timestamp="2025-09-28T12:34:56+00:00">
   <testcase classname="foo1" name="test1" timestamp="2025-09-28T22:33:44+00:00"/>
   <testcase classname="foo2" name="test2">
    <flakyFailure type="SomeFlakyType" message="Flaky message" timestamp="2025-09-28T10:20:30+00:00">
      Failure details
    </flakyFailure>
   </testcase>
 </testsuite>
</testsuites>
 "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let tss = r.unwrap();
    let dt = Utc.with_ymd_and_hms(2025, 9, 28, 11, 11, 11).unwrap();
    assert_eq!(tss.timestamp, Some(dt));
    assert_eq!(tss.suites.len(), 1);
    let ts = &tss.suites[0];
    assert_eq!(ts.name, "suite1");
    let dt = Utc.with_ymd_and_hms(2025, 9, 28, 12, 34, 56).unwrap();
    assert_eq!(ts.timestamp, Some(dt));
    assert_eq!(ts.cases.len(), 2);
    let tc = &ts.cases[0];
    assert_eq!(tc.name, "foo1::test1");
    let dt = Utc.with_ymd_and_hms(2025, 9, 28, 22, 33, 44).unwrap();
    assert_eq!(tc.timestamp, Some(dt));
    let tc = &ts.cases[1];
    assert_eq!(tc.name, "foo2::test2");
    assert_eq!(tc.timestamp, None);
    let ff = &tc.reruns[0];
    let dt = Utc.with_ymd_and_hms(2025, 9, 28, 10, 20, 30).unwrap();
    assert_eq!(ff.timestamp, Some(dt));
}

#[test]
#[cfg(feature = "jiff")]
/// Test parsing the `timestamp` attribute of the `testsuites`, `testsuite`
/// and `testcase` elements
fn test_timestamps_as_jiff_timestamp() {
    use jiff::{Timestamp, civil::date, tz::TimeZone};

    /// Build the [`Timestamp`] for the given UTC date and time
    fn utc(y: i16, m: i8, d: i8, h: i8, mi: i8, s: i8) -> Timestamp {
        date(y, m, d)
            .at(h, mi, s, 0)
            .to_zoned(TimeZone::UTC)
            .unwrap()
            .timestamp()
    }

    let xml = r#"
<testsuites timestamp="2025-09-28T11:11:11+00:00">
 <testsuite name="suite1" timestamp="2025-09-28T12:34:56+00:00">
   <testcase classname="foo1" name="test1" timestamp="2025-09-28T22:33:44+00:00"/>
   <testcase classname="foo2" name="test2">
    <flakyFailure type="SomeFlakyType" message="Flaky message" timestamp="2025-09-28T10:20:30+00:00">
      Failure details
    </flakyFailure>
   </testcase>
 </testsuite>
</testsuites>
 "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let tss = r.unwrap();
    assert_eq!(tss.timestamp, Some(utc(2025, 9, 28, 11, 11, 11)));
    assert_eq!(tss.suites.len(), 1);
    let ts = &tss.suites[0];
    assert_eq!(ts.name, "suite1");
    assert_eq!(ts.timestamp, Some(utc(2025, 9, 28, 12, 34, 56)));
    assert_eq!(ts.cases.len(), 2);
    let tc = &ts.cases[0];
    assert_eq!(tc.name, "foo1::test1");
    assert_eq!(tc.timestamp, Some(utc(2025, 9, 28, 22, 33, 44)));
    let tc = &ts.cases[1];
    assert_eq!(tc.name, "foo2::test2");
    assert_eq!(tc.timestamp, None);
    let ff = &tc.reruns[0];
    assert_eq!(ff.timestamp, Some(utc(2025, 9, 28, 10, 20, 30)));
}

#[test]
#[cfg(feature = "jiff")]
/// Test that a `timestamp` attribute with a non-UTC offset is normalized to
/// the right instant
fn test_timestamps_as_jiff_timestamp_with_offset() {
    let xml = r#"
<testsuite name="suite1" timestamp="2025-09-28T14:34:56+02:00"/>
 "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let tss = r.unwrap();
    let ts = &tss.suites[0];
    assert_eq!(
        ts.timestamp,
        Some("2025-09-28T12:34:56Z".parse::<jiff::Timestamp>().unwrap())
    );
}

#[test]
#[cfg(any(feature = "chrono", feature = "jiff"))]
/// Test that an invalid `timestamp` attribute is reported as an error
fn test_invalid_timestamp() {
    let xml = r#"
<testsuite name="suite1" timestamp="not a timestamp"/>
 "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_err());
}
