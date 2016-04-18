extern crate rustc_serialize;

use rustc_serialize::json;
use request::Request;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Check_api_response{
    pub ok: bool,
    pub error: String
}

pub fn check_api() -> Check_api_response {

    let request = Request {
            requires_auth: false,
            request_url: "https://api.stockfighter.io/ob/api/heartbeat"
        };

    let response = request.send_request();

    let deserialized: Check_api_response = json::decode(&response).unwrap();

    return deserialized;
}
