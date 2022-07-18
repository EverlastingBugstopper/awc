# awc

This project runs [`apollo-compiler`](https://github.com/apollographql/apollo-rs) on GraphQL schemas as a service.

## Usage

There are many ways to use `awc`. None of them are particularly stable.

### Install the CLI

1) Install [rustup](https://rustup.rs)
1) Run the following:

```
$ cargo install --git https://github.com/EverlastingBugstopper/awc --example cli
```

### With `curl`

```console
$ cat ./src/ui/schemas/prod.graphql| curl -S -X POST --data-binary @- https://awc.fly.dev/validate | jq
{
  "diagnostics": [
    {
      "code": "apollo-compiler validation error",
      "labels": [
        {
          "label": "not found in this scope",
          "span": {
            "length": 8,
            "offset": 34
          }
        }
      ],
      "message": "cannot find type `Resulttt` in this document",
      "pretty": "apollo-compiler validation error\n\n  × cannot find type `Resulttt` in this document\n   ╭─[1:1]\n 1 │ type Subscription {\n 2 │   newMessage: Resulttt\n   ·               ────┬───\n   ·                   ╰── not found in this scope\n 3 │ }\n   ╰────\n",
      "severity": "error"
    }
  ],
  "success": false
}
```

### In the browser _(under development)_

Visit [awc.fly.dev](https://awc.fly.dev), type GraphQL into the left panel and watch diagnostics appear on the right. Display for this is a bit buggy at the moment but the messages should be good.