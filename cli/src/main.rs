#[allow(warnings)]
mod bindings;

use clap::Parser;
use std::fs;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use bindings::dipankardas011::{
    crypto::password::generate_random,
    githubapi::releases,
    openai::llm,
    watttime,
};
use waki::{handler, ErrorCode, Request, Response};
use anyhow::Result;
use ansi_term;
use ansi_term::Colour::{Cyan, Red, Green, Blue, Yellow};

const OP_CRYPTO: &str = "crypto";
const OP_GITHUBAPI: &str = "githubapi";
const OP_OPENAI: &str = "openai";
const OP_GREEN: &str = "green";
const INDEX_HTML: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>WASI Server</title>
    <style>
        body {
            text-align: center;
        }
        table {
            margin-left: auto;
            margin-right: auto;
        }
    </style>
</head>
<body>
    <h1>Welcome to WASI Server</h1>
    <p>Thanks for running it</p>
    <p>Created in collobration with Dipankar, Joel, and others</p>
    <table>
        <tr>
            <th>Method</th>
            <th>Endpoint</th>
            <th>Description</th>
        </tr>
        <tr>
            <td>GET</td>
            <td>/healthz</td>
            <td>Health Check</td>
        </tr>
        <tr>
            <td>GET</td>
            <td>/</td>
            <td>Home Page</td>
        </tr>
        <tr>
            <td>PUT</td>
            <td>/get-lazy?seconds={integer}</td>
            <td>Get Lazy</td>
        </tr>
        <tr>
            <td>GET</td>
            <td>/**</td>
            <td>Serve as a reverse Proxy</td>
        </tr>
    </table>
</body>
</html>
"#;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CommandToPerform {
    #[arg(short = 'n', long = "name")]
    name: String,

    #[arg(short='o', long="op", value_parser=[OP_CRYPTO, OP_GITHUBAPI, OP_OPENAI, OP_GREEN])]
    operation: String,
}

async fn hh(name: &str) {
    println!("{} {} {}",
        Yellow.bold().paint("@@"),
        format!("{} {} {}", Green.paint("Welcome"), Blue.bold().paint(name), Green.paint("🚀 to WASI CLI based on Component Model")),
        Yellow.bold().paint("@@"),
    );
}


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {

    let args = CommandToPerform::parse();

    hh(&args.name).await;

    #[handler]
    fn hello(req: Request) -> Result<Response, ErrorCode> {
        println!("client_request: {:?} {:?}", req.method(), req.url());
        match (req.method(), req.path()) {
            (waki::Method::Get, "/healthz") => {
                println!("Health Check");
                Response::builder()
                    .status_code(200)
                    .headers([("Content-Type", "text/plain"), ("Server", "WASI@v0.2")])
                    .body(
                        "Server is READY".as_bytes(),
                    )
                    .build()
            }

            (waki::Method::Get, "/") => {
                Response::builder()
                    .status_code(200)
                    .headers([("Content-Type", "text/html"), ("Server", "WASI@v0.2")])
                    .body(
                        INDEX_HTML.as_bytes(),
                    )
                    .build()
            }


            (waki::Method::Put, "/get-lazy") => {
                let query = req.query();
                let now = SystemTime::now();
                let sleep_duration = match query.get("seconds") {
                    Some(s) => s.parse::<u64>().unwrap_or(1),
                    None => 1,
                };
                sleep(Duration::new(sleep_duration, 0));
                if let Err(e) = now.elapsed() {
                    return Response::builder()
                        .body(
                            format!(
                                "Error: {e:?}"
                            ).as_bytes(),
                        )
                        .build();
                }

                Response::builder()
                    .body(
                        format!(
                            "Thanks for making server lazy for, {}s!",
                            query.get("seconds").unwrap_or(&"1".to_string())
                        )
                            .as_bytes(),
                    )
                    .build()
            }

            (waki::Method::Get, _) => {
                let file_loc = req.path();

                println!("{}", Blue.paint(format!("In file {:?}", file_loc)));

                match fs::read_to_string(file_loc) {
                    Ok(contents) => {
                        Response::builder()
                            .status_code(200)
                            .headers([("Content-Type", "text/plain"), ("Server", "WASI@v0.2")])
                            .body(
                                format!("reverseproxy\n====\n{}",contents).as_bytes(),
                            )
                            .build()
                    }
                    Err(e) => {
                        Response::builder()
                            .status_code(404)
                            .body(
                                format!(
                                    "Error: {e:?}"
                                ).as_bytes(),
                            )
                            .build()
                    }
                }
            }

            _ => Response::builder()
                .status_code(404)
                .body("Not Found".as_bytes())
                .build()
        }
    }

    match args.operation.as_str() {
        OP_GREEN => {
            println!("{}", Cyan.paint("> Enter [1] register [2] get region code based on curr loc [3] get forecast based on current loc [4] get current CO2 MOER index"));
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let choice: i32 = input.trim().parse().expect("Invalid Input");
            if choice < 1 || choice > 4 {
                eprintln!("{}", Red.bold().paint("Invalid choice"));
            }
            if choice == 1 {
                println!("{}", Cyan.paint(" > Enter UserName"));
                let mut input_username = String::new();
                std::io::stdin().read_line(&mut input_username).expect("Failed to read line");
                let username: String = input_username.trim().parse().expect("invalid organization");

                println!("{}", Cyan.paint(" > Enter Password"));
                let mut input_password = String::new();
                std::io::stdin().read_line(&mut input_password).expect("Failed to read line");
                let password: String = input_password.trim().parse().expect("invalid organization");

                println!("{}", Cyan.paint(" > Enter Email"));
                let mut input_email = String::new();
                std::io::stdin().read_line(&mut input_email).expect("Failed to read line");
                let email: String = input_email.trim().parse().expect("invalid organization");

                watttime::watttime::register(&username, &password, &email);
            } else if choice == 2 {

                println!("{}", Cyan.paint(" > Enter Watttime Signal type co2_moer or health_damage: "));
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Failed to read line");
                let choice: String= input.trim().parse().expect("Invalid Input");
                if choice != "co2_moer" && choice != "health_damage" {
                    eprintln!("{}", Red.bold().paint("Invalid choice"));
                }

                let token = watttime::watttime::get_token();
                if let None = token {
                    eprintln!("{}", Red.bold().paint("Failed to get token"));
                }
                let t = token.unwrap();
                let region_code = watttime::watttime::get_region(&t, &choice);
                match region_code {
                    Some(code) => {
                        println!("{}: {code}", Green.bold().paint("Region Code"));
                    }
                    None => {
                        eprintln!("{}", Red.bold().paint("Failed to get region code"));
                    }
                }
            } else if choice == 3 {
                println!("{}", Cyan.paint(" > Enter Watttime Signal type co2_moer or health_damage: "));
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Failed to read line");
                let choice: String= input.trim().parse().expect("Invalid Input");
                if choice != "co2_moer" && choice != "health_damage" {
                    eprintln!("{}", Red.bold().paint("Invalid choice"));
                }

                let token = watttime::watttime::get_token();
                if let None = token {
                    eprintln!("{}", Red.bold().paint("Failed to get token"));
                }
                let t = token.unwrap();
                let region_code = watttime::watttime::get_region(&t, &choice);
                match region_code {
                    Some(code) => {
                        println!("{}: {code}", Green.bold().paint("Region Code"));

                        let dd = watttime::watttime::get_forecast(&t, &code, &choice);
                        match dd {
                            Some(d) => {
                                println!("{}\n{d:?}", Blue.paint("Most recently generated forecast for the specified region and signal_type. Forecasts are generated periodically (e.g. at 5-minute frequency, this frequency is described in the generated_at_period_seconds metadata), and there is a data list made up of point_time and value pairs in the forecast horizon (e.g. a 24-hr forecast horizon with 5-min frequency results in 288 values). Each forecast response is valid starting from its generated_at time until it is superseded by a new forecast with a new generated_at time, and each point_time in the data list is valid from its point_time for the duration described in data_point_period_seconds"));
                            }
                            None => {
                                eprintln!("{}", Red.bold().paint("Failed to get forecast"));
                            }
                        }
                    }
                    None => {
                        eprintln!("{}", Red.bold().paint("Failed to get region code"));
                    }
                }
            } else {
                let token = watttime::watttime::get_token();
                if let None = token {
                    eprintln!("{}", Red.bold().paint("Failed to get token"));
                }
                let t = token.unwrap();
                let region_code = watttime::watttime::get_region(&t, "");
                match region_code {
                    Some(code) => {
                        println!("{}: {code}", Green.bold().paint("Region Code"));
                        let dd = watttime::watttime::get_current_co2_moer_index(&t, &code, ""); // current
                        // only co2_moer
                        match dd {
                            Some(d) => {
                                println!("{}\n{d:?}", Blue.paint("Current Index value for the specified region for the co2_moer signal type. This 0-100 value is the statistical percentile of the current MOER relative to the upcoming 24 hours of forecast MOER values for the specified location (100=dirtiest, 0=cleanest). Values are updated periodically (e.g. at 5-minute frequency), and each value is valid starting from its point_time for the duration described in data_point_period_seconds. No historical query is available for this endpoint."));
                            }
                            None => {
                                eprintln!("{}", Red.bold().paint("Failed to get forecast"));
                            }
                        }
                    }
                    None => {
                        eprintln!("{}", Red.bold().paint("Failed to get region code"));
                    }
                }
            }
        }
        OP_OPENAI => {
            println!("{}", Cyan.paint("> Enter [1] text-to-text [2] text-to-image"));
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let choice: i32 = input.trim().parse().expect("Invalid Input");
            if choice < 1 || choice > 2 {
                eprintln!("{}", Red.bold().paint("Invalid choice"));
            }
            if choice == 1 {
                llm::text_to_text();
            } else if choice == 2 {
                llm::text_to_image();
            }
        }
        OP_CRYPTO => {
            println!("{}", Cyan.paint("> Enter Length of Password"));
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let length_pass: u32 = input.trim().parse().expect("Invalid Input");

            let gen_pass = generate_random(length_pass);

            println!("{} pass={}", Green.bold().paint("Created password"),gen_pass);
        }
        OP_GITHUBAPI => {

            println!("{}", Cyan.paint("> Enter [1] latest release [2] contributors [3] stars"));
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let choice: i32 = input.trim().parse().expect("Invalid Input");
            if choice < 1 || choice > 3 {
                eprintln!("{}", Red.bold().paint("Invalid choice"));
            }

            println!("{}", Cyan.paint(" > Enter Organization Name"));
            let mut input_org = String::new();
            std::io::stdin().read_line(&mut input_org).expect("Failed to read line");
            let org: String = input_org.trim().parse().expect("invalid organization");

            println!("{}", Cyan.paint(" > Enter Repo Name"));
            let mut input_proj = String::new();
            std::io::stdin().read_line(&mut input_proj).expect("Failed to read line");
            let proj: String = input_proj.trim().parse().expect("invalid organization");

            if choice == 1 {
                let ver = releases::get_latest_release(&org, &proj);
                println!("{}: {ver}", Green.bold().paint("Latest Release"));
            }
            else if choice ==2 {
                let contrib = releases::get_contributors(&org, &proj);
                println!("{}\n{contrib}", Green.bold().paint("Contributors"));
            }
            else {
                let stars = releases::get_stars(&org, &proj);
                println!("{}={stars}", Green.bold().paint("Total Stars"));
            }

        }
        _ => eprintln!("{}", Red.bold().paint("Invalid Operation choosen"))
    }

    Ok(())
}

