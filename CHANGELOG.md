# Changelog

This project roughly adheres to [Semantic Versioning](http://semver.org/). For 0.x.y releases, `x` is the major version in semver, while `y` is the minor version.

## [0.8.0] - 2016-01-31

### Breaking changes

* Change `Error` to use `std::io::ErrorKind` as one of the variants

### Library

* Implement `std::io::Iterator` for `FileReader`
* Fix corrupt segment issue on FileWriter

### Project

* Use multiple version Rust to compile on Travis: nightly, beta, stable, 1.6.0 
* Add support for Coveralls
* Add documentation page on crates.io
