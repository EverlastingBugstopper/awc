# awc (apollo-web-compiler)

This project runs [`apollo-compiler`](https://github.com/apollographql/apollo-rs) on GraphQL schemas as a service.

## Usage

There are many ways to use `awc`. None of them are particularly stable.

### Run the CLI

1) Install [rustup](https://rustup.rs)
1) Get a [GraphQL Schema](./schemas/prod.graphql) and save it to `document.graphql`
1) Clone this repo.
1) Run the following:

```console
$ cargo awc lint --schema document.graphql
apollo-compiler validation error

  × cannot find type `Resulttt` in this document
   ╭─[1:1]
 1 │ type Subscription {
 2 │   newMessage: Resulttt
   ·               ────┬───
   ·                   ╰── not found in this scope
 3 │ }
   ╰────
```

### With `rover graph introspect`, `curl`, and `jq`

```console
$ rover graph introspect https://countries.trevorblades.com | curl -X POST -sSL --data-binary @- https://awc.fly.dev | jq -r .pretty
apollo-compiler validation advice

  > Custom scalars should provide a scalar specification URL via the @specifiedBy directive
   ,-[1:1]
 1 | scalar _Any
   : ^^^^^^|^^^^^
   :       `-- consider adding a @specifiedBy directive to this scalar definition
 2 | type Country {
 3 |   code: ID!
   `----
```

### In the browser _(under development)_

Visit [awc.fly.dev](https://awc.fly.dev), type GraphQL into the left panel and watch diagnostics appear on the right. Display for this is a bit buggy at the moment but it uses the same POST request to validate GraphQL.
