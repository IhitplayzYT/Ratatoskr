mod helper;
mod model;
mod ratatoskr;
mod db;
mod tree;
mod conversion;

use helper::Helper;
use ratatoskr::Ratatoskr;
use db::Database;
use conversion::Conversion;
fn main() -> anyhow::Result<()> {
    let mut clargs = Helper::CLI::new();
    clargs.Parse_Args();
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(Conversion::update_exchange_rates())?;

    if clargs.debug {
        println!("{clargs:?}");
    }
    Ok(())
}
