mod all;
mod bucket;
mod css;
mod deps;
mod html;
mod js;

pub(crate) use all::*;
pub(crate) use bucket::*;
pub(crate) use css::*;
pub(crate) use deps::*;
pub(crate) use html::*;
pub(crate) use js::*;

use saucer::prelude::*;

#[derive(Debug, Clone, Parser)]
pub(crate) struct BundleCommand {
    /// Run a specific bundle step
    #[clap(subcommand)]
    bundle_command: Option<BundleCommands>,

    #[clap(flatten)]
    all_opts: AllOpts,
}

#[derive(Debug, Clone, Subcommand)]
enum BundleCommands {
    /// Run all bundler steps.
    ///
    /// Runs in parallel where possible.
    All(AllCommands),

    /// Copy files from bucket to public
    Bucket(BucketCommand),

    /// Install node dependencies.
    Deps(DepsCommand),

    /// Bundle CSS with Tailwind.
    Css(CssCommand),

    /// Build JavaScript with swc via webpack.
    Js(JsCommand),

    /// Insert values from awc.json into HTML templates.
    Html(HtmlCommand),
}

impl BundleCommand {
    /// Run a bundle subcommand
    pub(crate) fn run(&self) -> Result<()> {
        if let Some(bundle_command) = &self.bundle_command {
            match bundle_command {
                BundleCommands::All(command) => command.run(),
                BundleCommands::Bucket(command) => command.run(),
                BundleCommands::Deps(command) => command.run(),
                BundleCommands::Css(command) => command.run(),
                BundleCommands::Js(command) => command.run(),
                BundleCommands::Html(command) => command.run(),
            }
        } else {
            AllCommands {
                opts: self.all_opts.clone(),
            }
            .run()
        }
    }
}
