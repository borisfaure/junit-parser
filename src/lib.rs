//! Library to parse JUnit XML files

mod errors;

use errors::JunitParserError;
use quick_xml::events::Event as XMLEvent;
use quick_xml::Reader as XMLReader;
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::prelude::*;
use std::str;

pub struct TestCase {}

pub struct TestSuite {}

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
    let mut test_suites = TestSuites::new();
    loop {
        match r.read_event(&mut buf) {
            Ok(XMLEvent::Start(ref e)) | Ok(XMLEvent::Empty(ref e)) => match e.name() {
                b"testsuites" => {
                    for a in e.attributes() {
                        let a = a?;
                        match a.key {
                            b"time" => test_suites.time = try_from_attribute_value_f64(a.value)?,
                            b"tests" => test_suites.tests = try_from_attribute_value_u64(a.value)?,
                            b"errors" => {
                                test_suites.errors = try_from_attribute_value_u64(a.value)?
                            }
                            b"failures" => {
                                test_suites.failures = try_from_attribute_value_u64(a.value)?
                            }
                            b"skipped" => {
                                test_suites.skipped = try_from_attribute_value_u64(a.value)?
                            }
                            b"name" => test_suites.name = try_from_attribute_value_string(a.value)?,
                            _ => {}
                        };
                    }
                }
                b"testsuite" => {}
                _ => (),
            },
            Ok(XMLEvent::Eof) => break,
            Err(e) => return Err(e.into()),
            _ => (),
        }
        buf.clear();
    }
    Ok(test_suites)
}
