You want to help contribute? Awesome! Thanks for taking the time to look at the
guidelines for this repo. Here's what you need to know!

## Code
If you want to contribute code but don't know how everything works check out the
[design docs](./docs/design.md) for the library. If you want to know what
endpoints still need to be implemented see the [endpoints](./docs/endpoints.md)
docs.

Due to the use of certain features github-rs requires rustc version 1.18 or higher.

To run tests, you will need a auth token from GitHub. You can follow the official [GitHub documentation](https://help.github.com/articles/creating-a-personal-access-token-for-the-command-line/) to get a personal access token for testing. Once you have an access token, create a file under the tests folder called auth_token with your token as the only string. Run `cargo test` to make sure all of the tests pass.

## Documentation
As with any project, documentation is a key part that can make or break usage of
a library. Why use the best library ever if it has no documentation? With that
in mind, github-rs strives to document every aspect of it in order to make it
easier for the user. While everything is documented with doc comments in the
code the real important part is the graph representation of the code base.
github-rs essentially uses a DAG in order to build up queries that work while
preventing the end user from doing anything that would cause an error for the
GitHub API due to malformed URLs. All of these states are easier to see as
a picture rather than as links built with rustdoc. Maintaining this is an
important part of the project and making sure it's accurate and reflects the
current code base is an important job that anyone can help with! All of the
endpoints that have been completed can be found in [the endpoints
file](./docs/endpoints.md). Using this as reference one can cross check that the
graph implements these as well.

## Issue Tracker
Have you encountered a problem with the API? Please take a look at [the issue
guidelines](./docs/issues.md) to properly file a bug in the repo!

## Examples
Do you want to help show off some ways for how the library works? Feel free to
work on an example and open up a PR!
