extern crate github_gql as gh;
extern crate serde_json;
use gh::client::Github;
use gh::query::Query;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
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
    let mut g = Github::new(&auth_token().expect("auth token")).unwrap();
    let (headers, status, json) = g
        .query::<Value>(&Query::new_raw("query { viewer { login } }"))
        .unwrap();

    println!("{:#?}", headers);
    println!("{}", status);
    if let Some(json) = json {
        println!("{}", json);
    }
}

//testing if escaping json properly
#[test]
fn graphql_escaping_test() {
    let q_str = r#"{
  user(login: "mgattozzi") {
    login
  }
}
"#;

    let mut g = Github::new(&auth_token().expect("auth token")).unwrap();
    let (headers, status, json) = g.query::<Value>(&Query::new_raw(q_str)).unwrap();

    println!("{:#?}", headers);
    println!("response status: {}", status);
    if let Some(ref json) = json {
        println!("{}", json);
    }
    assert_eq!(
        json.unwrap()["data"]["user"]["login"].as_str().unwrap(),
        "mgattozzi"
    );
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
//     println!("{:#?}", headers);
//     println!("{}", status);
//     if let Some(json) = json {
//         println!("{}", json);
//     }
// }
