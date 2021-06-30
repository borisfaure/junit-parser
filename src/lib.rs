#![warn(missing_docs)]
//! Library to parse JUnit XML files

mod errors;

pub use errors::Error;
use quick_xml::events::BytesStart as XMLBytesStart;
use quick_xml::events::Event as XMLEvent;
use quick_xml::name::QName;
use quick_xml::Error as XMLError;
use quick_xml::Reader as XMLReader;
use std::borrow::Cow;
use std::io::prelude::*;
use std::str;
use std::vec::Vec;

#[derive(Debug, Clone, Default)]
/// Value from a `<failure />` tag
pub struct TestFailure {
    /// The `message` attribute
    pub message: String,
    /// Body of the `<failure />` tag
    pub text: String,
    /// The `type` attribute
    pub failure_type: String,
}
impl TestFailure {
    fn parse_attributes(&mut self, e: &XMLBytesStart) -> Result<(), Error> {
        for a in e.attributes() {
            let a = a?;
            match a.key {
                QName(b"type") => self.failure_type = try_from_attribute_value_string(a.value)?,
                QName(b"message") => self.message = try_from_attribute_value_string(a.value)?,
                _ => {}
            };
        }
        Ok(())
    }

    fn new_empty(e: &XMLBytesStart) -> Result<Self, Error> {
        let mut tf = Self::default();
        tf.parse_attributes(e)?;
        Ok(tf)
    }

    fn new_from_reader<B: BufRead>(e: &XMLBytesStart, r: &mut XMLReader<B>) -> Result<Self, Error> {
        let mut tf = Self::default();
        tf.parse_attributes(e)?;
        let mut buf = Vec::new();
        loop {
            match r.read_event_into(&mut buf) {
                Ok(XMLEvent::End(ref e)) if e.name() == QName(b"failure") => break,
                Ok(XMLEvent::Text(e)) => {
                    tf.text = e.unescape()?.trim().to_string();
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

#[derive(Debug, Clone, Default)]
/// Value from an `<error />` tag
pub struct TestError {
    /// The `message` attribute
    pub message: String,
    /// Body of the `<error />` tag
    pub text: String,
    /// The `type` attribute
    pub error_type: String,
}
impl TestError {
    fn parse_attributes(&mut self, e: &XMLBytesStart) -> Result<(), Error> {
        for a in e.attributes() {
            let a = a?;
            match a.key {
                QName(b"type") => self.error_type = try_from_attribute_value_string(a.value)?,
                QName(b"message") => self.message = try_from_attribute_value_string(a.value)?,
                _ => {}
            };
        }
        Ok(())
    }

    fn new_empty(e: &XMLBytesStart) -> Result<Self, Error> {
        let mut te = Self::default();
        te.parse_attributes(e)?;
        Ok(te)
    }

    fn new_from_reader<B: BufRead>(e: &XMLBytesStart, r: &mut XMLReader<B>) -> Result<Self, Error> {
        let mut te = Self::default();
        te.parse_attributes(e)?;
        let mut buf = Vec::new();
        loop {
            match r.read_event_into(&mut buf) {
                Ok(XMLEvent::End(ref e)) if e.name() == QName(b"error") => break,
                Ok(XMLEvent::Text(e)) => {
                    te.text = e.unescape()?.trim().to_string();
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

#[derive(Debug, Clone, Default)]
/// Value from a `<skipped />` tag
pub struct TestSkipped {
    /// The `message` attribute
    pub message: String,
    /// Body of the `<skipped />` tag
    pub text: String,
    /// The `type` attribute
    pub skipped_type: String,
}
impl TestSkipped {
    fn parse_attributes(&mut self, e: &XMLBytesStart) -> Result<(), Error> {
        for a in e.attributes() {
            let a = a?;
            match a.key {
                QName(b"type") => self.skipped_type = try_from_attribute_value_string(a.value)?,
                QName(b"message") => self.message = try_from_attribute_value_string(a.value)?,
                _ => {}
            };
        }
        Ok(())
    }

    fn new_empty(e: &XMLBytesStart) -> Result<Self, Error> {
        let mut ts = Self::default();
        ts.parse_attributes(e)?;
        Ok(ts)
    }

    fn new_from_reader<B: BufRead>(e: &XMLBytesStart, r: &mut XMLReader<B>) -> Result<Self, Error> {
        let mut ts = Self::default();
        ts.parse_attributes(e)?;
        let mut buf = Vec::new();
        loop {
            match r.read_event_into(&mut buf) {
                Ok(XMLEvent::End(ref e)) if e.name() == QName(b"skipped") => break,
                Ok(XMLEvent::Text(e)) => {
                    ts.text = e.unescape()?.trim().to_string();
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

#[derive(Debug, Clone)]
/// Status of a test case
pub enum TestStatus {
    /// Success
    Success,
    /// Test case has a `<error />` tag
    Error(TestError),
    /// Test case has a `<failure />` tag
    Failure(TestFailure),
    /// Test case has a `<skipped />` tag
    Skipped(TestSkipped),
}
impl TestStatus {
    /// Returns `true` if the `TestStatus` is [`Success`](#variant.Success).
    pub fn is_success(&self) -> bool {
        matches!(self, TestStatus::Success)
    }
    /// Returns `true` if the `TestStatus` is [`Error(_)`](#variant.Error).
    pub fn is_error(&self) -> bool {
        matches!(self, TestStatus::Error(_))
    }
    /// Returns the contained [`Error(_)`](#variant.Error) value as a reference
    ///
    /// # Panics
    ///
    /// Panics if the value is not an [`Errror(_)`](#variant.Error)
    pub fn error_as_ref(&self) -> &TestError {
        if let TestStatus::Error(ref e) = self {
            return e;
        }
        panic!("called `TestStatus::error()` on a value that is not TestStatus::Error(_)");
    }

    /// Returns `true` if the `TestStatus` is [`Failure(_)`](#variant.Failure).
    pub fn is_failure(&self) -> bool {
        matches!(self, TestStatus::Failure(_))
    }

    /// Returns the contained [`Failure(_)`](#variant.Failure) value as a reference
    ///
    /// # Panics
    ///
    /// Panics if the value is not a [`Failure(_)`](#variant.Failure)
    pub fn failure_as_ref(&self) -> &TestFailure {
        if let TestStatus::Failure(ref e) = self {
            return e;
        }
        panic!("called `TestStatus::failure()` on a value that is not TestStatus::Failure(_)");
    }

    /// Returns `true` if the `TestStatus` is [`Skipped(_)`](#variant.Skipped).
    pub fn is_skipped(&self) -> bool {
        matches!(self, TestStatus::Skipped(_))
    }

    /// Returns the contained [`Skipped(_)`](#variant.Skipped) value as a reference
    ///
    /// # Panics
    ///
    /// Panics if the value is not a [`Skipped(_)`](#variant.Skipped)
    pub fn skipped_as_ref(&self) -> &TestSkipped {
        if let TestStatus::Skipped(ref e) = self {
            return e;
        }
        panic!("called `TestStatus::skipped()` on a value that is not TestStatus::Skipped(_)");
    }
}

#[derive(Debug, Clone)]
/// A test case
pub struct TestCase {
    /// How long the test case took to run, from the `time` attribute
    pub time: f64,
    /// Name of the test case, from the `name` attribute
    /// If there is a `classname` attribute, store it as `classname::name`
    /// See `original_name` for the original name
    pub name: String,
    /// Status of the test case
    pub status: TestStatus,
    /// Original name, from the `name` attribute
    pub original_name: String,
    /// Class name, from the `classname` attribute
    pub classname: Option<String>,
}
impl TestCase {
    fn new() -> Self {
        Self {
            time: 0f64,
            name: String::new(),
            status: TestStatus::Success,
            original_name: String::new(),
            classname: None,
        }
    }
    fn parse_attributes(&mut self, e: &XMLBytesStart) -> Result<(), Error> {
        for a in e.attributes() {
            let a = a?;
            match a.key {
                QName(b"time") => self.time = try_from_attribute_value_f64(a.value)?,
                QName(b"name") => self.original_name = try_from_attribute_value_string(a.value)?,
                QName(b"classname") => {
                    self.classname = Some(try_from_attribute_value_string(a.value)?)
                }
                _ => {}
            };
        }
        if let Some(cn) = self.classname.as_ref() {
            self.name = format!("{}::{}", cn, self.original_name);
        } else {
            self.name = self.original_name.clone();
        }
        Ok(())
    }

    fn new_empty(e: &XMLBytesStart) -> Result<Self, Error> {
        let mut tc = Self::new();
        tc.parse_attributes(e)?;
        Ok(tc)
    }

    fn new_from_reader<B: BufRead>(e: &XMLBytesStart, r: &mut XMLReader<B>) -> Result<Self, Error> {
        let mut tc = Self::new();
        tc.parse_attributes(e)?;
        let mut buf = Vec::new();
        loop {
            match r.read_event_into(&mut buf) {
                Ok(XMLEvent::End(ref e)) if e.name() == QName(b"testcase") => break,
                Ok(XMLEvent::Start(ref e)) if e.name() == QName(b"skipped") => {
                    let ts = TestSkipped::new_from_reader(e, r)?;
                    tc.status = TestStatus::Skipped(ts);
                }
                Ok(XMLEvent::Empty(ref e)) if e.name() == QName(b"skipped") => {
                    let ts = TestSkipped::new_empty(e)?;
                    tc.status = TestStatus::Skipped(ts);
                }
                Ok(XMLEvent::Start(ref e)) if e.name() == QName(b"failure") => {
                    let tf = TestFailure::new_from_reader(e, r)?;
                    tc.status = TestStatus::Failure(tf);
                }
                Ok(XMLEvent::Empty(ref e)) if e.name() == QName(b"failure") => {
                    let tf = TestFailure::new_empty(e)?;
                    tc.status = TestStatus::Failure(tf);
                }
                Ok(XMLEvent::Start(ref e)) if e.name() == QName(b"error") => {
                    let te = TestError::new_from_reader(e, r)?;
                    tc.status = TestStatus::Error(te);
                }
                Ok(XMLEvent::Empty(ref e)) if e.name() == QName(b"error") => {
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

#[derive(Debug, Clone)]
/// A test suite, containing test cases [`TestCase`](struct.TestCase.html)
pub struct TestSuite {
    /// List of status of tests represented by [`TestCase`]
    pub cases: Vec<TestCase>,
    /// How long the test suite took to run, from the `time` attribute
    pub time: f64,
    /// Number of tests in the test suite, from the `tests` attribute
    pub tests: u64,
    /// Number of tests in error in the test suite, from the `errors` attribute
    pub errors: u64,
    /// Number of tests in failure in the test suite, from the `failures` attribute
    pub failures: u64,
    /// Number of tests skipped in the test suites, from the `skipped` attribute
    pub skipped: u64,
    /// Name of the test suite, from the `name` attribute
    pub name: String,
}
impl TestSuite {
    fn new() -> Self {
        Self {
            cases: Vec::new(),
            time: 0f64,
            tests: 0u64,
            errors: 0u64,
            failures: 0u64,
            skipped: 0u64,
            name: String::new(),
        }
    }
    fn parse_attributes(&mut self, e: &XMLBytesStart) -> Result<(), Error> {
        for a in e.attributes() {
            let a = a?;
            match a.key {
                QName(b"time") => self.time = try_from_attribute_value_f64(a.value)?,
                QName(b"tests") => self.tests = try_from_attribute_value_u64(a.value)?,
                QName(b"errors") => self.errors = try_from_attribute_value_u64(a.value)?,
                QName(b"failures") => self.failures = try_from_attribute_value_u64(a.value)?,
                QName(b"skipped") => self.skipped = try_from_attribute_value_u64(a.value)?,
                QName(b"name") => self.name = try_from_attribute_value_string(a.value)?,
                _ => {}
            };
        }
        Ok(())
    }

    fn new_empty(e: &XMLBytesStart) -> Result<Self, Error> {
        let mut ts = Self::new();
        ts.parse_attributes(e)?;
        Ok(ts)
    }

    fn new_from_reader<B: BufRead>(e: &XMLBytesStart, r: &mut XMLReader<B>) -> Result<Self, Error> {
        let mut ts = Self::new();
        ts.parse_attributes(e)?;
        let mut buf = Vec::new();
        loop {
            match r.read_event_into(&mut buf) {
                Ok(XMLEvent::End(ref e)) if e.name() == QName(b"testsuite") => break,
                Ok(XMLEvent::Start(ref e)) if e.name() == QName(b"testcase") => {
                    ts.cases.push(TestCase::new_from_reader(e, r)?);
                }
                Ok(XMLEvent::Empty(ref e)) if e.name() == QName(b"testcase") => {
                    ts.cases.push(TestCase::new_empty(e)?);
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

#[derive(Debug, Clone)]
/// Struct representing a JUnit report, containing test suites [`TestSuite`](struct.TestSuite.html)
pub struct TestSuites {
    /// List of tests suites represented by [`TestSuite`]
    pub suites: Vec<TestSuite>,
    /// How long the test suites took to run, from the `time` attribute
    pub time: f64,
    /// Number of tests in the test suites, from the `tests` attribute
    pub tests: u64,
    /// Number of tests in error in the test suites, from the `errors` attribute
    pub errors: u64,
    /// Number of tests in failure in the test suites, from the `failures` attribute
    pub failures: u64,
    /// Number of tests skipped in the test suites, from the `skipped` attribute
    pub skipped: u64,
    /// Name of the test suites, from the `name` attribute
    pub name: String,
}
impl TestSuites {
    fn new() -> Self {
        Self {
            suites: Vec::new(),
            time: 0f64,
            tests: 0u64,
            errors: 0u64,
            failures: 0u64,
            skipped: 0u64,
            name: String::new(),
        }
    }

    fn parse_attributes(&mut self, e: &XMLBytesStart) -> Result<(), Error> {
        for a in e.attributes() {
            let a = a?;
            match a.key {
                QName(b"time") => self.time = try_from_attribute_value_f64(a.value)?,
                QName(b"tests") => self.tests = try_from_attribute_value_u64(a.value)?,
                QName(b"errors") => self.errors = try_from_attribute_value_u64(a.value)?,
                QName(b"failures") => self.failures = try_from_attribute_value_u64(a.value)?,
                QName(b"skipped") => self.skipped = try_from_attribute_value_u64(a.value)?,
                QName(b"name") => self.name = try_from_attribute_value_string(a.value)?,
                _ => {}
            };
        }
        Ok(())
    }

    fn new_empty(e: &XMLBytesStart) -> Result<Self, Error> {
        let mut ts = Self::new();
        ts.parse_attributes(e)?;
        Ok(ts)
    }

    fn new_from_reader<B: BufRead>(e: &XMLBytesStart, r: &mut XMLReader<B>) -> Result<Self, Error> {
        let mut ts = Self::new();
        ts.parse_attributes(e)?;
        let mut buf = Vec::new();
        loop {
            match r.read_event_into(&mut buf) {
                Ok(XMLEvent::End(ref e)) if e.name() == QName(b"testsuites") => break,
                Ok(XMLEvent::Start(ref e)) if e.name() == QName(b"testsuite") => {
                    ts.suites.push(TestSuite::new_from_reader(e, r)?);
                }
                Ok(XMLEvent::Empty(ref e)) if e.name() == QName(b"testsuite") => {
                    ts.suites.push(TestSuite::new_empty(e)?);
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

fn try_from_attribute_value_f64(value: Cow<[u8]>) -> Result<f64, Error> {
    match value {
        Cow::Borrowed(b) => {
            let s = str::from_utf8(b)?;
            if s.is_empty() {
                return Ok(0f64);
            }
            Ok(s.parse::<f64>()?)
        }
        Cow::Owned(ref b) => {
            let s = str::from_utf8(b)?;
            if s.is_empty() {
                return Ok(0f64);
            }
            Ok(s.parse::<f64>()?)
        }
    }
}

fn try_from_attribute_value_u64(value: Cow<[u8]>) -> Result<u64, Error> {
    match value {
        Cow::Borrowed(b) => {
            let s = str::from_utf8(b)?;
            if s.is_empty() {
                return Ok(0u64);
            }
            Ok(s.parse::<u64>()?)
        }
        Cow::Owned(ref b) => {
            let s = str::from_utf8(b)?;
            if s.is_empty() {
                return Ok(0u64);
            }
            Ok(s.parse::<u64>()?)
        }
    }
}

fn try_from_attribute_value_string(value: Cow<[u8]>) -> Result<String, Error> {
    match value {
        Cow::Borrowed(b) => Ok(str::from_utf8(b)?.to_owned()),
        Cow::Owned(ref b) => Ok(str::from_utf8(b)?.to_owned()),
    }
}

/// Creates a [`TestSuites`](struct.TestSuites.html) structure from a JUnit XML data read from `reader`
///
/// # Example
/// ```
/// use std::io::Cursor;
///     let xml = r#"
/// <testsuite tests="3" failures="1">
///   <testcase classname="foo1" name="ASuccessfulTest"/>
///   <testcase classname="foo2" name="AnotherSuccessfulTest"/>
///   <testcase classname="foo3" name="AFailingTest">
///     <failure type="NotEnoughFoo"> details about failure </failure>
///   </testcase>
/// </testsuite>
/// "#;
///     let cursor = Cursor::new(xml);
///     let r = junit_parser::from_reader(cursor);
///     assert!(r.is_ok());
/// ```

pub fn from_reader<B: BufRead>(reader: B) -> Result<TestSuites, Error> {
    let mut r = XMLReader::from_reader(reader);
    let mut buf = Vec::new();
    loop {
        match r.read_event_into(&mut buf) {
            Ok(XMLEvent::Empty(ref e)) if e.name() == QName(b"testsuites") => {
                return TestSuites::new_empty(e);
            }
            Ok(XMLEvent::Start(ref e)) if e.name() == QName(b"testsuites") => {
                return TestSuites::new_from_reader(e, &mut r);
            }
            Ok(XMLEvent::Empty(ref e)) if e.name() == QName(b"testsuite") => {
                let ts = TestSuite::new_empty(e)?;
                let mut suites = TestSuites::new();
                suites.suites.push(ts);
                return Ok(suites);
            }
            Ok(XMLEvent::Start(ref e)) if e.name() == QName(b"testsuite") => {
                let ts = TestSuite::new_from_reader(e, &mut r)?;
                let mut suites = TestSuites::new();
                suites.suites.push(ts);
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
