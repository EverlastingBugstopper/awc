mod all;
mod css;
mod deps;
mod html;
mod js;

pub(crate) use all::*;
pub(crate) use css::*;
pub(crate) use deps::*;
pub(crate) use html::*;
pub(crate) use js::*;

use crate::framework::prelude::*;

#[derive(Debug, Clone, Parser)]
pub(crate) struct BundleCommand {
    /// Run a specific bundle step
    #[clap(subcommand)]
    bundle_command: BundleCommands,
}

#[derive(Debug, Clone, Parser)]
enum BundleCommands {
    /// Run all bundler steps.
    ///
    /// Runs in parallel where possible.
    All(AllCommands),

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
        match &self.bundle_command {
            BundleCommands::All(command) => command.run(),
            BundleCommands::Deps(command) => command.run(),
            BundleCommands::Css(command) => command.run(),
            BundleCommands::Js(command) => command.run(),
            BundleCommands::Html(command) => command.run(),
        }
    }
}
