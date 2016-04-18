extern crate docopt;
extern crate hyper;
extern crate rustc_serialize;

use docopt::Docopt;
mod check_api;
mod request;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const USAGE: &'static str = "
Usage:
  stockfighter checkapi
  stockfighter -h | --help
  stockfighter -v | --version

Options:
  -h, --help  Show this message
  -v, --version  Show the version of stockfighter
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_help: bool,
    flag_version: bool,
    cmd_checkapi: bool
}

fn main() {

    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    if (args.flag_version)
    {
        println!("StockFighter v{}", VERSION)
    } else if (args.cmd_checkapi)
    {
        let check_api = check_api::check_api();

        if (check_api.ok)
        {
            println!("StockFighter API is up!");
        }
        else
        {
            println!("StockFighter API is down. Error is {}", check_api.error)
        }
    }
}
