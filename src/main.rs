#![deny(warnings)]

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::convert::Infallible;
use std::hash::{Hash, Hasher};
use std::net::SocketAddr;

use chrono::{Local, TimeZone};
use hyper::http::HeaderValue;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Request, Response};
use json::object;
use tokio::net::TcpListener;

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.3f";

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
            "1" => {
                // time_str 为时间戳
                // 1659680082
                // 1659680096469
                // 13 
                // 10 
                // 毫秒戳为 13位
                // 秒戳为   10位
                let time_str = time_str.trim();
                if time_str.len() == 13 {
                    // 毫秒戳
                    let time_num = time_str.parse().unwrap();
                    let parse_res = Local.timestamp_millis(time_num).format(DATE_FORMAT);
                    // let res_0 = object! {};
                    return Ok(Response::new(Body::from(parse_res.to_string())));
                } else if time_str.len() == 10 {
                    // 秒戳
                    let time_num = time_str.parse().unwrap();
                    let parse_res = Local.timestamp(time_num, 0).format(DATE_FORMAT);
                    return Ok(Response::new(Body::from(parse_res.to_string())));
                } else {
                    // 非毫秒戳也非秒戳
                    let mut result_build = String::new();
                    result_build.push_str(
                        "非毫秒戳也非秒戳,秒戳长度为10,毫秒戳长度为13,输入数据长度为:[");
                    result_build.push_str(time_str.trim().len().to_string().as_str());
                    result_build.push_str("]");
                    return Ok(Response::new(Body::from(result_build)));
                }
            },
            "2" => {
                // time_str为格式化日期
                // 如: 2022-01-02 13:22:33.111
                let time_str = time_str.trim();
                let time_res = Local.datetime_from_str(time_str, "%F %H:%M:%S%.3f");
                match time_res {
                    Ok(res) => return Ok(Response::new(Body::from(res.to_string()))),
                    Err(err) => return Ok(Response::new(Body::from(err.to_string()))),
                }
            },
            _ => {
                let res_data = format!("未能识别的日期类型,time-type->[{}]", time_type);
                let res_o = object! {code: 0, data: res_data};
                let mut res = Response::new(Body::from(res_o.to_string()));
                res.headers_mut().insert("content-type", HeaderValue::from_static("application/json"));
                return Ok(res);
            },
        }
    }
    // let cc = format!("{}111", "aaa");

    Ok(Response::new(Body::from("Hello World!")))
}


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    // if true {
    //     let s = "aa";
    //     let ss = s.split("?").collect::<Vec<&str>>();
    // }
    // if true {
    //     let arr = "cc=".split("=").filter(|f|!f.is_empty()).collect::<Vec<&str>>();
    //     println!("arr -> {:?}", arr);
    //     panic!()
    // }

    // if true {
    //     let cc = format!("格式化输出-> {}, [{}]", "ccc", 11);
    //     println!("{}", cc);
    //     panic!()
    // }
    if true {
        let jn = json::parse(r#"
            {
                "a":1,
                "arr": [1,2,3]
            }
        "#).unwrap();
        // jn.
        // PartialEq
        
        println!("is obj -> {}", jn.is_object());
        println!("data -> [{}]", jn);
        let jn_a = &jn["a"];
        println!("jn_a -> {}", jn_a);
        
        let ins = object! {
            success: true,
            code: 200
        };
        println!("data is -> {}", ins);

        println!("item data is -> {}", ins["item"]);

        let b = Book{isbn: 1, mark: "ccc".to_string()};
        println!("book -> {:?}", b);
        let mut hasher = DefaultHasher::new();
        b.hash(&mut hasher);
        println!("hash -> {:?}", hasher.finish());

        // println!("book -> {}", b.isbn);
        // println!("book -> {}", b.mark);

        panic!()
    }

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

#[derive(Debug, PartialEq, Eq, Hash)]
struct Book {
    isbn: i32,
    mark: String,
}



// impl PartialEq for Book {
//     fn eq(&self, other: &Self) -> bool {
//         self.isbn == other.isbn && self.mark == other.mark
//     }
// }

