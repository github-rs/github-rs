# github-gql-rs

## Incomplete Bindings
Given that this is a bit more free form a full on GraphQL library for client
queries has not been made. Please refer to the GitHub V4 API docs for queries
you can make.

## Dependencies and Support
github-gql-rs is intended to work on all tier 1 supported Rust systems:

- Windows
- Linux
- MacOSX

## Minimum Compiler Version
Due to the use of certain features githubgql--rs requires rustc version 1.18 or
higher.

## Project Aims
- Have a robust API where everything is error handled properly to avoid
  panics of any kind. A library is the base of an application and should
  be a solid foundation to be built upon
- Having stability as part of the API. As such effort will be
  taken to make sure this code compiles on stable Rust, rather than
  nightly.
- Ease of use. The complexity should be hidden from those not hacking on
  the code itself.
- Documentation of everything so not only is it easy to hack on but
  finding out how to use the library should be easy to find.

## Getting Started
Add the following to your `Cargo.toml`

```toml
[dependencies]
github-gql-rs = "0.0.1"
```

Then in your `lib.rs` or `main.rs` file add:

```rust
extern crate github_gql;
use github_gql::client::Github;
```

Now you can start making queries. Here's a small example to get your user
information:

```rust
extern crate github_gql as gh;
extern crate serde_json;
use gh::client::Github;
use gh::query::Query;
use serde_json::Value;

fn main() {
    let mut g = Github::new("YOUR API TOKEN GOES HERE").unwrap();
    let (headers, status, json) = g.query::<Value>(
        &Query::new_raw("query { viewer { login } }")
    ).unwrap();

    println!("{}", headers);
    println!("{}", status);
    if let Some(json) = json {
        println!("{}", json);
    }

    // This will update https://github.com/octocat/Hello-World/issues/349
    // with a HOORAY emoji!
    let (headers, status, json) = g.mutation::<Value>(
        &Mutation::new_raw(
        r#"mutation AddReactionToIssue { 
            addReaction( input: { subjectId: "MDU6SXNzdWUyMzEzOTE1NTE=", content: HOORAY } ) { 
                reaction { 
                    content 
                } 
                subject { 
                    id 
                } 
            } 
        }"#)
    ).unwrap();
    println!("{}", headers);
    println!("{}", status);
    if let Some(json) = json {
        println!("{}", json);
    }
}
```

## Hacking on the Library
- [GitHub API Reference Docs](https://developer.github.com/v4/)

## Contributing
See [CONTRIBUTING.md](../CONTRIBUTING.md) for more information.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
