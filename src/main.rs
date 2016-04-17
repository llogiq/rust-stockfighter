extern crate hyper;

mod request;
use self::request::Request;

fn main() {

    let request = Request {
        requires_auth: false,
        request_url: "https://api.stockfighter.io/ob/api/heartbeat"
    };

    println!("Response: {}", request.send_request());
}
