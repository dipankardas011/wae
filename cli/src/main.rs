#[allow(warnings)]
mod bindings;

// use std::env;
use clap::Parser;
// use std::fs;
// use std::time::{Duration, SystemTime};
// use std::thread::sleep;
use wasi::http::outgoing_handler::*;
use wasi::http::types::{Scheme, Method};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CommandToPerform {
    #[arg(short = 'n', long = "name")]
    name: String,

    #[arg(short='o', long="op", value_parser=[OP_GENERATE_RANDOM_PASSWORD, OP_PROJ_LATEST_RELEASE, "demo"], default_value_t=String::from("demo"))]
    operation: String,
}


const FILE_PATH: &str = "README.md";
const OP_GENERATE_RANDOM_PASSWORD: &str = "gen_rand_pass";
const OP_PROJ_LATEST_RELEASE: &str = "pro_latest_release";


// use bindings::dipankardas011::{ crypto::password::generate_random, githubapi::releases::fetch_latest };


use anyhow::Result;
use wasi::http::types::Fields;
// use spin_sdk::{
//     http::{IntoResponse, Request, Response},
//     http_component,
// };
// use spin_sdk::wit::wasi;
//
// /// Send an HTTP request and return the response.
// #[http_component]
// async fn send_outbound(_req: Request) -> Result<impl IntoResponse> {
//     let resp: Response = spin_sdk::http::send(Request::get(
//    "https://random-data-api.fermyon.app/animals/json",
//     ))
//     .await?;
//     let resp = resp
//         .into_builder()
//         .header("spin-component", "rust-outbound-http")
//         .build();
//     println!("{resp:?}");
//     Ok(resp)
// }

async fn hh() {
    println!(" @@ Called Async GG @@");
}






#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {

    {
        // if args.operation == OP_GENERATE_RANDOM_PASSWORD {
        //     println!(" > Enter Length of Password");
        //     let mut input = String::new();
        //     std::io::stdin().read_line(&mut input).expect("Failed to read line");
        //     std::io::stdin().read_line(&mut input).expect("Failed to read line");
        //     let length_pass: u32 = input.trim().parse().expect("Invalid Input");
        //
        //     let gen_pass = generate_random(length_pass);
        //
        //     println!("Created password '{gen_pass}'");
        //
        // } else if args.operation == OP_PROJ_LATEST_RELEASE {
        //     println!(" > Enter Organization Name");
        //     let mut input_org = String::new();
        //     std::io::stdin().read_line(&mut input_org).expect("Failed to read line");
        //     let org: String = input_org.trim().parse().expect("invalid organization");
        //
        //     println!(" > Enter Repo Name");
        //     let mut input_proj = String::new();
        //     std::io::stdin().read_line(&mut input_proj).expect("Failed to read line");
        //     let proj: String = input_proj.trim().parse().expect("invalid organization");
        //
        //     let ver = fetch_latest(&org, &proj);
        //     println!("Latest version: {ver}");
        //     let _ = send_outbound(Request::new(spin_sdk::http::Method::Get, "")).await;
        //
        // } else {
        //     println!("Your Name: {}, Op: {}", args.name, args.operation);
        //
        //     for (key, value) in env::vars() {
        //         println!("{key} : {value}");
        //     }
        //
        //     println!("In file {FILE_PATH}");
        //
        //     let contents =
        //         fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
        //
        //     println!("Content upto 50 chars: {}\n", &contents[..50]);
        //
        //     let now = SystemTime::now();
        //
        //     sleep(Duration::new(2, 0));
        //     match now.elapsed() {
        //         Ok(elapsed) => {
        //             println!("Sleeped for {}s", elapsed.as_secs());
        //         }
        //         Err(e) => {
        //             println!("Error: {e:?}");
        //         }
        //     }
        // }
    }


    let args = CommandToPerform::parse();

    hh().await;

    let headers = Fields::new();

    headers.append(&String::from("accept").into(), &"*/*".as_bytes().to_vec())?;
    headers.append(&String::from("user-agent").into(), &"wasi".as_bytes().to_vec())?;
    // headers.append(&String::from("host").into(), &"LDKJFLDHFKL".as_bytes().to_vec())?;

    let request = OutgoingRequest::new(headers);

    let stsc = request.set_scheme(Some( &Scheme::Https ));

    if let Err(_) = stsc {
        println!("Error: Invalid Scheme, couldn't set request scheme");
        return Ok(());
    }

    let stmd = request.set_method(&Method::Get);

    if let Err(_) = stmd {
        println!("Error: Invalid Method, couldn't set request method");
        return Ok(());
    }

    let spwq = request.set_path_with_query(Some( "37755ae8-bcfc-4d15-9c60-a896dcad5453.mock.pstmn.io/path?q=1#something" ));
    if let Err(_) = spwq {
        println!("Error: Invalid URL or something, couldn't set request url");
        return Ok(());
    }

    let fut = handle( request, None );

    println!("After Future");
    if let Err(e) = fut {
        println!("Error: {e}");
        return Err(e.into());
    }

    let fut = fut.unwrap();

    fut.subscribe().block();

    // fut is now ready to be harvested

    let result = fut.get();
    println!("Above Match");
    let response = match result {
        None => {
            println!("No response");
            None
        }
        Some(Err(e)) => {
            println!("Error: {e:?}");
            None
        }
        Some(Ok(Err(e))) => {
            println!("HTTP Errort: {e:?}");
            None
        }
        Some(Ok(Ok(response))) => {
            Some(response)
        }
    };

    if let None = response {
        return Ok(());
    }

    let response = response.unwrap();

    println!("Incoming Response: {response:?}");

    Ok(())

}

