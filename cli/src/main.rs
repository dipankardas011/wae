#[allow(warnings)]
mod bindings;

use std::env;
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
use anyhow::Result;
use ansi_term;
use ansi_term::Colour::{Cyan, Black, Red, Green, Blue, Yellow};

const FILE_PATH: &str = "README.md";
const OP_CRYPTO: &str = "crypto";
const OP_GITHUBAPI: &str = "githubapi";
const OP_OPENAI: &str = "openai";
const OP_GREEN: &str = "green";
const OP_DEMO: &str = "demo";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CommandToPerform {
    #[arg(short = 'n', long = "name")]
    name: String,

    #[arg(short='o', long="op", value_parser=[OP_CRYPTO, OP_GITHUBAPI, OP_DEMO, OP_OPENAI, OP_GREEN], default_value_t=OP_DEMO.to_string())]
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
        OP_DEMO => {
            println!("{}", Black.paint(format!("Your Name: {}, Op: {}", args.name, args.operation)));

            for (key, value) in env::vars() {
                println!("{}", Black.paint(format!("{key} : {value}")));
            }

            println!("{}", Blue.paint(format!("In file {FILE_PATH}")));

            let contents =
            fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

            println!("Content upto 50 chars: \n-----\n{}\n-----\n", &contents[..50]);

            let now = SystemTime::now();

            sleep(Duration::new(2, 0));
            match now.elapsed() {
                Ok(elapsed) => {
                    println!("{}", Black.paint(format!("Sleeped for {}s", elapsed.as_secs())));
                }
                Err(e) => {
                    eprintln!("{}", Red.bold().paint(format!("Error: {e:?}")));
                }
            }
        }
        _ => eprintln!("{}", Red.bold().paint("Invalid Operation choosen"))
    }

    Ok(())
}

