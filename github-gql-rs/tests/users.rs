extern crate github_gql as gh;
extern crate serde_json;
use gh::client::Github;
use gh::query::Query;
use serde_json::Value;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
//use gh::mutation::Mutation;

fn auth_token() -> Result<String, std::io::Error> {
    let file = File::open("tests/auth_token")?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let _ = reader.read_line(&mut buffer)?;
    Ok(buffer)
}

#[test]
fn graphql_basic_test() {
    let mut g = Github::new(&auth_token().unwrap()).unwrap();
    let (headers, status, json) = g.query::<Value>(
        &Query::new_raw("query { viewer { login } }")
    ).unwrap();

    println!("{}", headers);
    println!("{}", status);
    if let Some(json) = json {
        println!("{}", json);
    }
}

// #[test]
// fn add_reaction() {
//     let mut g = Github::new(&auth_token().unwrap()).unwrap();
//     let (headers, status, json) = g.mutation::<Value>(
//         &Mutation::new_raw(
//         "mutation AddReactionToIssue { \
//             addReaction( input: { subjectId: \\\"MDU6SXNzdWUyMzEzOTE1NTE=\\\", content: HOORAY } ) { \
//                 reaction { \
//                     content \
//                 } \
//                 subject { \
//                     id \
//                 } \
//             } \
//         }")
//     ).unwrap();
//     println!("{}", headers);
//     println!("{}", status);
//     if let Some(json) = json {
//         println!("{}", json);
//     }
// }
