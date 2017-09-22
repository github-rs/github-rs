# API Design
You want to help out? Awesome! Chances are you don't know how everything is
structured, where everything is, etc. This document will help you out by
explaining how the API works, how it's structured, the macros used to generate
all of the code work and how to add new endpoints.

## Builder Pattern Queries
One of the main things to understand about this API is that each query is
following the builder pattern. Function calls build up a query that can
eventually be executed. For instance to get all of a specific users repos you
would do the following:

```rust
let mut client = GitHub::new("API KEY");
let repos = client.get()
                  .users()
                  .username("mgattozzi")
                  .repos()
                  .execute();
```

This set of function calls corresponds to the following endpoint described in
the GitHub API Docs:

```
GET /users/:username/repos
```

Think of the first function call as the type of operation (GET, POST, PUT, PATCH
and DELETE) and every other function call as building up the url, taking
parameters as needed. This is the builder pattern that's implemented elsewhere
in Rust, usually when using Rust in a more functional style. Now what's
important to note is that at each function call we get a different type. This
type limits what functions can be called so that users can only make valid
requests. You can only build up the url until you have no choice but to execute
the request to the API. This means that the library is implementing a DAG under
the hood!

## Code Structure
Under the `src` directory all of the library code is divided up logically.
Anything with a subdirectory is where all of the data types for each logical
grouping of endpoints (e.g. user/users is under the users directory) exist.
They're further divided up into files named after each request type (GET
= get.rs, POST = post.rs, PUT = put.rs, PATCH = patch.rs, and DELETE
= delete.rs) that might be available under those endpoints. Here are what each
file or subdirectory is for:

- client.rs
  - Contains all of the logic for the `Client` struct which is what's used to
    dispatch requests to the API
- lib.rs
  - Sets library feature flags
  - Import crates and declare modules
  - Set any project lints
- macros.rs
  - The meat of the project, used to help automate a lot of the boiler plate
    code in this library's design.
- util.rs
  - Helper functions
  - Any function that doesn't have a place elsewhere
- gists
  - Code for requests regarding GitHub Gists
- issues
  - Code for requests regarding GitHub Issues
- misc
  - Code for requests for GitHub that don't fall under the other categories
- notifications
  - Code for requests regarding GitHub Notifications
- orgs
  - Code for requests regarding GitHub Organizations
- repos
  - Code for requests regarding GitHub Repos
- search
  - Code for requests regarding GitHub Search
- teams
  - Code for requests regarding GitHub Teams
- users
  - Code for requests regarding GitHub Users

## Macros
Macros are used to generate all of the code needed for the library. Here are the
macros you need to understand, how to use them, and what they do:

- `new_type!` is used to declare a new type that represents a step in building
  up the Url. Here's how it's used:

  ```rust
  new_type!(User); // <-- This means we've declared a new struct called User
  ```

  This expands out to:

  ```rust
  pub struct User<'a> {
      pub(crate) request: Result<Request<Body>>,
      pub(crate) core: &'a mut Core,
      pub(crate) client: &'g Rc<Client<HttpsConnector>>,
      pub(crate) parameter: Option<String>,
  }
  ```

  It contains a value request which is the current value of the request and core
  which contains a mutable reference to a Tokio Core type which is used to run
  the request when it's ready. It also contains an optional parameter which can
  be used to specify GET/POST parameters. Note you'll need to use 'a as a
  lifetime for any `impl` of the new type you need to implement manually.

- `from!` is used to create an implementation of `From` of a type you specify to
  another type. It's needed every time you want one type to turn into another.
  This is needed for the `func!` macro. If you forget to do this it will not
  work. It can be used in three different ways:

  ```rust
  from!(
      @TypeA                      //<-- Create a From implementation from A to B
        ?> TypeB = "param"        //    which includes a GET/POST parameter
  );                              //    named 'param'
  from!(
      @TypeA                      //<-- Create a From implementation from A to B
        => TypeB
  );
  from!(
      @TypeA                      //<-- Create a From implementation from A to B
        -> TypeB = "path"         //    which appends a path to the constructed
  );                              //    URL
  ```

- `impl_macro!` is used to create functions on types created by the `new_type!`
  macro. This is used to implement the functions used when contructing a
  request. It can be used in four different ways:

  ```rust
  impl_macro!(                     //<-- Create a function 'func' which, when
      @TypeA                       //    called, returns a B
        |=> func -> TypeB
  );
  impl_macro!(                     //<-- Create a function 'func' which, when
      @TypeA                       //    called, returns a B with 'path' added
        |=> func -> TypeB = path   //    to the request URL. 'path' is the name
  );                               //    of the variable is for documentation
  impl_macro!(                     //<-- Create a function 'func' which, when
      @TypeA                       //    called, returns a B with a GET/POST
        |?> func -> TypeB = param  //    parameter. 'param' is the name of the
  );                               //    variable and is for documentation
  ```

- `exec!` is used to terminate the request chain. The result is an
  implementation of the Executor trait on the type. This allows the execute
  method to be called in order to actually perform the request.

  ```rust
  exec!(TypeA);            //<-- Creates an impl of Executor for Type A. This
                           //    is neccessary for every type which needs an
                           //    execute method.
  }
  ```
