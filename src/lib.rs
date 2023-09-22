#![warn(missing_docs)]
//! Library to parse JUnit XML files

/// Errors
mod errors;

pub use errors::Error;
use quick_xml::escape::unescape;
use quick_xml::events::BytesStart as XMLBytesStart;
use quick_xml::events::Event as XMLEvent;
use quick_xml::name::QName;
use quick_xml::Error as XMLError;
use quick_xml::Reader as XMLReader;
use std::borrow::Cow;
#[cfg(feature = "properties_as_hashmap")]
use std::collections::HashMap;
use std::io::prelude::*;
use std::str;
use std::vec::Vec;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
/// Properties associated to a [`TestSuite`] or a [`TestCase`]
pub struct Properties {
    /// Hashmap of the properties
    #[cfg(feature = "properties_as_hashmap")]
    pub hashmap: HashMap<String, String>,
    /// Vector of the properties
    #[cfg(feature = "properties_as_vector")]
    pub vec: Vec<(String, String)>,
}

/// Parse attributes of a `property` element
fn parse_property<B: BufRead>(
    e: &XMLBytesStart,
    r: Option<&mut XMLReader<B>>,
) -> Result<(String, String), Error> {
    let mut k: Option<String> = None;
    let mut v: Option<String> = None;
    for a in e.attributes() {
        let a = a?;
        match a.key {
            QName(b"name") => k = Some(try_from_attribute_value_string(a.value)?),
            QName(b"value") => v = Some(try_from_attribute_value_string(a.value)?),
            _ => {}
        };
    }
    if let Some(r) = r {
        let mut buf = Vec::new();
        loop {
            match r.read_event_into(&mut buf) {
                Ok(XMLEvent::End(ref e)) if e.name() == QName(b"property") => break,
                Ok(XMLEvent::Eof) => {
                    return Err(XMLError::UnexpectedEof("property".to_string()).into())
                }
                Ok(XMLEvent::Text(e)) => {
                    v = Some(e.unescape()?.trim().to_string());
                }
                Err(err) => return Err(err.into()),
                _ => (),
            }
        }
        buf.clear();
    }
    match (k, v) {
        (Some(k), Some(v)) => Ok((k, v)),
        (Some(k), None) => Ok((k, "".to_string())),
        _ => Err(Error::MissingPropertyName),
    }
}

impl Properties {
    /// Create a [`Properties`] from a XML `properties` element
    fn from_reader<B: BufRead>(r: &mut XMLReader<B>) -> Result<Self, Error> {
        let mut p = Self::default();
        let mut buf = Vec::new();
        loop {
            match r.read_event_into(&mut buf) {
                Ok(XMLEvent::End(ref e)) if e.name() == QName(b"properties") => break,

                Ok(XMLEvent::Empty(ref e)) if e.name() == QName(b"property") => {
                    let (k, v) = parse_property::<B>(e, None)?;
                    p.add_property(k, v);
                }
                Ok(XMLEvent::Start(ref e)) if e.name() == QName(b"property") => {
                    let (k, v) = parse_property(e, Some(r))?;
                    p.add_property(k, v);
                }
                Ok(XMLEvent::Eof) => {
                    return Err(XMLError::UnexpectedEof("properties".to_string()).into())
                }
                Err(err) => return Err(err.into()),
                _ => (),
            }
        }
        buf.clear();
        Ok(p)
    }

    // `key` and `value` if no feature to store them
    #[cfg_attr(
        all(
            not(feature = "properties_as_hashmap"),
            not(feature = "properties_as_vector")
        ),
        allow(unused_variables)
    )]
    /// Add a property to the set of properties
    fn add_property(&mut self, key: String, value: String) {
        #[cfg(feature = "properties_as_hashmap")]
        self.hashmap.insert(key.clone(), value.clone());
        #[cfg(feature = "properties_as_vector")]
        self.vec.push((key, value));
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// Fill up `self` with attributes from the XML tag
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

    /// New [`TestFailure`] from empty XML tag
    fn new_empty(e: &XMLBytesStart) -> Result<Self, Error> {
        let mut tf = Self::default();
        tf.parse_attributes(e)?;
        Ok(tf)
    }

    /// New [`TestFailure`] from XML tree
    fn from_reader<B: BufRead>(e: &XMLBytesStart, r: &mut XMLReader<B>) -> Result<Self, Error> {
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// Fill up `self` with attributes from the XML tag
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

    /// New [`TestError`] from empty XML tag
    fn new_empty(e: &XMLBytesStart) -> Result<Self, Error> {
        let mut te = Self::default();
        te.parse_attributes(e)?;
        Ok(te)
    }

    /// New [`TestError`] from XML tree
    fn from_reader<B: BufRead>(e: &XMLBytesStart, r: &mut XMLReader<B>) -> Result<Self, Error> {
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// Fill up `self` with attributes from the XML tag
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

    /// New [`TestSkipped`] from empty XML tag
    fn new_empty(e: &XMLBytesStart) -> Result<Self, Error> {
        let mut ts = Self::default();
        ts.parse_attributes(e)?;
        Ok(ts)
    }

    /// New [`TestSkipped`] from XML tree
    fn from_reader<B: BufRead>(e: &XMLBytesStart, r: &mut XMLReader<B>) -> Result<Self, Error> {
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
/// Status of a test case
pub enum TestStatus {
    /// Success
    #[default]
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
/// A test case
pub struct TestCase {
    /// How long the test case took to run, from the `time` attribute
    pub time: f64,
    /// Name of the test case, from the `name` attribute
    /// If there is a `classname` attribute, store it as `classname::name`
    /// Otherwise if there is a `group` attribute, store it as `group::name`
    /// See [`TestCase::original_name`] for the original name
    pub name: String,
    /// Status of the test case
    pub status: TestStatus,
    /// Original name, from the `name` attribute
    pub original_name: String,
    /// Class name, from the `classname` attribute
    pub classname: Option<String>,
    /// Group name, from the `group` attribute
    pub group: Option<String>,
    /// File source code of the test
    pub file: Option<String>,
    /// Related line in the source code
    pub line: Option<u64>,
    /// stdout output from the `system-out` element
    pub system_out: Option<String>,
    /// stderr output from the `system-err` element
    pub system_err: Option<String>,
    /// Properties of the test case
    pub properties: Properties,
}
impl TestCase {
    /// Fill up `self` with attributes from the XML tag
    fn parse_attributes(&mut self, e: &XMLBytesStart) -> Result<(), Error> {
        for a in e.attributes() {
            let a = a?;
            match a.key {
                QName(b"time") => self.time = try_from_attribute_value_f64(a.value)?,
                QName(b"name") => self.original_name = try_from_attribute_value_string(a.value)?,
                QName(b"classname") => {
                    self.classname = Some(try_from_attribute_value_string(a.value)?)
                }
                QName(b"group") => self.group = Some(try_from_attribute_value_string(a.value)?),
                QName(b"file") => self.file = Some(try_from_attribute_value_string(a.value)?),
                QName(b"line") => self.line = Some(try_from_attribute_value_u64(a.value)?),
                _ => {}
            };
        }
        if let Some(cn) = self.classname.as_ref() {
            self.name = format!("{}::{}", cn, self.original_name);
        } else if let Some(gn) = self.group.as_ref() {
            self.name = format!("{}::{}", gn, self.original_name);
        } else {
            self.name = self.original_name.clone();
        }
        Ok(())
    }

    /// New [`TestCase`] from empty XML tag
    fn new_empty(e: &XMLBytesStart) -> Result<Self, Error> {
        let mut tc = Self::default();
        tc.parse_attributes(e)?;
        Ok(tc)
    }

    /// New [`TestCase`] from XML tree
    fn from_reader<B: BufRead>(e: &XMLBytesStart, r: &mut XMLReader<B>) -> Result<Self, Error> {
        let mut tc = Self::default();
        tc.parse_attributes(e)?;
        let mut buf = Vec::new();
        loop {
            match r.read_event_into(&mut buf) {
                Ok(XMLEvent::End(ref e)) if e.name() == QName(b"testcase") => break,
                Ok(XMLEvent::Start(ref e)) if e.name() == QName(b"skipped") => {
                    let ts = TestSkipped::from_reader(e, r)?;
                    tc.status = TestStatus::Skipped(ts);
                }
                Ok(XMLEvent::Empty(ref e)) if e.name() == QName(b"skipped") => {
                    let ts = TestSkipped::new_empty(e)?;
                    tc.status = TestStatus::Skipped(ts);
                }
                Ok(XMLEvent::Start(ref e)) if e.name() == QName(b"failure") => {
                    let tf = TestFailure::from_reader(e, r)?;
                    tc.status = TestStatus::Failure(tf);
                }
                Ok(XMLEvent::Empty(ref e)) if e.name() == QName(b"failure") => {
                    let tf = TestFailure::new_empty(e)?;
                    tc.status = TestStatus::Failure(tf);
                }
                Ok(XMLEvent::Start(ref e)) if e.name() == QName(b"error") => {
                    let te = TestError::from_reader(e, r)?;
                    tc.status = TestStatus::Error(te);
                }
                Ok(XMLEvent::Empty(ref e)) if e.name() == QName(b"error") => {
                    let te = TestError::new_empty(e)?;
                    tc.status = TestStatus::Error(te);
                }
                Ok(XMLEvent::Empty(ref e)) if e.name() == QName(b"system-out") => {}
                Ok(XMLEvent::Start(ref e)) if e.name() == QName(b"system-out") => {
                    tc.system_out = parse_system(e, r)?;
                }
                Ok(XMLEvent::Empty(ref e)) if e.name() == QName(b"system-err") => {}
                Ok(XMLEvent::Start(ref e)) if e.name() == QName(b"system-err") => {
                    tc.system_err = parse_system(e, r)?;
                }
                Ok(XMLEvent::Start(ref e)) if e.name() == QName(b"properties") => {
                    tc.properties = Properties::from_reader(r)?;
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
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
    /// Number of assertions in the test suites, from the `assertions` attribute
    pub assertions: Option<u64>,
    /// Name of the test suite, from the `name` attribute
    pub name: String,
    /// Timestamp when the test suite was run, from the `timestamp` attribute
    pub timestamp: Option<String>,
    /// Hostname where the test suite was run, from the `hostname` attribute
    pub hostname: Option<String>,
    /// Identifier of the test suite, from the `id` attribute
    pub id: Option<String>,
    /// Package of the test suite, from the `package` attribute
    pub package: Option<String>,
    /// Source code file of the test suite, from the `file` attribute
    pub file: Option<String>,
    /// Logger of the test suite, from the `log` attribute
    pub log: Option<String>,
    /// URL of the test suite, from the `uri` attribute
    pub url: Option<String>,
    /// Version of the test suite, from the `version` attribute
    pub version: Option<String>,
    /// stdout output from the `system-out` element
    pub system_out: Option<String>,
    /// stderr output from the `system-err` element
    pub system_err: Option<String>,
    /// Properties of the test suite
    pub properties: Properties,
}
impl TestSuite {
    /// Fill up `self` with attributes from the XML tag
    fn parse_attributes(&mut self, e: &XMLBytesStart) -> Result<(), Error> {
        for a in e.attributes() {
            let a = a?;
            match a.key {
                QName(b"time") => self.time = try_from_attribute_value_f64(a.value)?,
                QName(b"tests") => self.tests = try_from_attribute_value_u64(a.value)?,
                QName(b"errors") => self.errors = try_from_attribute_value_u64(a.value)?,
                QName(b"failures") => self.failures = try_from_attribute_value_u64(a.value)?,
                QName(b"skipped") => self.skipped = try_from_attribute_value_u64(a.value)?,
                QName(b"assertions") => {
                    self.assertions = Some(try_from_attribute_value_u64(a.value)?)
                }
                QName(b"name") => self.name = try_from_attribute_value_string(a.value)?,
                QName(b"timestamp") => {
                    self.timestamp = Some(try_from_attribute_value_string(a.value)?)
                }
                QName(b"hostname") => {
                    self.hostname = Some(try_from_attribute_value_string(a.value)?)
                }
                QName(b"id") => self.id = Some(try_from_attribute_value_string(a.value)?),
                QName(b"package") => self.package = Some(try_from_attribute_value_string(a.value)?),
                QName(b"file") => self.file = Some(try_from_attribute_value_string(a.value)?),
                QName(b"log") => self.log = Some(try_from_attribute_value_string(a.value)?),
                QName(b"url") => self.url = Some(try_from_attribute_value_string(a.value)?),
                QName(b"version") => self.version = Some(try_from_attribute_value_string(a.value)?),
                _ => {}
            };
        }
        Ok(())
    }

    /// New [`TestSuite`] from empty XML tag
    fn new_empty(e: &XMLBytesStart) -> Result<Self, Error> {
        let mut ts = Self::default();
        ts.parse_attributes(e)?;
        Ok(ts)
    }

    /// New [`TestSuite`] from XML tree
    fn from_reader<B: BufRead>(e: &XMLBytesStart, r: &mut XMLReader<B>) -> Result<Self, Error> {
        let mut ts = Self::default();
        ts.parse_attributes(e)?;
        let mut buf = Vec::new();
        loop {
            match r.read_event_into(&mut buf) {
                Ok(XMLEvent::End(ref e)) if e.name() == QName(b"testsuite") => break,
                Ok(XMLEvent::Start(ref e)) if e.name() == QName(b"testcase") => {
                    ts.cases.push(TestCase::from_reader(e, r)?);
                }
                Ok(XMLEvent::Empty(ref e)) if e.name() == QName(b"testcase") => {
                    ts.cases.push(TestCase::new_empty(e)?);
                }
                Ok(XMLEvent::Empty(ref e)) if e.name() == QName(b"system-out") => {}
                Ok(XMLEvent::Start(ref e)) if e.name() == QName(b"system-out") => {
                    ts.system_out = parse_system(e, r)?;
                }
                Ok(XMLEvent::Empty(ref e)) if e.name() == QName(b"system-err") => {}
                Ok(XMLEvent::Start(ref e)) if e.name() == QName(b"system-err") => {
                    ts.system_err = parse_system(e, r)?;
                }
                Ok(XMLEvent::Start(ref e)) if e.name() == QName(b"properties") => {
                    ts.properties = Properties::from_reader(r)?;
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
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
    /// Fill up `self` with attributes from the XML tag
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

    /// New [`TestSuites`] from empty XML tag
    fn new_empty(e: &XMLBytesStart) -> Result<Self, Error> {
        let mut ts = Self::default();
        ts.parse_attributes(e)?;
        Ok(ts)
    }

    /// New [`TestSuites`] from XML tree
    fn from_reader<B: BufRead>(e: &XMLBytesStart, r: &mut XMLReader<B>) -> Result<Self, Error> {
        let mut ts = Self::default();
        ts.parse_attributes(e)?;
        let mut buf = Vec::new();
        loop {
            match r.read_event_into(&mut buf) {
                Ok(XMLEvent::End(ref e)) if e.name() == QName(b"testsuites") => break,
                Ok(XMLEvent::Start(ref e)) if e.name() == QName(b"testsuite") => {
                    ts.suites.push(TestSuite::from_reader(e, r)?);
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

/// Try to decode attribute value as [`f64`]
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

/// Try to decode attribute value as [`u64`]
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

/// Try to decode and unescape attribute value as [`String`]
fn try_from_attribute_value_string(value: Cow<[u8]>) -> Result<String, Error> {
    let s = match value {
        Cow::Borrowed(b) => str::from_utf8(b)?,
        Cow::Owned(ref b) => str::from_utf8(b)?,
    };
    match unescape(s)? {
        Cow::Borrowed(u) => Ok(u.to_owned()),
        Cow::Owned(ref u) => Ok(u.to_owned()),
    }
}

/// Parse a chunk of xml as system-out or system-err
fn parse_system<B: BufRead>(
    orig: &XMLBytesStart,
    r: &mut XMLReader<B>,
) -> Result<Option<String>, Error> {
    let mut buf = Vec::new();
    let mut res = None;
    loop {
        match r.read_event_into(&mut buf) {
            Ok(XMLEvent::End(ref e)) if e.name() == orig.name() => break,
            Ok(XMLEvent::Text(e)) => {
                res = Some(e.unescape()?.to_string());
            }
            Ok(XMLEvent::Eof) => {
                return Err(XMLError::UnexpectedEof(format!("{:?}", orig.name())).into());
            }
            Err(err) => return Err(err.into()),
            _ => (),
        }
    }
    buf.clear();
    Ok(res)
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
                return TestSuites::from_reader(e, &mut r);
            }
            Ok(XMLEvent::Empty(ref e)) if e.name() == QName(b"testsuite") => {
                let ts = TestSuite::new_empty(e)?;
                let mut suites = TestSuites::default();
                suites.suites.push(ts);
                return Ok(suites);
            }
            Ok(XMLEvent::Start(ref e)) if e.name() == QName(b"testsuite") => {
                let ts = TestSuite::from_reader(e, &mut r)?;
                let mut suites = TestSuites::default();
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
