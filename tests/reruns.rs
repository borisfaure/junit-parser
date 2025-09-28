//! Test the reruns parsing: `<flakyFailure />`, `<flakyError />`, `<rerunFailure />` and `<rerunError />`

#[cfg(feature = "chrono")]
use chrono::{TimeZone, Utc};
use junit_parser::RerunOrFlakyKind;
use std::io::Cursor;

// Test parsing a simple testcase with a single flakyFailure
#[test]
fn test_single_flaky_failure() {
    let xml = r#"
<testsuite tests="1">
  <testcase classname="some.class" name="SomeTest">
    <flakyFailure type="SomeFlakyType" message="Flaky message">
      Failure details
    </flakyFailure>
  </testcase>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let suites = junit_parser::from_reader(cursor).expect("Failed to parse XML");

    assert_eq!(suites.suites.len(), 1);
    let suite = &suites.suites[0];
    assert_eq!(suite.cases.len(), 1);
    let case = &suite.cases[0];

    assert!(case.status.is_success()); // Primary status is success
    assert_eq!(case.reruns.len(), 1);

    let rerun = &case.reruns[0];
    assert!(matches!(rerun.kind, RerunOrFlakyKind::FlakyFailure));
    assert_eq!(rerun.rerun_type, "SomeFlakyType");
    assert_eq!(rerun.message, "Flaky message");
    assert_eq!(rerun.text.trim(), "Failure details"); // Use trim for potential whitespace
    assert_eq!(rerun.time, 0.0);
    assert!(rerun.timestamp.is_none());
    assert!(rerun.system_out.is_none());
    assert!(rerun.system_err.is_none());
    assert!(rerun.stack_trace.is_none());
}

// Test parsing a simple testcase with a single flakyError
#[test]
fn test_single_flaky_error() {
    let xml = r#"
<testsuite tests="1">
  <testcase classname="some.class" name="SomeTest">
    <flakyError type="SomeFlakyErrorType" message="Flaky error message">
      Error details
    </flakyError>
  </testcase>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let suites = junit_parser::from_reader(cursor).expect("Failed to parse XML");

    assert_eq!(suites.suites.len(), 1);
    let suite = &suites.suites[0];
    assert_eq!(suite.cases.len(), 1);
    let case = &suite.cases[0];

    assert!(case.status.is_success()); // Primary status is success
    assert_eq!(case.reruns.len(), 1);

    let rerun = &case.reruns[0];
    assert!(matches!(rerun.kind, RerunOrFlakyKind::FlakyError));
    assert_eq!(rerun.rerun_type, "SomeFlakyErrorType");
    assert_eq!(rerun.message, "Flaky error message");
    assert_eq!(rerun.text.trim(), "Error details");
    assert_eq!(rerun.time, 0.0);
    assert!(rerun.timestamp.is_none());
    assert!(rerun.system_out.is_none());
    assert!(rerun.system_err.is_none());
    assert!(rerun.stack_trace.is_none());
}

// Test parsing a simple testcase with a single rerunFailure
#[test]
fn test_single_rerun_failure() {
    let xml = r#"
<testsuite tests="1">
  <testcase classname="some.class" name="SomeTest">
    <rerunFailure type="SomeRerunFailureType" message="Rerun failure message">
      Rerun failure details
    </rerunFailure>
  </testcase>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let suites = junit_parser::from_reader(cursor).expect("Failed to parse XML");

    assert_eq!(suites.suites.len(), 1);
    let suite = &suites.suites[0];
    assert_eq!(suite.cases.len(), 1);
    let case = &suite.cases[0];

    assert!(case.status.is_success()); // Primary status is success
    assert_eq!(case.reruns.len(), 1);

    let rerun = &case.reruns[0];
    assert!(matches!(rerun.kind, RerunOrFlakyKind::RerunFailure));
    assert_eq!(rerun.rerun_type, "SomeRerunFailureType");
    assert_eq!(rerun.message, "Rerun failure message");
    assert_eq!(rerun.text.trim(), "Rerun failure details");
    assert_eq!(rerun.time, 0.0);
    assert!(rerun.timestamp.is_none());
    assert!(rerun.system_out.is_none());
    assert!(rerun.system_err.is_none());
    assert!(rerun.stack_trace.is_none());
}

// Test parsing a simple testcase with a single rerunError
#[test]
fn test_single_rerun_error() {
    let xml = r#"
<testsuite tests="1">
  <testcase classname="some.class" name="SomeTest">
    <rerunError type="SomeRerunErrorType" message="Rerun error message">
      Rerun error details
    </rerunError>
  </testcase>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let suites = junit_parser::from_reader(cursor).expect("Failed to parse XML");

    assert_eq!(suites.suites.len(), 1);
    let suite = &suites.suites[0];
    assert_eq!(suite.cases.len(), 1);
    let case = &suite.cases[0];

    assert!(case.status.is_success()); // Primary status is success
    assert_eq!(case.reruns.len(), 1);

    let rerun = &case.reruns[0];
    assert!(matches!(rerun.kind, RerunOrFlakyKind::RerunError));
    assert_eq!(rerun.rerun_type, "SomeRerunErrorType");
    assert_eq!(rerun.message, "Rerun error message");
    assert_eq!(rerun.text.trim(), "Rerun error details");
    assert_eq!(rerun.time, 0.0);
    assert!(rerun.timestamp.is_none());
    assert!(rerun.system_out.is_none());
    assert!(rerun.system_err.is_none());
    assert!(rerun.stack_trace.is_none());
}

// Test parsing a testcase with multiple different rerun types, inspired by junit.xml
#[test]
fn test_multiple_mixed_reruns() {
    let xml = r#"
<testsuite tests="1" failures="1">
  <testcase classname="some.class" name="SomeTest" time="0.01">
    <failure type="AssertionFailed" message="Main failure msg">Main failure text</failure>
    <rerunFailure timestamp="2024-01-10T10:00:01Z" time="0.002" type="AssertionFailed" message="Rerun fail msg 1">Rerun fail text 1
      <system-out>Rerun 1 stdout</system-out>
    </rerunFailure>
    <rerunFailure timestamp="2024-01-10T10:00:02Z" time="0.003" type="AssertionFailed" message="Rerun fail msg 2">Rerun fail text 2
       <stackTrace>Rerun 2 stack</stackTrace>
    </rerunFailure>
    <flakyFailure timestamp="2024-01-10T10:00:03Z" time="0.001" type="FlakyFailType" message="Flaky fail msg">Flaky fail text</flakyFailure>
    <flakyError timestamp="2024-01-10T10:00:04Z" time="0.001" type="FlakyErrType" message="Flaky err msg">Flaky err text
        <system-err>Flaky err stderr</system-err>
    </flakyError>
  </testcase>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let suites = junit_parser::from_reader(cursor).expect("Failed to parse XML");

    let case = &suites.suites[0].cases[0];
    assert_eq!(case.reruns.len(), 4); // 2 rerunFail, 1 flakyFail, 1 flakyErr

    // Check primary status is Failure
    assert!(case.status.is_failure());
    if let junit_parser::TestStatus::Failure(f) = &case.status {
        assert_eq!(f.failure_type, "AssertionFailed");
        assert_eq!(f.message, "Main failure msg");
        assert_eq!(f.text, "Main failure text");
    } else {
        panic!("Expected primary status to be Failure");
    }

    // Check counts for each rerun kind
    let flaky_failures = case
        .reruns
        .iter()
        .filter(|r| matches!(r.kind, RerunOrFlakyKind::FlakyFailure))
        .count();
    let flaky_errors = case
        .reruns
        .iter()
        .filter(|r| matches!(r.kind, RerunOrFlakyKind::FlakyError))
        .count();
    let rerun_failures = case
        .reruns
        .iter()
        .filter(|r| matches!(r.kind, RerunOrFlakyKind::RerunFailure))
        .count();
    let rerun_errors = case
        .reruns
        .iter()
        .filter(|r| matches!(r.kind, RerunOrFlakyKind::RerunError))
        .count();

    assert_eq!(flaky_failures, 1);
    assert_eq!(flaky_errors, 1);
    assert_eq!(rerun_failures, 2); // Now expecting 2 rerunFailures
    assert_eq!(rerun_errors, 0);

    // Verify content of one specific rerunFailure (e.g., the first one)
    let first_rerun_fail = case
        .reruns
        .iter()
        .find(|r| matches!(r.kind, RerunOrFlakyKind::RerunFailure))
        .expect("Should find a RerunFailure");
    assert_eq!(first_rerun_fail.rerun_type, "AssertionFailed");
    assert_eq!(first_rerun_fail.message, "Rerun fail msg 1");
    assert_eq!(first_rerun_fail.text.trim(), "Rerun fail text 1");
    #[cfg(feature = "chrono")]
    {
        let expected_dt = Utc.with_ymd_and_hms(2024, 1, 10, 10, 0, 1).unwrap();
        assert_eq!(first_rerun_fail.timestamp, Some(expected_dt));
    }
    #[cfg(not(feature = "chrono"))]
    assert_eq!(
        first_rerun_fail.timestamp.as_deref(),
        Some("2024-01-10T10:00:01Z")
    );
    assert_eq!(first_rerun_fail.time, 0.002);
    assert_eq!(
        first_rerun_fail.system_out.as_deref(),
        Some("Rerun 1 stdout")
    );
    assert!(first_rerun_fail.system_err.is_none());
    assert!(first_rerun_fail.stack_trace.is_none());

    // Verify content of the flakyError
    let the_flaky_error = case
        .reruns
        .iter()
        .find(|r| matches!(r.kind, RerunOrFlakyKind::FlakyError))
        .expect("Should find a FlakyError");
    assert_eq!(the_flaky_error.rerun_type, "FlakyErrType");
    assert_eq!(the_flaky_error.message, "Flaky err msg");
    assert_eq!(the_flaky_error.text.trim(), "Flaky err text");
    #[cfg(feature = "chrono")]
    {
        let expected_dt = Utc.with_ymd_and_hms(2024, 1, 10, 10, 0, 4).unwrap();
        assert_eq!(the_flaky_error.timestamp, Some(expected_dt));
    }
    #[cfg(not(feature = "chrono"))]
    assert_eq!(
        the_flaky_error.timestamp.as_deref(),
        Some("2024-01-10T10:00:04Z")
    );
    assert_eq!(the_flaky_error.time, 0.001);
    assert!(the_flaky_error.system_out.is_none());
    assert_eq!(
        the_flaky_error.system_err.as_deref(),
        Some("Flaky err stderr")
    );
    assert!(the_flaky_error.stack_trace.is_none());
}

// Test parsing a rerun tag containing system-out
#[test]
fn test_rerun_with_system_out() {
    let xml = r#"
<testsuite tests="1">
  <testcase classname="some.class" name="SomeTest">
    <flakyFailure type="SomeType" message="msg">
      Flaky text
      <system-out>Stdout content here</system-out>
    </flakyFailure>
  </testcase>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let suites = junit_parser::from_reader(cursor).expect("Failed to parse XML");

    let case = &suites.suites[0].cases[0];
    assert_eq!(case.reruns.len(), 1);
    let rerun = &case.reruns[0];

    assert!(matches!(rerun.kind, RerunOrFlakyKind::FlakyFailure));
    assert_eq!(rerun.text.trim(), "Flaky text");
    assert!(rerun.system_out.is_some());
    assert_eq!(rerun.system_out.as_deref(), Some("Stdout content here"));
    assert!(rerun.system_err.is_none());
    assert!(rerun.stack_trace.is_none());
}

// Test parsing a rerun tag containing system-err
#[test]
fn test_rerun_with_system_err() {
    let xml = r#"
<testsuite tests="1">
  <testcase classname="some.class" name="SomeTest">
    <rerunError type="SomeType" message="msg">
      Rerun text
      <system-err>Stderr content here</system-err>
    </rerunError>
  </testcase>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let suites = junit_parser::from_reader(cursor).expect("Failed to parse XML");

    let case = &suites.suites[0].cases[0];
    assert_eq!(case.reruns.len(), 1);
    let rerun = &case.reruns[0];

    assert!(matches!(rerun.kind, RerunOrFlakyKind::RerunError));
    assert_eq!(rerun.text.trim(), "Rerun text");
    assert!(rerun.system_out.is_none());
    assert!(rerun.system_err.is_some());
    assert_eq!(rerun.system_err.as_deref(), Some("Stderr content here"));
    assert!(rerun.stack_trace.is_none());
}

// Test parsing a rerun tag containing stackTrace
#[test]
fn test_rerun_with_stack_trace() {
    let xml = r#"
<testsuite tests="1">
  <testcase classname="some.class" name="SomeTest">
    <rerunFailure type="SomeType" message="msg">
      Rerun text
      <stackTrace>Stack trace content here</stackTrace>
    </rerunFailure>
  </testcase>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let suites = junit_parser::from_reader(cursor).expect("Failed to parse XML");

    let case = &suites.suites[0].cases[0];
    assert_eq!(case.reruns.len(), 1);
    let rerun = &case.reruns[0];

    assert!(matches!(rerun.kind, RerunOrFlakyKind::RerunFailure));
    assert_eq!(rerun.text.trim(), "Rerun text");
    assert!(rerun.system_out.is_none());
    assert!(rerun.system_err.is_none());
    assert!(rerun.stack_trace.is_some());
    assert_eq!(
        rerun.stack_trace.as_deref(),
        Some("Stack trace content here")
    );
}

// Test parsing a rerun tag with all attributes (type, message, time, timestamp)
#[test]
fn test_rerun_all_attributes() {
    let xml = r#"
<testsuite tests="1">
  <testcase classname="some.class" name="SomeTest">
    <flakyError type="SpecificErrorType" message="Detailed error message" time="0.123" timestamp="2023-10-27T10:30:00Z">
      Error body text
    </flakyError>
  </testcase>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let suites = junit_parser::from_reader(cursor).expect("Failed to parse XML");

    let case = &suites.suites[0].cases[0];
    assert_eq!(case.reruns.len(), 1);
    let rerun = &case.reruns[0];

    assert!(matches!(rerun.kind, RerunOrFlakyKind::FlakyError));
    assert_eq!(rerun.rerun_type, "SpecificErrorType");
    assert_eq!(rerun.message, "Detailed error message");
    assert_eq!(rerun.time, 0.123);
    assert!(rerun.timestamp.is_some());
    #[cfg(feature = "chrono")]
    {
        let expected_dt = Utc.with_ymd_and_hms(2023, 10, 27, 10, 30, 0).unwrap();
        assert_eq!(rerun.timestamp, Some(expected_dt));
    }
    #[cfg(not(feature = "chrono"))]
    assert_eq!(rerun.timestamp.as_deref(), Some("2023-10-27T10:30:00Z"));
    assert_eq!(rerun.text.trim(), "Error body text");
    assert!(rerun.system_out.is_none());
    assert!(rerun.system_err.is_none());
    assert!(rerun.stack_trace.is_none());
}

// Test parsing a testcase with a primary failure status and reruns
#[test]
fn test_primary_failure_with_reruns() {
    let xml = r#"
<testsuite tests="1" failures="1">
  <testcase classname="some.class" name="SomeTest">
    <failure type="PrimaryFailureType" message="Primary failure message">
      Primary failure details
    </failure>
    <flakyFailure type="SomeFlakyType" message="Flaky message">
      Flaky failure details
    </flakyFailure>
     <rerunError type="SomeRerunErrorType" message="Rerun error message">
       Rerun error details
     </rerunError>
  </testcase>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let suites = junit_parser::from_reader(cursor).expect("Failed to parse XML");

    let case = &suites.suites[0].cases[0];

    // Check primary status
    assert!(case.status.is_failure());
    if let junit_parser::TestStatus::Failure(f) = &case.status {
        assert_eq!(f.failure_type, "PrimaryFailureType");
        assert_eq!(f.message, "Primary failure message");
        assert_eq!(f.text, "Primary failure details");
    } else {
        panic!("Expected status to be Failure");
    }

    // Check reruns are still recorded
    assert_eq!(case.reruns.len(), 2);
    assert!(case
        .reruns
        .iter()
        .any(|r| matches!(r.kind, RerunOrFlakyKind::FlakyFailure)));
    assert!(case
        .reruns
        .iter()
        .any(|r| matches!(r.kind, RerunOrFlakyKind::RerunError)));
}

// Test case where rerun element is empty (just attributes, no body/nested elements)
#[test]
fn test_empty_rerun_element_body() {
    let xml = r#"
<testsuite tests="1">
  <testcase classname="some.class" name="SomeTest">
    <rerunFailure type="EmptyBodyType" message="Empty message" />
    <flakyError type="EmptyBodyType2" message="Empty message 2" time="0.001"/>
  </testcase>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let suites = junit_parser::from_reader(cursor).expect("Failed to parse XML");

    let case = &suites.suites[0].cases[0];
    assert_eq!(case.reruns.len(), 2);
    assert!(case.status.is_success());

    // Check the first empty rerun
    let rerun1 = case
        .reruns
        .iter()
        .find(|r| r.rerun_type == "EmptyBodyType")
        .unwrap();
    assert!(matches!(rerun1.kind, RerunOrFlakyKind::RerunFailure));
    assert_eq!(rerun1.message, "Empty message");
    assert_eq!(rerun1.text, ""); // Expect empty text body
    assert!(rerun1.system_out.is_none());
    assert!(rerun1.system_err.is_none());
    assert!(rerun1.stack_trace.is_none());
    assert_eq!(rerun1.time, 0.0);
    assert!(rerun1.timestamp.is_none());

    // Check the second empty rerun
    let rerun2 = case
        .reruns
        .iter()
        .find(|r| r.rerun_type == "EmptyBodyType2")
        .unwrap();
    assert!(matches!(rerun2.kind, RerunOrFlakyKind::FlakyError));
    assert_eq!(rerun2.message, "Empty message 2");
    assert_eq!(rerun2.text, ""); // Expect empty text body
    assert!(rerun2.system_out.is_none());
    assert!(rerun2.system_err.is_none());
    assert!(rerun2.stack_trace.is_none());
    assert_eq!(rerun2.time, 0.001);
    assert!(rerun2.timestamp.is_none());
}

// Test case where system-out/err/stackTrace inside rerun is empty
#[test]
fn test_empty_nested_elements_in_rerun() {
    let xml = r#"
<testsuite tests="1">
  <testcase classname="some.class" name="SomeTest">
    <flakyFailure type="SomeType" message="msg">
      Flaky text
      <system-out></system-out>
      <system-err/>
      <stackTrace />
    </flakyFailure>
  </testcase>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let suites = junit_parser::from_reader(cursor).expect("Failed to parse XML");

    let case = &suites.suites[0].cases[0];
    assert_eq!(case.reruns.len(), 1);
    let rerun = &case.reruns[0];

    assert!(matches!(rerun.kind, RerunOrFlakyKind::FlakyFailure));
    assert_eq!(rerun.text.trim(), "Flaky text");

    // Check that empty nested elements result in Some("") or None depending on parser behavior
    // Based on `parse_system`, empty tags should result in Some("")
    assert_eq!(
        rerun.system_out,
        Some("".to_string()),
        "Expected Some(\"\") for empty <system-out>"
    );
    assert_eq!(
        rerun.system_err, None,
        "Expected None for self-closing <system-err/>"
    );
    assert_eq!(
        rerun.stack_trace, None,
        "Expected None for self-closing <stackTrace/>"
    );
}

// Test case demonstrating that only the *first* system-out/err within a rerun is parsed
#[test]
fn test_multiple_system_out_err_in_rerun() {
    let xml = r#"
<testsuite tests="1">
  <testcase classname="some.class" name="SomeTest">
    <flakyFailure type="MultiSysType" message="MultiSysMsg">
      Main flaky text.
      <system-out>First stdout</system-out>
      <system-err>First stderr</system-err>
      Some more text.
      <system-out>Second stdout</system-out>
      <system-err>Second stderr</system-err>
      <system-out/>
      <system-err/>
    </flakyFailure>
  </testcase>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let suites = junit_parser::from_reader(cursor).expect("Failed to parse XML");

    let case = &suites.suites[0].cases[0];
    assert_eq!(case.reruns.len(), 1);
    let rerun = &case.reruns[0];

    assert!(matches!(rerun.kind, RerunOrFlakyKind::FlakyFailure));
    assert!(rerun.system_out.is_some());
    assert_eq!(
        rerun.system_out.as_deref(),
        Some("First stdout\nSecond stdout"),
        "Expected all system-out content"
    );

    assert!(rerun.system_err.is_some());
    assert_eq!(
        rerun.system_err.as_deref(),
        Some("First stderr\nSecond stderr"),
        "Expected all system-err content"
    );

    assert!(rerun.stack_trace.is_none());
}

// Test parsing the example junit.xml provided earlier which contains reruns
#[test]
fn test_parse_nextest_junit_xml_shortened() {
    let xml = r#"
<?xml version="1.0" encoding="UTF-8"?>
<testsuites name="nextest-run" tests="3" failures="1" errors="0" uuid="45c50042-482e-477e-88a2-60cfcc3eaf95" timestamp="2024-01-09T07:50:12.664+00:00" time="0.023">
    <testsuite name="nextest-tests::basic" tests="3" disabled="0" errors="0" failures="1">
        <testcase name="test_cwd" classname="nextest-tests::basic" timestamp="2024-01-09T07:50:12.665+00:00" time="0.004">
        </testcase>
        <testcase name="test_failure_assert" classname="nextest-tests::basic" timestamp="2024-01-09T07:50:12.665+00:00" time="0.004">
            <failure type="test failure">details</failure>
            <rerunFailure timestamp="2024-01-09T07:50:12.670+00:00" time="0.004" type="test failure">details</rerunFailure>
            <rerunFailure timestamp="2024-01-09T07:50:12.676+00:00" time="0.004" type="test failure">details</rerunFailure>
            <system-out>details</system-out>
            <system-err>details</system-err>
        </testcase>
        <testcase name="test_flaky_mod_4" classname="nextest-tests::basic" timestamp="2024-01-09T07:50:12.683+00:00" time="0.004">
            <flakyFailure timestamp="2024-01-09T07:50:12.665+00:00" time="0.004" type="test failure">details</flakyFailure>
            <flakyFailure timestamp="2024-01-09T07:50:12.671+00:00" time="0.004" type="test failure">details</flakyFailure>
            <flakyFailure timestamp="2024-01-09T07:50:12.676+00:00" time="0.005" type="test failure">details</flakyFailure>
        </testcase>
    </testsuite>
</testsuites>
"#;
    let cursor = Cursor::new(xml);
    let suites = junit_parser::from_reader(cursor).expect("Failed to parse JUnit XML");

    assert_eq!(suites.suites.len(), 1);
    let suite = &suites.suites[0];
    assert_eq!(suite.cases.len(), 3);

    // Test case 1: test_cwd
    let case_cwd = &suite.cases[0];
    assert_eq!(case_cwd.name, "nextest-tests::basic::test_cwd");
    // Reason: No <failure>, <error>, or <skipped> tag present, defaults to success.
    assert!(case_cwd.status.is_success(), "'test_cwd' should be success");

    // Test case 2: test_failure_assert
    let case_fail = &suite.cases[1];
    assert_eq!(case_fail.name, "nextest-tests::basic::test_failure_assert");
    // Reason: Contains a primary <failure> tag, determining its status.
    assert!(
        case_fail.status.is_failure(),
        "'test_failure_assert' should be failure"
    );

    // Test case 3: test_flaky_mod_4
    let case_flaky = &suite.cases[2];
    assert_eq!(case_flaky.name, "nextest-tests::basic::test_flaky_mod_4");
    // Reason: Contains only <flakyFailure> tags (reruns). No primary <failure>, <error>, or <skipped> tag means it defaults to success.
    assert!(
        case_flaky.status.is_success(),
        "'test_flaky_mod_4' should be success (flaky pass)"
    );
}
