cargo component new --lib githubapi

wasi = "0.13.1+wasi-0.2.0"


```rs
use wasi::http::types::Fields;
use wasi::http::outgoing_handler::*;
use wasi::http::types::{Scheme, Method};

#[tokio::main(flavor = "current_thread")]

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

    let stau = request.set_authority(Some( "37755ae8-bcfc-4d15-9c60-a896dcad5453.mock.pstmn.io" ));
    if let Err(_) = stau {
        println!("Error: Invalid Authority, couldn't set request authority");
        return Ok(());
    }

    let spwq = request.set_path_with_query(Some( "https://37755ae8-bcfc-4d15-9c60-a896dcad5453.mock.pstmn.io/" ));
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

    let status = response.status();

    let headers = response.headers();

    let baudi = response.consume().unwrap();

    let body = baudi.stream().unwrap().read(1000).unwrap();

    let body_str = String::from_utf8(body).unwrap();

    println!("Response Headers: {headers:?} \n Status: {status:?} \n Body: {body_str}");

    Ok(())
```
