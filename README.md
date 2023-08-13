![CI](https://github.com/borisfaure/junit-parser/actions/workflows/ci.yml/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/junit-parser.svg)](https://crates.io/crates/junit-parser)
[![Docs](https://docs.rs/junit_parser/badge.svg)](https://docs.rs/junit_parser/)
[![License](https://img.shields.io/badge/license-BSD--2--Clause-green.svg)](LICENSE.txt)

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
```

## License

This project is available under the terms of either the [BSD-2-Clause license](LICENSE.txt).
