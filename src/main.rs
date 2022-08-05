#![deny(warnings)]

use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Request, Response};
use tokio::net::TcpListener;

async fn hello(re: Request<Body>) -> Result<Response<Body>, Infallible> {
    // re.version();
    println!("re.uri -> {}", re.uri());
    // /favicon.ico
    // if re.uri().eq("/favicon.ico") {
    //     let resp = Response::builder().header("Content-Type", "image/*")
    //         .status(StatusCode::OK)
    //         .body(Body::from("ss"))
    //         .unwrap();
    //     return Ok(resp)
    // }
    
    let s = re.uri().to_string();
    let mut params = HashMap::new();
    if s.contains("?") {
        let arr = s.split("?").collect::<Vec<&str>>();
        println!("arr -> {:?}", arr);
        let arr = arr.get(1).unwrap().to_owned().split("&");
        for item in arr {
            let kv = item.split("=").collect::<Vec<&str>>();
            params.insert(kv.get(0).unwrap().to_owned(), kv.get(1).unwrap().to_owned());
        }
    }
    println!("params -> {:?}", params);
    if let (Some(&time_type), Some(&time_str)) = (params.get("time-type"), params.get("time-str")) {
        //  && let Some(time_str) = params.get("time-str")
        println!("time_type = {}, time_str = {}", time_type, time_str);
        // let resp = Response::builder().header("content-type", "application/json");
        match time_type {
            "1" => println!("1111"),
            "2" => println!("2222"),
            _ => {}
        }
    }

    Ok(Response::new(Body::from("Hello World!")))
}


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    // if true {
    //     let s = "aa";
    //     let ss = s.split("?").collect::<Vec<&str>>();
    // }

    pretty_env_logger::init();

    // let addr: SocketAddr = ([127, 0, 0, 1], 3300).into();
    let addr: SocketAddr = ([0, 0, 0, 0], 3300).into();

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = Http::new()
                .serve_connection(stream, service_fn(hello))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}


