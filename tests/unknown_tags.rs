//! Test unknown tags

use junit_parser::{self, TestStatus};
use std::io::Cursor;

#[test]
fn test_unknown_empty_element_in_testcase() {
    let xml = r#"
    <testsuite tests="1">
    <testcase classname="some.class" name="SomeTest">
     <unknown_empty_element />
    </testcase>
    </testsuite>
    "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let suite = &t.suites[0];
    assert_eq!(suite.cases.len(), 1);
}

#[test]
fn test_unknown_tags_in_testcase() {
    let xml = r#"
    <testsuite tests="1">
    <testcase classname="some.class" name="SomeTest">
    <unknown_element_with_content>
    <with><quite><some><content>content here</content></some></quite></with>
    </unknown_element_with_content>
    </testcase>
    </testsuite>
    "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let suite = &t.suites[0];
    assert_eq!(suite.cases.len(), 1);
}

// --- New Test Implementations ---

// Test parsing an unknown empty element directly within a <testsuite>
#[test]
fn test_unknown_empty_element_in_testsuite() {
    let xml = r#"
    <testsuite tests="1" name="Suite1">
      <testcase name="Case1"/>
      <unknown_empty />
      <testcase name="Case2"/>
    </testsuite>
    "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);

    assert!(
        r.is_ok(),
        "Parsing should succeed even with unknown empty elements in testsuite"
    );

    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let suite = &t.suites[0];
    assert_eq!(suite.cases.len(), 2, "Should parse known test cases");
}

// Test parsing an unknown element with nested content directly within a <testsuite>
#[test]
fn test_unknown_element_with_content_in_testsuite() {
    let xml = r#"
    <testsuite tests="1" name="Suite1">
      <testcase name="Case1"/>
      <unknown_parent>
        <nested_tag attr="val">Some text</nested_tag>
      </unknown_parent>
      <testcase name="Case2"/>
    </testsuite>
    "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);

    assert!(
        r.is_ok(),
        "Parsing should succeed by skipping unknown elements with content"
    );

    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let suite = &t.suites[0];
    assert_eq!(suite.cases.len(), 2, "Should still parse known test cases");
}

// Test parsing an unknown empty element directly within a <testsuites> element
#[test]
fn test_unknown_empty_element_in_testsuites() {
    let xml = r#"
    <testsuites name="Root">
        <testsuite name="Suite1">
            <testcase name="Case1"/>
        </testsuite>
        <unknown_sibling_empty />
    </testsuites>
    "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);

    assert!(
        r.is_ok(),
        "Parsing should succeed even with unknown empty elements in testsuites"
    );

    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1, "Should parse known testsuite sibling");
    assert_eq!(t.suites[0].name, "Suite1");
}

// Test parsing an unknown element with nested content directly within a <testsuites> element
#[test]
fn test_unknown_element_with_content_in_testsuites() {
    let xml = r#"
    <testsuites name="Root">
        <testsuite name="Suite1">
            <testcase name="Case1"/>
        </testsuite>
        <unknown_sibling_with_content>
            <some_data key="value"/>
            Some text content
        </unknown_sibling_with_content>
        <testsuite name="Suite2">
             <testcase name="Case2"/>
        </testsuite>
    </testsuites>
    "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);

    assert!(
        r.is_ok(),
        "Parsing should succeed by skipping unknown elements with content in testsuites"
    );

    let t = r.unwrap();
    assert_eq!(t.suites.len(), 2, "Should parse known testsuite siblings");
    assert_eq!(t.suites[0].name, "Suite1");
    assert_eq!(t.suites[1].name, "Suite2");
}

// Test parsing an unknown empty element within a <failure> tag
#[test]
fn test_unknown_empty_element_in_failure() {
    let xml = r#"
    <testsuite tests="1" failures="1">
        <testcase name="Test1">
            <failure message="Failed">
                Some failure text.
                <unknown_empty_in_failure />
                More failure text.
            </failure>
        </testcase>
    </testsuite>
    "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);

    assert!(
        r.is_ok(),
        "Parsing should succeed with unknown empty tag in failure"
    );

    let t = r.unwrap();
    let case = &t.suites[0].cases[0];
    assert!(case.status.is_failure());

    if let junit_parser::TestStatus::Failure(failure) = &case.status {
        // Expect concatenated text, unknown tag ignored
        let expected_text = "Some failure text.\nMore failure text.";
        assert_eq!(
            failure.text.trim(),
            expected_text.trim(),
            "Failure text should be parsed correctly, ignoring unknown tag"
        );
    } else {
        panic!("Expected TestStatus::Failure");
    }
}

// Test parsing an unknown element with content within an <error> tag
#[test]
fn test_unknown_element_with_content_in_error() {
    let xml = r#"
    <testsuite tests="1" errors="1">
        <testcase name="Test1">
            <error message="ErrorOccurred" type="ErrorType">
                Some error text.
                <unknown_with_content>
                    <nested>Ignored</nested>
                </unknown_with_content>
                More error text.
            </error>
        </testcase>
    </testsuite>
    "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);

    assert!(
        r.is_ok(),
        "Parsing should succeed with unknown tag with content in error"
    );

    let t = r.unwrap();
    let case = &t.suites[0].cases[0];
    assert!(case.status.is_error());

    if let junit_parser::TestStatus::Error(error) = &case.status {
        // Expect concatenated text, unknown tag ignored
        let expected_text = "Some error text.\nMore error text.";
        assert_eq!(
            error.text.trim(),
            expected_text.trim(),
            "Error text should be parsed correctly, ignoring unknown tag"
        );
    } else {
        panic!("Expected TestStatus::Error");
    }
}

// Test parsing an unknown empty element within a <skipped> tag
#[test]
fn test_unknown_empty_element_in_skipped() {
    let xml = r#"
    <testsuite tests="1" skipped="1">
        <testcase name="Test1">
            <skipped message="SkippedReason">
                Some skip text.
                <unknown_empty_in_skipped />
                More skip text.
            </skipped>
        </testcase>
    </testsuite>
    "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);

    assert!(
        r.is_ok(),
        "Parsing should succeed with unknown empty tag in skipped"
    );

    let t = r.unwrap();
    let case = &t.suites[0].cases[0];
    assert!(case.status.is_skipped());

    if let junit_parser::TestStatus::Skipped(skipped) = &case.status {
        // Expect concatenated text, unknown tag ignored
        let expected_text = "Some skip text.\nMore skip text.";
        assert_eq!(
            skipped.text.trim(),
            expected_text.trim(),
            "Skipped text should be parsed correctly, ignoring unknown tag"
        );
        assert_eq!(skipped.message, "SkippedReason");
    } else {
        panic!("Expected TestStatus::Skipped");
    }
}

// Test parsing an unknown element with content within a <flakyFailure> tag
#[test]
fn test_unknown_element_with_content_in_rerun_or_flaky() {
    let xml = r#"
    <testsuite tests="1">
        <testcase name="Test1">
            <flakyFailure type="FlakyType" message="FlakyMsg">
                Some flaky text.
                <unknown_in_rerun>
                    <stuff/>
                    More ignored stuff.
                </unknown_in_rerun>
                More flaky text.
            </flakyFailure>
        </testcase>
    </testsuite>
    "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);

    assert!(
        r.is_ok(),
        "Parsing should succeed with unknown tag with content in rerun/flaky"
    );

    let t = r.unwrap();
    let case = &t.suites[0].cases[0];
    assert!(case.status.is_success());
    assert_eq!(case.reruns.len(), 1);

    let rerun = &case.reruns[0];
    assert!(matches!(
        rerun.kind,
        junit_parser::RerunOrFlakyKind::FlakyFailure
    ));
    // Expect concatenated text, unknown tag ignored
    let expected_text = "Some flaky text.\nMore flaky text.";
    assert_eq!(
        rerun.text.trim(),
        expected_text.trim(),
        "Rerun/flaky text should be parsed correctly, ignoring unknown tag"
    );
    assert_eq!(rerun.message, "FlakyMsg");
    assert_eq!(rerun.rerun_type, "FlakyType");
}

// Test parsing an unknown element as a sibling to <testsuites>
// This tests tags before/after the main <testsuites> which should be ignored
#[test]
fn test_unknown_element_sibling_to_testsuites() {
    let xml = r#"
    <?xml version="1.0"?>
    <ignored_before />
    <testsuites name="Root">
        <testsuite name="Suite1">
            <testcase name="Case1"/>
        </testsuite>
    </testsuites>
    <ignored_after>data</ignored_after>
    "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);

    assert!(
        r.is_ok(),
        "Parsing should succeed ignoring elements outside <testsuites>"
    );

    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1); // Only the content within <testsuites> is parsed
    assert_eq!(t.suites[0].name, "Suite1");
    assert_eq!(t.name, "Root"); // Name from <testsuites> attribute
}

// Test parsing an unknown empty element directly within a <properties>
#[test]
#[cfg(feature = "properties_as_vector")] // Test requires property storage
fn test_unknown_empty_element_in_properties() {
    let xml = r#"
    <testsuite tests="1">
        <testcase name="Test1">
            <properties>
                <property name="prop1" value="val1"/>
                <unknown_empty />
                <property name="prop2" value="val2"/>
            </properties>
        </testcase>
    </testsuite>
    "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(
        r.is_ok(),
        "Parsing should succeed with unknown empty tag in properties"
    );

    let t = r.unwrap();
    let props = &t.suites[0].cases[0].properties;
    assert_eq!(props.vec.len(), 2, "Should parse known properties");
    assert_eq!(props.vec[0], ("prop1".to_string(), "val1".to_string()));
    assert_eq!(props.vec[1], ("prop2".to_string(), "val2".to_string()));
}

// Test parsing an unknown element with nested content directly within a <properties>
#[test]
#[cfg(feature = "properties_as_vector")] // Test requires property storage
fn test_unknown_element_with_content_in_properties() {
    let xml = r#"
    <testsuite tests="1">
        <testcase name="Test1">
            <properties>
                <property name="prop1" value="val1"/>
                <unknown_with_content><data/></unknown_with_content>
                <property name="prop2" value="val2"/>
            </properties>
        </testcase>
    </testsuite>
    "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(
        r.is_ok(),
        "Parsing should succeed with unknown tag with content in properties"
    );

    let t = r.unwrap();
    let props = &t.suites[0].cases[0].properties;
    assert_eq!(props.vec.len(), 2, "Should parse known properties");
    assert_eq!(props.vec[0], ("prop1".to_string(), "val1".to_string()));
    assert_eq!(props.vec[1], ("prop2".to_string(), "val2".to_string()));
}

// Test parsing an unknown empty element within a <property> tag body
#[test]
#[cfg(feature = "properties_as_vector")] // Test requires property storage
fn test_unknown_empty_element_in_property() {
    let xml = r#"
    <testsuite tests="1">
        <testcase name="Test1">
            <properties>
                <property name="prop1">
                    Value Part 1
                    <unknown_empty_in_prop />
                    Value Part 2
                </property>
            </properties>
        </testcase>
    </testsuite>
    "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(
        r.is_ok(),
        "Parsing should succeed with unknown empty tag in property body"
    );

    let t = r.unwrap();
    let props = &t.suites[0].cases[0].properties;
    assert_eq!(props.vec.len(), 1);
    // Expect concatenated text value
    let expected_value = "Value Part 1\nValue Part 2";
    assert_eq!(props.vec[0].0, "prop1");
    assert_eq!(
        props.vec[0].1.trim(),
        expected_value.trim(),
        "Property value should be parsed correctly, ignoring unknown tag"
    );
}

// Test parsing an unknown element with content within a <property> tag body
#[test]
#[cfg(feature = "properties_as_vector")] // Test requires property storage
fn test_unknown_element_with_content_in_property() {
    let xml = r#"
    <testsuite tests="1">
        <testcase name="Test1">
            <properties>
                <property name="prop1">
                    Value Part 1
                    <unknown_content_in_prop><foo/></unknown_content_in_prop>
                    Value Part 2
                </property>
            </properties>
        </testcase>
    </testsuite>
    "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(
        r.is_ok(),
        "Parsing should succeed with unknown tag with content in property body"
    );

    let t = r.unwrap();
    let props = &t.suites[0].cases[0].properties;
    assert_eq!(props.vec.len(), 1);
    // Expect concatenated text value
    let expected_value = "Value Part 1\nValue Part 2";
    assert_eq!(props.vec[0].0, "prop1");
    assert_eq!(
        props.vec[0].1.trim(),
        expected_value.trim(),
        "Property value should be parsed correctly, ignoring unknown tag"
    );
}

// Test parsing an unknown tag within the main content of <failure>
#[test]
fn test_unknown_tag_in_failure_content() {
    // Same logic as test_unknown_empty_element_in_failure
    let xml = r#"
    <testsuite tests="1" failures="1">
        <testcase name="Test1">
            <failure message="Failed">
                Some failure text.
                <unknown_tag>Ignored</unknown_tag>
                More failure text.
            </failure>
        </testcase>
    </testsuite>
    "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    let case = &t.suites[0].cases[0];
    assert!(case.status.is_failure());
    if let TestStatus::Failure(failure) = &case.status {
        let expected_text = "Some failure text.\nMore failure text.";
        assert_eq!(failure.text.trim(), expected_text.trim());
    } else {
        panic!();
    }
}

// Test parsing an unknown tag within the main content of <error>
#[test]
fn test_unknown_tag_in_error_content() {
    // Same logic as test_unknown_element_with_content_in_error
    let xml = r#"
    <testsuite tests="1" errors="1">
        <testcase name="Test1">
            <error message="Error">
                Some error text.
                <unknown_tag>Ignored</unknown_tag>
                More error text.
            </error>
        </testcase>
    </testsuite>
    "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    let case = &t.suites[0].cases[0];
    assert!(case.status.is_error());
    if let TestStatus::Error(error) = &case.status {
        let expected_text = "Some error text.\nMore error text.";
        assert_eq!(error.text.trim(), expected_text.trim());
    } else {
        panic!();
    }
}

// Test parsing an unknown tag within the main content of <skipped>
#[test]
fn test_unknown_tag_in_skipped_content() {
    // Same logic as test_unknown_empty_element_in_skipped
    let xml = r#"
    <testsuite tests="1" skipped="1">
        <testcase name="Test1">
            <skipped message="Skipped">
                Some skip text.
                 <unknown_tag>Ignored</unknown_tag>
                More skip text.
            </skipped>
        </testcase>
    </testsuite>
    "#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    let case = &t.suites[0].cases[0];
    assert!(case.status.is_skipped());
    if let TestStatus::Skipped(skipped) = &case.status {
        let expected_text = "Some skip text.\nMore skip text.";
        assert_eq!(skipped.text.trim(), expected_text.trim());
    } else {
        panic!();
    }
}

// --- Tests for unknown tags inside system-out/err ---

// Test that unknown tags inside <system-out> are ignored
#[test]
fn test_unknown_tag_inside_system_out() {
    let xml = r#"
    <testsuite tests="1">
    <testcase name="Test1">
      <system-out>
        Some output text.
        <an_unknown_tag>This should be ignored</an_unknown_tag>
        More output text.
        <another_one />
      </system-out>
    </testcase>
    </testsuite>
    "#;
    let cursor = Cursor::new(xml);
    let suites = junit_parser::from_reader(cursor).expect("Parse failed");
    let case = &suites.suites[0].cases[0];
    assert!(case.system_out.is_some());
    // Expect only the text content, tags ignored by parse_system
    let expected_out = "Some output text.\n        \n        More output text.";
    assert_eq!(
        case.system_out.as_ref().unwrap().trim(),
        expected_out.trim()
    );
}

// Test that unknown tags inside <system-err> are ignored
#[test]
fn test_unknown_tag_inside_system_err() {
    let xml = r#"
    <testsuite tests="1">
    <testcase name="Test1">
      <system-err>
        Some error text.
        <foo><bar>Nested ignored</bar></foo>
        More error text.
      </system-err>
    </testcase>
    </testsuite>
    "#;
    let cursor = Cursor::new(xml);
    let suites = junit_parser::from_reader(cursor).expect("Parse failed");
    let case = &suites.suites[0].cases[0];
    assert!(case.system_err.is_some());
    // Expect only the text content, tags ignored by parse_system
    let expected_err = "Some error text.\n        \n        More error text.";
    assert_eq!(
        case.system_err.as_ref().unwrap().trim(),
        expected_err.trim()
    );
}
