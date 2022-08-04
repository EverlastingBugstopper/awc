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
$ cargo awc lint --schema ./schemas/cat.graphql --watch
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/awc-cli lint --schema ./schemas/cat.graphql --watch`
📚 reading ./schemas/cat.graphql from disk
apollo-compiler validation error

  × cannot find type `Cat` in this document
    ╭─[36:1]
 36 │
 37 │ union Pet = Cat | Dog
    ·             ──┬─
    ·               ╰── not found in this scope
    ╰────

apollo-compiler validation error

  × cannot find type `Dog` in this document
    ╭─[36:1]
 36 │
 37 │ union Pet = Cat | Dog
    ·                   ─┬─
    ·                    ╰── not found in this scope
    ╰────

apollo-compiler validation error

  × cannot find type `Result` in this document
    ╭─[33:1]
 33 │ type Subscription {
 34 │   newMessage: Result
    ·               ───┬──
    ·                  ╰── not found in this scope
 35 │ }
    ╰────

apollo-compiler validation error

  × expected identifier
   ╭─[1:1]
 1 │ ╭─▶ query {
 2 │ │     cat {
 3 │ │       name
 4 │ │     }
 5 │ │   }
 6 │ ├─▶
   · ╰──── provide a name for this definition
 7 │     query getPet {
 8 │       cat {
   ╰────
  help: GraphQL allows a short-hand form for defining query operations when only that one operation exists in the
        document. There are 4 operations in this document.

apollo-compiler validation error

  × the operation `getPet` is defined multiple times in the document
    ╭─[6:1]
  6 │
  7 │ ╭─▶ query getPet {
  8 │ │     cat {
  9 │ │       owner {
 10 │ │         name
 11 │ │       }
 12 │ │     }
 13 │ │   }
 14 │ ├─▶
    · ╰──── previous definition of `getPet` here
 15 │ ╭─▶ query getPet {
 16 │ │     cat {
 17 │ │       treat
 18 │ │     }
 19 │ │   }
 20 │ ├─▶
    · ╰──── `getPet` is redefined here
 21 │     subscription sub {
 22 │       newMessage {
    ╰────
  help: `getPet` must only be defined once in this document.

apollo-compiler validation error

  × Subscriptions operations can only have one root field
    ╭─[20:1]
 20 │
 21 │ ╭─▶ subscription sub {
 22 │ │     newMessage {
 23 │ │       body
 24 │ │       sender
 25 │ │     }
 26 │ │     disallowedSecondRootField
 27 │ │   }
 28 │ ├─▶
    · ╰──── subscription with 2 root fields
 29 │     type Query {
 30 │       cat: Pet
    ╰────
  help: There are 2 root fields: newMessage, disallowedSecondRootField. This is not allowed.

apollo-compiler validation error

  × Cannot query `disallowedSecondRootField` field
    ╭─[25:1]
 25 │   }
 26 │   disallowedSecondRootField
    ·   ─────────────┬────────────
    ·                ╰── `disallowedSecondRootField` field is not in scope
 27 │ }
 28 │
    ╰────
  help: `disallowedSecondRootField` is not defined on `Subscription` type


❌ Found 7 errors in 3 ms.
👀 Watching ./schemas/cat.graphql for changes
```

If you make a change to the schema on the file system, the CLI will detect the change and print the updated diagnostics to the terminal.

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
