#[allow(warnings)]
mod bindings;

use anyhow::anyhow;
use bindings::exports::dipankardas011::httpclient::outgoing_http::Guest;

use bindings::exports::dipankardas011::httpclient::outgoing_http::{
    Response as WitResponse,
    RequestHeader as WitHeader,
    Reserror as WitError,
};

use waki::Client;
use waki::header::{HeaderName, HeaderValue};

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
    println!("< UserRequest\n<< Method: {method}\n<< Url: {url}\n<\n");

    let mut headers: Vec<(HeaderName, HeaderValue)> = vec![
        // (HeaderName::from_bytes("Content-Name".as_bytes())?,  HeaderValue::from_str("application/json")?),
        // (HeaderName::from_bytes("Accept".as_bytes())?,  HeaderValue::from_str("*/*")?),
        (HeaderName::from_bytes("User-Agent".as_bytes())?,  HeaderValue::from_str("Curl/8.6.0")?),
    ];
    
    for header in usr_headers {
        let name = HeaderName::from_bytes(header.key.as_bytes())?;
        let value = HeaderValue::from_str(header.value.as_str())?;
        headers.push((name, value));
    }
    
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

