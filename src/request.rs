extern crate hyper;

use std::io::Read;
use hyper::Client;
use hyper::header::Connection;

pub struct Request{
    pub requires_auth: bool,
    pub request_url: String
}

impl Request {

    pub fn send_request(&self) -> String
    {
        // Create a client.
        let mut client = Client::new();

        // Creating an outgoing request.
        let mut res = client.get(&self.request_url)
            // set a header
            .header(Connection::close())
            // let 'er go!
            .send().unwrap();

        // Read the Response.
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        return body;
    }
}
