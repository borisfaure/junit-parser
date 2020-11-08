//! Library to parse JUnit XML files

mod errors;

use errors::JunitParserError;
use quick_xml::events::BytesStart as XMLBytesStart;
use quick_xml::events::Event as XMLEvent;
use quick_xml::Error as XMLError;
use quick_xml::Reader as XMLReader;
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::prelude::*;
use std::str;

pub struct TestCase {}

pub struct TestSuite {
    pub cases: HashMap<String, TestCase>,
    pub time: f64,
    pub tests: u64,
    pub errors: u64,
    pub failures: u64,
    pub skipped: u64,
    pub name: String,
}
impl TestSuite {
    fn new() -> Self {
        Self {
            cases: HashMap::new(),
            time: 0f64,
            tests: 0u64,
            errors: 0u64,
            failures: 0u64,
            skipped: 0u64,
            name: String::new(),
        }
    }
    fn parse_attributes<'a>(&mut self, e: &'a XMLBytesStart) -> Result<(), JunitParserError> {
        for a in e.attributes() {
            let a = a?;
            match a.key {
                b"time" => self.time = try_from_attribute_value_f64(a.value)?,
                b"tests" => self.tests = try_from_attribute_value_u64(a.value)?,
                b"errors" => self.errors = try_from_attribute_value_u64(a.value)?,
                b"failures" => self.failures = try_from_attribute_value_u64(a.value)?,
                b"skipped" => self.skipped = try_from_attribute_value_u64(a.value)?,
                b"name" => self.name = try_from_attribute_value_string(a.value)?,
                _ => {}
            };
        }
        Ok(())
    }

    fn new_empty<'a>(e: &'a XMLBytesStart) -> Result<Self, JunitParserError> {
        let mut ts = Self::new();
        ts.parse_attributes(e)?;
        Ok(ts)
    }

    fn new_from_reader<'a, B: BufRead>(
        e: &'a XMLBytesStart,
        r: &mut XMLReader<B>,
    ) -> Result<Self, JunitParserError> {
        let mut ts = Self::new();
        ts.parse_attributes(e)?;
        let mut buf = Vec::new();
        loop {
            match r.read_event(&mut buf) {
                Ok(XMLEvent::End(ref e)) if e.name() == b"testsuite" => break,
                Ok(XMLEvent::Eof) => {
                    return Err(XMLError::UnexpectedEof("testsuite".to_string()).into())
                }
                Err(err) => return Err(err.into()),
                _ => (),
            }
        }
        buf.clear();
        Ok(ts)
    }
}

pub struct TestSuites {
    pub suites: HashMap<String, TestSuite>,
    pub time: f64,
    pub tests: u64,
    pub errors: u64,
    pub failures: u64,
    pub skipped: u64,
    pub name: String,
}
impl TestSuites {
    fn new() -> Self {
        Self {
            suites: HashMap::new(),
            time: 0f64,
            tests: 0u64,
            errors: 0u64,
            failures: 0u64,
            skipped: 0u64,
            name: String::new(),
        }
    }

    fn parse_attributes<'a>(&mut self, e: &'a XMLBytesStart) -> Result<(), JunitParserError> {
        for a in e.attributes() {
            let a = a?;
            match a.key {
                b"time" => self.time = try_from_attribute_value_f64(a.value)?,
                b"tests" => self.tests = try_from_attribute_value_u64(a.value)?,
                b"errors" => self.errors = try_from_attribute_value_u64(a.value)?,
                b"failures" => self.failures = try_from_attribute_value_u64(a.value)?,
                b"skipped" => self.skipped = try_from_attribute_value_u64(a.value)?,
                b"name" => self.name = try_from_attribute_value_string(a.value)?,
                _ => {}
            };
        }
        Ok(())
    }

    fn new_empty<'a>(e: &'a XMLBytesStart) -> Result<Self, JunitParserError> {
        let mut ts = Self::new();
        ts.parse_attributes(e)?;
        Ok(ts)
    }

    fn new_from_reader<'a, B: BufRead>(
        e: &'a XMLBytesStart,
        r: &mut XMLReader<B>,
    ) -> Result<Self, JunitParserError> {
        let mut ts = Self::new();
        ts.parse_attributes(e)?;
        let mut buf = Vec::new();
        loop {
            match r.read_event(&mut buf) {
                Ok(XMLEvent::End(ref e)) if e.name() == b"testsuites" => break,
                Ok(XMLEvent::Start(ref e)) if e.name() == b"testsuite" => {
                    let suite = TestSuite::new_from_reader(e, r)?;
                    if ts.suites.contains_key(&suite.name) {
                        return Err(JunitParserError::DuplicateError {
                            kind: "testsuite".to_string(),
                            name: suite.name,
                        });
                    }
                    ts.suites.insert(suite.name.clone(), suite);
                }
                Ok(XMLEvent::Empty(ref e)) if e.name() == b"testsuite" => {
                    let suite = TestSuite::new_empty(e)?;
                    if ts.suites.contains_key(&suite.name) {
                        return Err(JunitParserError::DuplicateError {
                            kind: "testsuite".to_string(),
                            name: suite.name,
                        });
                    }
                    ts.suites.insert(suite.name.clone(), suite);
                }
                Ok(XMLEvent::Eof) => {
                    return Err(XMLError::UnexpectedEof("testsuites".to_string()).into())
                }
                Err(err) => return Err(err.into()),
                _ => (),
            }
        }
        buf.clear();
        Ok(ts)
    }
}

fn try_from_attribute_value_f64<'a>(value: Cow<'a, [u8]>) -> Result<f64, JunitParserError> {
    match value {
        Cow::Borrowed(b) => {
            let s = str::from_utf8(b)?;
            if s.len() == 0 {
                return Ok(0f64);
            }
            Ok(s.parse::<f64>()?)
        }
        Cow::Owned(ref b) => {
            let s = str::from_utf8(b)?;
            if s.len() == 0 {
                return Ok(0f64);
            }
            Ok(s.parse::<f64>()?)
        }
    }
}

fn try_from_attribute_value_u64<'a>(value: Cow<'a, [u8]>) -> Result<u64, JunitParserError> {
    match value {
        Cow::Borrowed(b) => {
            let s = str::from_utf8(b)?;
            if s.len() == 0 {
                return Ok(0u64);
            }
            Ok(s.parse::<u64>()?)
        }
        Cow::Owned(ref b) => {
            let s = str::from_utf8(b)?;
            if s.len() == 0 {
                return Ok(0u64);
            }
            Ok(s.parse::<u64>()?)
        }
    }
}

fn try_from_attribute_value_string<'a>(value: Cow<'a, [u8]>) -> Result<String, JunitParserError> {
    match value {
        Cow::Borrowed(b) => Ok(str::from_utf8(b)?.to_owned()),
        Cow::Owned(ref b) => Ok(str::from_utf8(b)?.to_owned()),
    }
}

pub fn from_reader<B: BufRead>(reader: B) -> Result<TestSuites, JunitParserError> {
    let mut r = XMLReader::from_reader(reader);
    let mut buf = Vec::new();
    loop {
        match r.read_event(&mut buf) {
            Ok(XMLEvent::Empty(ref e)) if e.name() == b"testsuites" => {
                return TestSuites::new_empty(e);
            }
            Ok(XMLEvent::Start(ref e)) if e.name() == b"testsuites" => {
                return TestSuites::new_from_reader(e, &mut r);
            }
            Ok(XMLEvent::Eof) => {
                return Err(XMLError::UnexpectedEof("testsuites".to_string()).into())
            }
            Err(err) => return Err(err.into()),
            _ => (),
        }
    }
}
