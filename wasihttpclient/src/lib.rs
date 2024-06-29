#[allow(warnings)]
mod bindings;

use anyhow::{self, anyhow};
use bindings::exports::dipankardas011::httpclient::outgoing_http::Guest;

use bindings::exports::dipankardas011::httpclient::outgoing_http::Response as WitResponse;
use bindings::exports::dipankardas011::httpclient::outgoing_http::Reserror as WitError;

use waki::Client;

struct Component;

impl Guest for Component {
    fn get_request(method: String, headers: Vec<String>, url: String) -> Result<WitResponse, WitError> {
        let result = execute_request(method, headers, url);
        match result {
            Ok(response) => Ok(WitResponse {
                status_code: response.status_code,
                headers: response.headers,
                body: response.body,
            }),
            Err(e) => {
                println!("Error from request: {}", e);
                Err(WitError{msg: format!("{e:?}")})
            }
        }
    }
}

fn execute_request(method: String, _headers: Vec<String>, url: String) -> Result<CustomResponse, anyhow::Error> {

    headers.append(&String::from("accept").into(), &"*/*".as_bytes().to_vec())?;
    headers.append(&String::from("user-agent").into(), &"wasi".as_bytes().to_vec())?;

    let method = match method.to_uppercase().as_str() {
        "GET" => Method::Get,
        "POST" => Method::Post,
        "PUT" => Method::Put,
        "DELETE" => Method::Delete,
        _ => return Err(anyhow!("Unsupported HTTP method")),
    };
}

struct CustomResponse {
    status_code: u16,
    headers: String,
    body: String,
}

bindings::export!(Component with_types_in bindings);

