#[allow(warnings)]
mod bindings;

use bindings::exports::dipankardas011::githubapi::releases::Guest;
use serde_json::Value;
use waki::Client;
use anyhow;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn fetch_latest(org: String, proj: String) -> String {
        let url = format!("https://api.github.com/repos/{org}/{proj}/releases/latest");
        println!("Url: {url}");

        let req = Client::new()
            .get(&url)
            .headers([("Content-Type", "application/json"), ("Accept", "*/*"), ("User-Agent", "Curl/8.6.0")]);

        let resp = req.send();

        match resp {
            Ok(v) => {
                println!("status code: {}", v.status_code());
                let body = String::from_utf8(v.body().unwrap()).expect("Failed to convert to the string");
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
