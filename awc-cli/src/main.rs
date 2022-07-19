use awc_cli::{AwcCli, Parser, Result};

fn main() -> Result<()> {
    let app = AwcCli::from_args();
    app.run()
}
