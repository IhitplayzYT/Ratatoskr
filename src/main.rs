mod helper;
mod model;
mod ratatoskr;
mod db;
mod tree;

use helper::Helper;
use ratatoskr::Ratatoskr;
use db::Database;

fn main() {
    let mut clargs = Helper::CLI::new();
    clargs.Parse_Args();
    if clargs.debug {
        println!("{clargs:?}");
    }
}
