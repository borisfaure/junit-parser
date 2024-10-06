//! Test the properties parsing

use junit_parser;
use std::io::Cursor;

#[test]
fn test_properties_empty() {
    let xml = r#"
<testsuite>
  <properties />
  <testcase name="ASuccessfulTest">
    <properties />
  </testcase>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
}

#[test]
fn test_properties_no_content() {
    let xml = r#"
<testsuite>
  <properties></properties>
  <testcase name="ASuccessfulTest">
    <properties></properties>
  </testcase>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
}

#[test]
fn test_property_no_name() {
    let xml = r#"
<testsuite>
  <properties>
    <property />
  </properties>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_err());

    let xml = r#"
<testsuite>
  <properties>
    <property name />
  </properties>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_err());
}

#[cfg(feature = "properties_as_hashmap")]
#[test]
fn test_properties_duplicates_hashmap() {
    let xml = r#"
<testsuite>
  <properties>
        <property name="language" value="english" />
        <property name="author">
        Me
        </property>
        <property name="step" value="First step" />
        <property name="step" value="Second step" />
  </properties>
  <testcase name="ASuccessfulTest">
    <properties>
        <property name="language" value="gibberish" />
        <property name="author">
        John Doe
        </property>
        <property name="step" value="1st step" />
        <property name="step" value="2nd step" />
    </properties>
  </testcase>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let ts = &t.suites[0];
    assert_eq!(ts.properties.hashmap.len(), 3);
    assert_eq!(
        ts.properties.hashmap.get(&"language".to_string()),
        Some(&"english".to_string())
    );
    assert_eq!(
        ts.properties.hashmap.get(&"author".to_string()),
        Some(&"Me".to_string())
    );
    assert_eq!(
        ts.properties.hashmap.get(&"step".to_string()),
        Some(&"Second step".to_string())
    );
    assert_eq!(ts.cases.len(), 1);
    let tc = &ts.cases[0];
    assert_eq!(tc.properties.hashmap.len(), 3);
    assert_eq!(
        tc.properties.hashmap.get(&"language".to_string()),
        Some(&"gibberish".to_string())
    );
    assert_eq!(
        tc.properties.hashmap.get(&"author".to_string()),
        Some(&"John Doe".to_string())
    );
    assert_eq!(
        tc.properties.hashmap.get(&"step".to_string()),
        Some(&"2nd step".to_string())
    );
}

#[cfg(feature = "properties_as_vector")]
#[test]
fn test_properties_duplicates_vec() {
    let xml = r#"
<testsuite>
  <properties>
        <property name="language" value="english" />
        <property name="author">
        Me
        </property>
        <property name="step" value="First step" />
        <property name="step" value="Second step" />
  </properties>
  <testcase name="ASuccessfulTest">
    <properties>
        <property name="language" value="gibberish" />
        <property name="author">
        John Doe
        </property>
        <property name="step" value="1st step" />
        <property name="step" value="2nd step" />
    </properties>
  </testcase>
</testsuite>
"#;
    let cursor = Cursor::new(xml);
    let r = junit_parser::from_reader(cursor);
    assert!(r.is_ok());
    let t = r.unwrap();
    assert_eq!(t.suites.len(), 1);
    let ts = &t.suites[0];
    assert_eq!(ts.properties.vec.len(), 4);
    assert_eq!(
        ts.properties.vec[0],
        ("language".to_string(), "english".to_string())
    );
    assert_eq!(
        ts.properties.vec[1],
        ("author".to_string(), "Me".to_string())
    );
    assert_eq!(
        ts.properties.vec[2],
        ("step".to_string(), "First step".to_string())
    );
    assert_eq!(
        ts.properties.vec[3],
        ("step".to_string(), "Second step".to_string())
    );
    assert_eq!(ts.cases.len(), 1);
    let tc = &ts.cases[0];
    assert_eq!(tc.properties.hashmap.len(), 3);
    assert_eq!(
        tc.properties.vec[0],
        ("language".to_string(), "gibberish".to_string())
    );
    assert_eq!(
        tc.properties.vec[1],
        ("author".to_string(), "John Doe".to_string())
    );
    assert_eq!(
        tc.properties.vec[2],
        ("step".to_string(), "1st step".to_string())
    );
    assert_eq!(
        tc.properties.vec[3],
        ("step".to_string(), "2nd step".to_string())
    );
}
