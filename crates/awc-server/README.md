# `awc-server`

This crate powers [awc.fly.dev](https://awc.fly.dev).

## Development

1) Install [`rustup`](https://rustup.rs)
1) Install [`volta`](https://volta.sh);;
1) From the root of the repo, run `npm i; npm run dev`
1) Navigate to http://localhost:8080

Making changes to any file impacting the server code will cause a reload.

## Configuring

`NODE_ENV="production"` sources Handlebars values from `awc.prod.json` while everything else uses `awc.dev.json`. The `ui` folder is front-end source code that is transpiled by various tools. Tailwind for CSS, swc for TS->JS, and the handlebars crate/build.rs for HTML substition. `build.rs` also takes care of running steps in parallel where it can.

## Deploying

Build a Docker image locally and map the service to a local port by running `npm run docker:predeploy`. Deploy to fly by running `flyctl deploy` to deploy if you have permissions. You can debug your Docker image by running `npm run docker:debug` and poking around the file system. I used [this article](https://fasterthanli.me/articles/remote-development-with-rust-on-fly-io) as my starting point for this.
