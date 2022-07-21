use awc_cli::{AwcCli, Result};
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

fn main() -> Result<()> {
    let mut builder = Builder::from_default_env();
    builder
        .filter(None, LevelFilter::Info)
        .filter_module("salsa", LevelFilter::Off)
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();
    AwcCli::run_from_args()
}
