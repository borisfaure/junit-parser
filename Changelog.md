# Changelog

## 1.3.0 -- 2024-10-06

 - Parse CDATA sections in `system-out`, `system-err`, `failure`, `error` and
   `skipped` elements

## 1.2.4 -- 2024-07-19

 - Update quick-xml to 0.36

## 1.2.3 -- 2024-07-01

 - Update quick-xml to 0.35

## 1.2.2 -- 2024-06-28

 - Update quick-xml to 0.34

## 1.2.1 -- 2024-06-25

 - Update quick-xml to 0.33

## 1.2.0 -- 2024-06-01

 - Update quick-xml to 0.32
 - Add `Error::UnexpectedEndOfFile` variant when the XML file ends unexpectedly

## 1.1.1 -- 2024-05-14

 - Simplify code

## 1.1.0 -- 2024-01-10

 - Support nested `testsuite` tags
 - Support the `testrun` tag acting as `testsuites`
 - Update quick-xml to 0.31


## 1.0.0 -- 2023-09-24

 - Support only rust-1.56 or later
 - Use rust 2021 edition
 - Update quick-xml to 0.30
 - HTML-unescape attributes
 - Parse the `system-out` and `system-err` elements
 - Parse the `properties` element as a hashmap (with the
   `properties_as_hashmap` feature enabled by default) or as a vector (with
   the `properties_as_vector` feature enabled by default)
 - Parse the `timestamp`, `hostname`, `assertions`, `id`, `package`, `log`,
  `url`, `version`, `file` and `line` optional attributes in `TestSuite`
 - Derive `Default` for `TestError`, `TestSkipped`, `TestFailure`
 - Derive `Clone` to public structure
 - Add serde as optional dependency to (de)serialize the public structures.
   Disabled by default but can be activated through the `serde` feature.
 - Update documentation
 - Add CI based on GitHub Actions


## 0.2.0 -- 2021-03-20

 - Update quick-xml to 0.22
 - Store test name as CLASSNAME::NAME
 - Use a vec instead of a hashmap, to keep test {suites/cases} order


## 0.1.1 -- 2020-12-02

 - Derive Clone on public structures


## 0.1.0 -- 2020-11-08

 - Initial release
