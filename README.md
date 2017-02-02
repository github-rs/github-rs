# github-rs

| Service      | Status |
| -------      | :----: |
| TravisCI     | [![Build Status](https://travis-ci.org/mgattozzi/github-rs.svg?branch=master)](https://travis-ci.org/mgattozzi/github-rs)   |
| DependencyCI | [![DependencyStatus](https://dependencyci.com/github/mgattozzi/github-rs/badge)](https://dependencyci.com/github/mgattozzi/github-rs)   |
| AppveyorCI   | [![Build status](https://ci.appveyor.com/api/projects/status/st04a7hltt8h42lq?svg=true)](https://ci.appveyor.com/project/mgattozzi/github-rs)       |
| CodeCov   | [![codecov](https://codecov.io/gh/mgattozzi/github-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/mgattozzi/github-rs)      |
| crates.io | ![crates.io](https://img.shields.io/crates/v/github-rs.svg)

Pure Rust bindings to the Github API using Hyper and Serde

## Incomplete Bindings
This is in no way close to being done for most of the Github API, but
the functionality that is available does do a lot of the work that one
would expect when accessing the API.

## Dependencies and Support
github-rs is intended to work on all tier 1 supported Rust systems:

- Windows
- Linux
- MacOSX

github-rs only supports the Rust compiler from stable versions 1.15 onwards
due to using Custom Derive.

## Project Aims
- Have a robust API where everything is error handled properly to avoid
  panics of any kind. A library is the base of an application and should
  be a solid foundation to be built upon
- Cover all Github stable endpoints. Anything that's deprecated and beta
  should be obtained only through configuration for those features. As
  deprecated endpoints are removed from Github so too should they be
  removed from this library.
- Having stability as part of the API. As such effort will be
  taken to make sure this code compiles on stable Rust, rather than
  nightly.
- Ease of use. The complexity should be hidden from those not hacking on
  the code itself.
- Documentation of everything so not only is it easy to hack on but
  finding out how to use the library should be easy to find.

## Hacking on the Library
- [API Reference Docs](https://developer.github.com/v3/)
- Each of the overarching groupings of the endpoints in the
  Github documentation have a corresponding .rs file under the `src`
  directory. If you wish to add endpoints or modify how one works that
  would be the place to go to.
- `src/types.rs` contains all new types and aliased types that are used
  throughout github-rs.
- `src/json.rs` contains all of the structures that represent the JSON
  that are either returned from or sent to the API.
- `src/error.rs` contains all code related to error handling including
  types
- `src/github.rs` contains the client code as well as top level
  documentation of the library. Most code is publicly reexported here.
- Any new additions must contain corresponding documentation comments
  explaining their use.
- All endpoint requests must be wrapped in a Result type. The Result
  type alias we use is located in `src/error.rs`
- No submitted code may use `unwrap` or `expect`.
- Use cargo clippy to determine if any other errors or lints pop up.
  cargo build --feature "dev" will build it with clippy
- Code when compiled may not emit warnings unless it's an underlying
  library imported into github-rs.

## License
All code is licensed under The MIT License. By submitting code you not
only agree to submit it under the same license, but are also saying that
the code you submitted is your own.
