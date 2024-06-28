#[allow(warnings)]
mod bindings;

use anyhow::anyhow;
use bindings::exports::dipankardas011::httpclient::outgoing_http::Guest;

use bindings::exports::dipankardas011::httpclient::outgoing_http::Response as WitResponse;

use wasi::http::types::Fields;
use wasi::http::outgoing_handler::*;
use wasi::http::types::{Scheme, Method};

struct Component;

impl Guest for Component {
    fn get_request(method: String, headers: Vec<String>, url: String) -> Result<WitResponse, ()> {
        let result = execute_request(method, headers, url);
        match result {
            Ok(response) => Ok(WitResponse {
                status_code: response.status_code,
                headers: response.headers,
                body: response.body,
            }),
            Err(e) => {
                println!("Error: {}", e);
                Err(())
            }
        }
    }
}

fn execute_request(method: String, _headers: Vec<String>, url: String) -> Result<CustomResponse, anyhow::Error> {
    let headers = Fields::new();

    headers.append(&String::from("accept").into(), &"*/*".as_bytes().to_vec())?;
    headers.append(&String::from("user-agent").into(), &"wasi".as_bytes().to_vec())?;

    let request = OutgoingRequest::new(headers);

    if "http" == url.split("://").nth(0).unwrap() {
        request.set_scheme(Some(&Scheme::Http)).map_err(|_| anyhow!("Invalid Scheme"))?;
    } else if "https" == url.split("://").nth(0).unwrap() {
        request.set_scheme(Some(&Scheme::Https)).map_err(|_| anyhow!("Invalid Scheme"))?;
    } else {
        return Err(anyhow!("invalid scheme"))
    }

    let method = match method.to_uppercase().as_str() {
        "GET" => Method::Get,
        "POST" => Method::Post,
        "PUT" => Method::Put,
        "DELETE" => Method::Delete,
        _ => return Err(anyhow!("Unsupported HTTP method")),
    };
    request.set_method(&method).map_err(|_| anyhow!("Invalid Method"))?;

    let authority = url.split("://").nth(1).ok_or_else(|| anyhow!("Invalid URL format"))?
        .split('/').next().ok_or_else(|| anyhow!("Invalid URL format"))?;
    request.set_authority(Some(authority)).map_err(|_| anyhow!("Invalid Authority"))?;

    request.set_path_with_query(Some(&url)).map_err(|_| anyhow!("Invalid URL"))?;

    let future = handle(request, None);

    if let Err(e) = future {
        println!("Error in future: {e}");
        return Err(e.into());
    }
    let fut = future.unwrap();

    fut.subscribe().block();

    let result = fut.get();
    match result {
        None => {
            println!("No response");
            return Err(anyhow!("No Response"))
        }
        Some(Err(e)) => {
            println!("Error: {e:?}");
            return Err(anyhow!("Error: {e:?}"))
        }
        Some(Ok(Err(e))) => {
            println!("HTTP Error: {e:?}");
            return Err(anyhow!("HTTP Error: {e:?}"))
        }
        Some(Ok(Ok(response))) => {
            let status = response.status();
            let headers = response.headers();
            let body = response.consume().unwrap().stream().unwrap().read(10000).unwrap();
            let body_str = String::from_utf8(body).unwrap();

            println!("Response Headers: {headers:?} \n Status: {status:?} \n Body: {body_str}");

            Ok(CustomResponse {
                status_code: status as u16,
                headers: format!("{headers:?}"),
                body: body_str,
            })
        }
    }
}

struct CustomResponse {
    status_code: u16,
    headers: String,
    body: String,
}

bindings::export!(Component with_types_in bindings);

