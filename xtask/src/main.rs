use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use xtask::{Result, Xtask};

fn main() -> Result<()> {
    let mut builder = Builder::from_default_env();
    builder
        .filter(None, LevelFilter::Info)
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();
    Xtask::run_from_args()
}
