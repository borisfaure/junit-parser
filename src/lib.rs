//! Library to parse JUnit XML files

use quick_xml::events::Event as XMLEvent;
use quick_xml::Error as XMLError;
use quick_xml::Reader as XMLReader;
use std::collections::HashMap;
use std::io::prelude::*;

pub struct TestCase {}

pub struct TestSuite {}

pub struct TestSuites {
    pub suites: HashMap<String, TestSuite>,
}
impl TestSuites {
    fn new() -> Self {
        Self {
            suites: HashMap::new(),
        }
    }
}

pub fn from_reader<B: BufRead>(reader: B) -> Result<TestSuites, XMLError> {
    let mut r = XMLReader::from_reader(reader);
    let mut buf = Vec::new();
    let test_suites = TestSuites::new();
    loop {
        match r.read_event(&mut buf) {
            Ok(XMLEvent::Start(ref e)) => match e.name() {
                b"testsuites" => {}
                b"testsuite" => {}
                _ => (),
            },
            Ok(XMLEvent::Eof) => break,
            Err(e) => return Err(e),
            _ => (),
        }
        buf.clear();
    }
    Ok(test_suites)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn empty_test_suites() {
        let xml = r#"<testsuites/>"#;
        let cursor = Cursor::new(xml);
        let r = from_reader(cursor);
        assert!(r.is_ok());
        let t = r.unwrap();
        assert_eq!(t.suites.len(), 0);
    }

    #[test]
    fn empty_test_suites_start_end() {
        let xml = r#"<testsuites></testsuites>"#;
        let cursor = Cursor::new(xml);
        let r = from_reader(cursor);
        assert!(r.is_ok());
        let t = r.unwrap();
        assert_eq!(t.suites.len(), 0);
    }
}
