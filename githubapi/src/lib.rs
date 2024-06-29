#[allow(warnings)]
mod bindings;

use bindings::exports::dipankardas011::githubapi::releases::Guest;
use serde_json::Value;
use bindings::dipankardas011::httpclient::outgoing_http::{
    get_request,
    RequestHeader,
};

struct Component;

impl Guest for Component {
    /// Say hello!
    fn fetch_latest(org: String, proj: String) -> String {
        let url = format!("https://api.github.com/repos/{org}/{proj}/releases/latest");
        println!("Url: {url}");

        let headers = vec![
            RequestHeader{
                key: "Content-Type".to_string(),
                value: "application/json".to_string(),
            },
            RequestHeader{
                key: "Accept".to_string(),
                value: "*/*".to_string(),
            },
        ];

        match get_request("GET", &headers, &url) {
            Ok(v) => {
                let status_code = v.status_code as u16;
                println!("status code: {}", status_code);
                let body = v.body;
                if status_code != http::StatusCode::OK {
                    return format!("Failed as status code: {status_code}, body: {body}")
                }
                
                let json: Value = serde_json::from_str(&body).unwrap();

                let tag_name = json["tag_name"]
                    .as_str()
                    .map(|s| s.to_string())
                    .ok_or_else(|| anyhow::anyhow!("failed to find the latest release. Reason: '.tag_name' not found in response"));

                tag_name.unwrap()
            }
            Err(e) => format!("error: {e}")
        }
    }
}

bindings::export!(Component with_types_in bindings);
