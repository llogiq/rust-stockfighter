extern crate hyper;
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
mod request;
use self::request::Request;

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
        let request = Request {
                requires_auth: false,
                request_url: "https://api.stockfighter.io/ob/api/heartbeat"
            };

        println!("Response: {}", request.send_request());
    }
}
