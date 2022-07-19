# awc-cli

This is a little CLI that wraps [`apollo-compiler`](https://github.com/apollographql/apollo-rs).

## Usage

```console
$ cargo run -p awc-cli -- lint --file ./schemas/prod.graphql
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
