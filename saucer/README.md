# `saucer`

Saucer is a library crate that allows for rapid development of CLI tools. 

Under the hood it uses [`clap`](https://crates.io/crates/clap) for argument parsing, [`anyhow`](https://crates.io/crates/anyhow) for error handling, [`rayon`](https://crates.io/crates/) for running things in parallel.

## Usage

`use saucer::prelude::*` will get you pretty far. The main thing to do is to implement the `Saucer` trait.

Here's an example of a `Saucer` that wraps an [npm script](https://docs.npmjs.com/cli/v8/commands/npm-run-script):


```rust
const EMOJI: &str = "💅 ";

use saucer::{prelude::*, Process};

#[derive(Default, Clone, Copy, Debug, Parser)]
pub(crate) struct CssCommand {}

impl CssCommand {
    /// Creates a new CssCommand
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl Saucer for CssCommand {
    /// Runs tailwind to generate only the CSS we need
    fn beam(&self) -> Result<()> {
        Process::new("npm", &["run", "build:css"]).run(EMOJI)
    }

    fn prefix(&self) -> String {
        EMOJI.to_string()
    }

    fn description(&self) -> String {
        "tailwindcss".to_string()
    }
}
```

When run, the output looks like this:

```console
$ cargo xtask web bundle css
💅  $ npm run build:css
💅  
💅  > awc-web-toolchain@1.0.0 build:css
💅  > tailwindcss -i ./awc-web/src/browser/index.css -o ./awc-web/public/index.css
💅  
💅  
💅  🌼 daisyUI components 2.19.0  https://github.com/saadeghi/daisyui
💅    ✔︎ Including:  base, components, themes[29], utilities
💅    
💅  
💅  Done in 404ms.
🎉 Success in 1 seconds, 289 ms!
```

### Parallelism

One of the most useful things about implementing `Saucer` is that you can run them in parallel with each other in stages.

Here's an example of installing npm dependencies, and then running the subsequent build steps in parallel:

```rust
const STAGE_PREFIX: &str = "🛸 stage ";

use crate::web::bundle::{CssCommand, DepsCommand, JsCommand};
use saucer::{prelude::*, EmptySaucer, ParallelSaucer};

#[derive(Clone, Debug, Parser)]
pub(crate) struct InstallAndBuild {}

impl Saucer for InstallAndBuild {
    fn description(&self) -> String {
        "install deps and build app".to_string()
    }

    /// Runs all bundle steps, parallelizing where possible
    fn beam(&self) -> Result<()> {
        // stage 1 of 2
        self.install_deps(1, 2).beam()?;

        // stage 2 of 2
        self.css_and_js(2, 2).beam()?;
        Ok(())
    }

    fn prefix(&self) -> String {
        STAGE_PREFIX.to_string()
    }
}

impl InstallAndBuild {
    fn install_deps(
        &self,
        current_stage: usize,
        total_stages: usize,
    ) -> ParallelSaucer<DepsCommand, EmptySaucer> {
        ParallelSaucer::new(
            DepsCommand::new(),
            // we don't want to run `DepsCommand` in parallel with anything, so pass it an `EmptySaucer`
            EmptySaucer::new(),
            &self.prefix(),
            current_stage,
            total_stages,
        )
    }

    fn css_and_js(
        &self,
        current_stage: usize,
        total_stages: usize,
    ) -> ParallelSaucer<CssCommand, JsCommand> {
        ParallelSaucer::new(
            CssCommand::new(),
            JsCommand::new(),
            &self.prefix(),
            current_stage,
            total_stages,
        )
    }
}


```

The output of this command will look like this:

```console
⬇️   $ npm install
⬇️   
⬇️   up to date, audited 220 packages in 511ms
⬇️   
⬇️   32 packages are looking for funding
⬇️     run `npm fund` for details
⬇️   
⬇️   found 0 vulnerabilities
🛸 stage [1/2] ⬇️  installing npm dependencies completed in 734 ms
💅  $ npm run build:css
🧳  $ npm run build:js
🧳  
🧳  > awc-web-toolchain@1.0.0 build:js
🧳  > webpack
🧳  
🧳  asset index.js 1.99 KiB [compared for emit] [minimized] (name: main) 1 related asset
🧳  ./awc-web/src/browser/index.ts 3.08 KiB [built] [code generated]
🧳  webpack 5.73.0 compiled successfully in 315 ms
💅  
💅  > awc-web-toolchain@1.0.0 build:css
💅  > tailwindcss -i ./awc-web/src/browser/index.css -o ./awc-web/public/index.css
💅  
💅  
💅  🌼 daisyUI components 2.19.0  https://github.com/saadeghi/daisyui
💅    ✔︎ Including:  base, components, themes[29], utilities
💅    
💅  
💅  Done in 418ms.
🛸 stage [2/2] 💅 tailwindcss & 🧳 webpack/swc completed in 1 seconds, 67 ms
🎉 Succeeded in 1 seconds, 802 ms!
```

As you can see, the npm install is run in the first stage, and the JS build and CSS build are run in parallel afterwards.