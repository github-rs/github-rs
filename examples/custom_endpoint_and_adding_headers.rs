use github_rs::client::{Executor, Github};
use github_rs::{HeaderMap, StatusCode};
use hyper::header::{HeaderValue, ACCEPT};
use serde_json::Value;

fn main() {
    //create new client
    let client = Github::new("API TOKEN").expect("failed to create client");
    //github username
    let owner = "rust-lang";
    //repository name
    let repo_name = "rfcs";
    //
    let issues = get_issues(&client, owner, repo_name).expect("failed to get issues");
    //get all issues as json array
    let issues_arr = issues
        .as_array()
        .expect("failed to cast issues to json array");
    //is array empty?
    if issues_arr.len() > 0 {
        //get first issue info
        let first_issue = &issues_arr[0];
        //println!("{:?}", &first_issue);
        let number_as_value = &first_issue["number"];
        let number = number_as_value.as_u64().unwrap();
        let reactions =
            get_reactions(&client, owner, repo_name, number).expect("failed to get reactions");

        //... do something with reactions
        println!(
            "reactions:
{:?}",
            reactions
        );
    } else {
        println!("no issues!");
    }
}

//get all issues for repo
fn get_issues(client: &Github, owner: &str, repo_name: &str) -> Option<Value> {
    //endpoint found on https://developer.github.com/v3/issues/#list-issues-for-a-repository
    let issues_endpoint = format!("repos/{}/{}/issues", owner, repo_name);
    //execute
    let response = client
        .get()
        //set custom endpoint here
        .custom_endpoint(&issues_endpoint)
        .execute::<Value>();
    print_info_and_get_json(response)
}

//get reactions for particular issue
fn get_reactions(
    client: &Github,
    owner: &str,
    repo_name: &str,
    issue_number: u64,
) -> Option<Value> {
    //build endpoint
    let reactions_endpoint = format!(
        "repos/{}/{}/issues/{}/reactions",
        owner, repo_name, issue_number
    );

    println!("reactions endpoint: {:?}", &reactions_endpoint);
    //send request with custom header
    let reactions_response = client
        .get()
        .custom_endpoint(&reactions_endpoint)
        .set_header(
            ACCEPT,
            HeaderValue::from_static("application/vnd.github.squirrel-girl-preview"),
        )
        .execute::<Value>();

    print_info_and_get_json(reactions_response)
}

//printing headers and status or error and returning json on success
fn print_info_and_get_json(
    response: Result<(HeaderMap, StatusCode, Option<Value>), github_rs::errors::Error>,
) -> Option<Value> {
    match response {
        Ok((headers, status, json)) => {
            println!("{:#?}", headers);
            println!("{}", status);
            json
        }
        Err(e) => {
            println!("{}", e);
            None
        }
    }
}
