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

#[derive(Debug)]
pub struct TestFailure {
    pub message: String,
    pub text: String,
    pub failure_type: String,
}
impl TestFailure {
    fn new() -> Self {
        Self {
            message: String::new(),
            text: String::new(),
            failure_type: String::new(),
        }
    }
    fn parse_attributes<'a>(&mut self, e: &'a XMLBytesStart) -> Result<(), JunitParserError> {
        for a in e.attributes() {
            let a = a?;
            match a.key {
                b"type" => self.failure_type = try_from_attribute_value_string(a.value)?,
                b"message" => self.message = try_from_attribute_value_string(a.value)?,
                _ => {}
            };
        }
        Ok(())
    }

    fn new_empty<'a>(e: &'a XMLBytesStart) -> Result<Self, JunitParserError> {
        let mut tf = Self::new();
        tf.parse_attributes(e)?;
        Ok(tf)
    }

    fn new_from_reader<'a, B: BufRead>(
        e: &'a XMLBytesStart,
        r: &mut XMLReader<B>,
    ) -> Result<Self, JunitParserError> {
        let mut tf = Self::new();
        tf.parse_attributes(e)?;
        let mut buf = Vec::new();
        loop {
            match r.read_event(&mut buf) {
                Ok(XMLEvent::End(ref e)) if e.name() == b"failure" => break,
                Ok(XMLEvent::Text(e)) => {
                    tf.text = e.unescape_and_decode(&r)?.trim().to_string();
                }
                Ok(XMLEvent::Eof) => {
                    return Err(XMLError::UnexpectedEof("failure".to_string()).into())
                }
                Err(err) => return Err(err.into()),
                _ => (),
            }
        }
        buf.clear();
        Ok(tf)
    }
}

#[derive(Debug)]
pub struct TestError {
    pub message: String,
    pub text: String,
    pub error_type: String,
}
impl TestError {
    fn new() -> Self {
        Self {
            message: String::new(),
            text: String::new(),
            error_type: String::new(),
        }
    }
    fn parse_attributes<'a>(&mut self, e: &'a XMLBytesStart) -> Result<(), JunitParserError> {
        for a in e.attributes() {
            let a = a?;
            match a.key {
                b"type" => self.error_type = try_from_attribute_value_string(a.value)?,
                b"message" => self.message = try_from_attribute_value_string(a.value)?,
                _ => {}
            };
        }
        Ok(())
    }

    fn new_empty<'a>(e: &'a XMLBytesStart) -> Result<Self, JunitParserError> {
        let mut te = Self::new();
        te.parse_attributes(e)?;
        Ok(te)
    }

    fn new_from_reader<'a, B: BufRead>(
        e: &'a XMLBytesStart,
        r: &mut XMLReader<B>,
    ) -> Result<Self, JunitParserError> {
        let mut te = Self::new();
        te.parse_attributes(e)?;
        let mut buf = Vec::new();
        loop {
            match r.read_event(&mut buf) {
                Ok(XMLEvent::End(ref e)) if e.name() == b"error" => break,
                Ok(XMLEvent::Text(e)) => {
                    te.text = e.unescape_and_decode(&r)?.trim().to_string();
                }
                Ok(XMLEvent::Eof) => {
                    return Err(XMLError::UnexpectedEof("error".to_string()).into())
                }
                Err(err) => return Err(err.into()),
                _ => (),
            }
        }
        buf.clear();
        Ok(te)
    }
}

#[derive(Debug)]
pub struct TestSkipped {
    pub message: String,
    pub text: String,
    pub skipped_type: String,
}
impl TestSkipped {
    fn new() -> Self {
        Self {
            message: String::new(),
            text: String::new(),
            skipped_type: String::new(),
        }
    }
    fn parse_attributes<'a>(&mut self, e: &'a XMLBytesStart) -> Result<(), JunitParserError> {
        for a in e.attributes() {
            let a = a?;
            match a.key {
                b"type" => self.skipped_type = try_from_attribute_value_string(a.value)?,
                b"message" => self.message = try_from_attribute_value_string(a.value)?,
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
                Ok(XMLEvent::End(ref e)) if e.name() == b"skipped" => break,
                Ok(XMLEvent::Text(e)) => {
                    ts.text = e.unescape_and_decode(&r)?.trim().to_string();
                }
                Ok(XMLEvent::Eof) => {
                    return Err(XMLError::UnexpectedEof("skipped".to_string()).into())
                }
                Err(err) => return Err(err.into()),
                _ => (),
            }
        }
        buf.clear();
        Ok(ts)
    }
}

#[derive(Debug)]
pub enum TestStatus {
    Success,
    Error(TestError),
    Failure(TestFailure),
    Skipped(TestSkipped),
}
impl TestStatus {
    pub fn is_success(&self) -> bool {
        match self {
            TestStatus::Success => true,
            _ => false,
        }
    }
    pub fn is_error(&self) -> bool {
        match self {
            TestStatus::Error(_) => true,
            _ => false,
        }
    }
    pub fn error_as_ref<'a>(&'a self) -> &'a TestError {
        if let TestStatus::Error(ref e) = self {
            return e;
        }
        panic!("called `TestStatus::error()` on a value that is not TestStatus::Error(_)");
    }

    pub fn is_failure(&self) -> bool {
        match self {
            TestStatus::Failure(_) => true,
            _ => false,
        }
    }
    pub fn failure_as_ref<'a>(&'a self) -> &'a TestFailure {
        if let TestStatus::Failure(ref e) = self {
            return e;
        }
        panic!("called `TestStatus::failure()` on a value that is not TestStatus::Failure(_)");
    }

    pub fn is_skipped(&self) -> bool {
        match self {
            TestStatus::Skipped(_) => true,
            _ => false,
        }
    }
    pub fn skipped_as_ref<'a>(&'a self) -> &'a TestSkipped {
        if let TestStatus::Skipped(ref e) = self {
            return e;
        }
        panic!("called `TestStatus::skipped()` on a value that is not TestStatus::Skipped(_)");
    }
}

#[derive(Debug)]
pub struct TestCase {
    pub time: f64,
    pub name: String,
    pub status: TestStatus,
}
impl TestCase {
    fn new() -> Self {
        Self {
            time: 0f64,
            name: String::new(),
            status: TestStatus::Success,
        }
    }
    fn parse_attributes<'a>(&mut self, e: &'a XMLBytesStart) -> Result<(), JunitParserError> {
        for a in e.attributes() {
            let a = a?;
            match a.key {
                b"time" => self.time = try_from_attribute_value_f64(a.value)?,
                b"name" => self.name = try_from_attribute_value_string(a.value)?,
                _ => {}
            };
        }
        Ok(())
    }

    fn new_empty<'a>(e: &'a XMLBytesStart) -> Result<Self, JunitParserError> {
        let mut tc = Self::new();
        tc.parse_attributes(e)?;
        Ok(tc)
    }

    fn new_from_reader<'a, B: BufRead>(
        e: &'a XMLBytesStart,
        r: &mut XMLReader<B>,
    ) -> Result<Self, JunitParserError> {
        let mut tc = Self::new();
        tc.parse_attributes(e)?;
        let mut buf = Vec::new();
        loop {
            match r.read_event(&mut buf) {
                Ok(XMLEvent::End(ref e)) if e.name() == b"testcase" => break,
                Ok(XMLEvent::Start(ref e)) if e.name() == b"skipped" => {
                    let ts = TestSkipped::new_from_reader(e, r)?;
                    tc.status = TestStatus::Skipped(ts);
                }
                Ok(XMLEvent::Empty(ref e)) if e.name() == b"skipped" => {
                    let ts = TestSkipped::new_empty(e)?;
                    tc.status = TestStatus::Skipped(ts);
                }
                Ok(XMLEvent::Start(ref e)) if e.name() == b"failure" => {
                    let tf = TestFailure::new_from_reader(e, r)?;
                    tc.status = TestStatus::Failure(tf);
                }
                Ok(XMLEvent::Empty(ref e)) if e.name() == b"failure" => {
                    let tf = TestFailure::new_empty(e)?;
                    tc.status = TestStatus::Failure(tf);
                }
                Ok(XMLEvent::Start(ref e)) if e.name() == b"error" => {
                    let te = TestError::new_from_reader(e, r)?;
                    tc.status = TestStatus::Error(te);
                }
                Ok(XMLEvent::Empty(ref e)) if e.name() == b"error" => {
                    let te = TestError::new_empty(e)?;
                    tc.status = TestStatus::Error(te);
                }
                Ok(XMLEvent::Eof) => {
                    return Err(XMLError::UnexpectedEof("testcase".to_string()).into())
                }
                Err(err) => return Err(err.into()),
                _ => (),
            }
        }
        buf.clear();
        Ok(tc)
    }
}

#[derive(Debug)]
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
                Ok(XMLEvent::Start(ref e)) if e.name() == b"testcase" => {
                    let testcase = TestCase::new_from_reader(e, r)?;
                    if ts.cases.contains_key(&testcase.name) {
                        return Err(JunitParserError::DuplicateError {
                            kind: "testcase".to_string(),
                            name: testcase.name,
                        });
                    }
                    ts.cases.insert(testcase.name.clone(), testcase);
                }
                Ok(XMLEvent::Empty(ref e)) if e.name() == b"testcase" => {
                    let testcase = TestCase::new_empty(e)?;
                    if ts.cases.contains_key(&testcase.name) {
                        return Err(JunitParserError::DuplicateError {
                            kind: "testcase".to_string(),
                            name: testcase.name,
                        });
                    }
                    ts.cases.insert(testcase.name.clone(), testcase);
                }
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

#[derive(Debug)]
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
            Ok(XMLEvent::Empty(ref e)) if e.name() == b"testsuite" => {
                let ts = TestSuite::new_empty(e)?;
                let mut suites = TestSuites::new();
                suites.suites.insert(ts.name.clone(), ts);
                return Ok(suites);
            }
            Ok(XMLEvent::Start(ref e)) if e.name() == b"testsuite" => {
                let ts = TestSuite::new_from_reader(e, &mut r)?;
                let mut suites = TestSuites::new();
                suites.suites.insert(ts.name.clone(), ts);
                return Ok(suites);
            }
            Ok(XMLEvent::Eof) => {
                return Err(XMLError::UnexpectedEof("testsuites".to_string()).into())
            }
            Err(err) => return Err(err.into()),
            _ => (),
        }
    }
}
