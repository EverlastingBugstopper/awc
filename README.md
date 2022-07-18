# awc

This project runs [`apollo-compiler`](https://github.com/apollographql/apollo-rs) on GraphQL schemas as a service.

## Usage

There are many ways to use `awc`. None of them are particularly stable.

### Run the CLI

1) Install [rustup](https://rustup.rs)
1) Get a [GraphQL Schema](./awc-server/schemas/prod.graphql) and save it to `document.graphql`
1) Run the following:

```console
$ cargo install --git https://github.com/EverlastingBugstopper/awc -p awc-cli
$ awc-cli lint --file document.graphql
apollo-compiler validation error\n\n  × cannot find type `Result` in this document\n   ╭─[2:1]\n 2 │ {\n 3 │   newMessage: Result\n   ·               ───┬──\n   ·                  ╰── not found in this scope\n 4 │ }\n   ╰────\n
```

### With `rover graph introspect` and `curl`

```console
$ rover graph introspect https://countries.trevorblades.com | curl -S -X POST --data-binary @- https://awc.fly.dev/validate | jq
{
  "diagnostics": [
    {
      "code": "apollo-compiler validation advice",
      "labels": [
        {
          "label": "consider adding a @specifiedBy directive to this scalar definition",
          "span": {
            "length": 12,
            "offset": 0
          }
        }
      ],
      "message": "Custom scalars should provide a scalar specification URL via the @specifiedBy directive",
      "pretty": "apollo-compiler validation advice\n\n  ☞ Custom scalars should provide a scalar specification URL via the\n  │ @specifiedBy directive\n   ╭─[1:1]\n 1 │ scalar _Any\n   · ──────┬─────\n   ·       ╰── consider adding a @specifiedBy directive to this scalar definition\n 2 │ type Country {\n 3 │   code: ID!\n   ╰────\n",
      "severity": "advice"
    }
  ],
  "success": false
}
```

### In the browser _(under development)_

Visit [awc.fly.dev](https://awc.fly.dev), type GraphQL into the left panel and watch diagnostics appear on the right. Display for this is a bit buggy at the moment but it uses the same POST request to validate GraphQL.
