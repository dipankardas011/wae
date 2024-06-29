#[allow(warnings)]
mod bindings;

use anyhow::anyhow;
use bindings::exports::dipankardas011::httpclient::outgoing_http::Guest;

use bindings::exports::dipankardas011::httpclient::outgoing_http::Response as WitResponse;
use bindings::exports::dipankardas011::httpclient::outgoing_http::Reserror as WitError;
use bindings::exports::dipankardas011::httpclient::outgoing_http::RequestHeader as WitHeader;

use waki::Client;

struct Component;

impl Guest for Component {
    fn get_request(method: String, headers: Vec<WitHeader>, url: String) -> Result<WitResponse, WitError> {
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

fn execute_request(method: String, usr_headers: Vec<WitHeader>, url: String) -> Result<CustomResponse, anyhow::Error> {
    println!("< UserRequest\n<< Method: {method}\n<< Url: {url}\n<< Headers: {usr_headers:?}\n<");

    let headers: Vec<(&str, &str)> = vec![
        ("Content-Type", "application/json"),
        ("Accept", "*/*"),
        ("User-Agent", "Curl/8.6.0"),
    ];

    // for i in 0..usr_headers.len() {
    //     let h = usr_headers[i].to_owned();
    //     let k = String::new();
    //     let v = String::new();
    //     headers.push((k.as_str(), v.as_str()));
    // }

    let http_client = Client::new();

    let req = match method.to_uppercase().as_str() {
        "GET" => http_client.get(&url).headers(headers),
        "POST" => http_client.post(&url).headers(headers),
        "PUT" => http_client.put(&url).headers(headers),
        "DELETE" => http_client.delete(&url).headers(headers),
        "PATCH" => http_client.patch(&url).headers(headers),
        _ => return Err(anyhow!("Unsupported HTTP method {method}")),
    };

    let resp = req.send();
    match resp {
        Ok(v) => {
            let status_code = v.status_code();
            println!("status code: {}", status_code);
            let headers = v.headers().to_owned();
            let body = String::from_utf8(v.body().unwrap()).expect("Failed to convert to the string");
            Ok(CustomResponse { status_code, headers: format!("{headers:?}"), body })
        }
        Err(e) => Err(anyhow!("Error(HTTP OUTGOING CLIENT) {e:?}"))
    }
}

struct CustomResponse {
    status_code: u16,
    headers: String,
    body: String,
}

bindings::export!(Component with_types_in bindings);

