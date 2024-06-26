[![CI](https://github.com/borisfaure/junit-parser/actions/workflows/ci.yml/badge.svg)](https://github.com/borisfaure/junit-parser/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/junit-parser.svg)](https://crates.io/crates/junit-parser)
[![Docs](https://docs.rs/junit-parser/badge.svg)](https://docs.rs/junit_parser/)
[![License](https://img.shields.io/badge/license-BSD--2--Clause-green.svg)](LICENSE.txt)
[![MSRV](https://img.shields.io/badge/rustc-1.56.0+-ab6000.svg)](https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html)

# JUnit-Parser

Rust library to parse JUnit XML files

[Documentation](https://docs.rs/junit_parser/)


## Example

Create a
[`TestSuites`](https://docs.rs/junit-parser/latest/junit_parser/struct.TestSuites.html) structure from a JUnit XML data read from `reader`:

```rust
use std::io::Cursor;
let xml = r#"
<testsuite tests="3" failures="1">
  <testcase classname="foo1" name="ASuccessfulTest"/>
  <testcase classname="foo2" name="AnotherSuccessfulTest"/>
  <testcase classname="foo3" name="AFailingTest">
    <failure type="NotEnoughFoo"> details about failure </failure>
  </testcase>
</testsuite>
"#;
let cursor = Cursor::new(xml);
let r = junit_parser::from_reader(cursor);
assert!(r.is_ok());
let t = r.unwrap();
assert_eq!(t.suites.len(), 1);
let ts = &t.suites[0];
assert_eq!(ts.tests, 3);
assert_eq!(ts.failures, 1);
assert_eq!(ts.cases.len(), 3);
assert!(ts.cases[0].status.is_success());
assert!(ts.cases[2].status.is_failure());
```


## Features

- `serde` — Enables `derive(serde::{Serialize,Deserialize})` on the Test* structures.
- `properties_as_hashmap` (enabled by default) — Parse the properties element as a hashmap
- `properties_as_vector` (enabled by default) — Parse the properties element as a vector


## License

This project is available under the terms of either the [BSD-2-Clause license](LICENSE.txt).
