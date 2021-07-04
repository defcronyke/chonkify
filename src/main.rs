use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::env;
// use std::net::SocketAddr;

async fn index(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("no.".into()))
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let mut port: u16 = 8080;
    match env::var("PORT") {
        Ok(p) => {
            match p.parse::<u16>() {
                Ok(n) => {
                    port = n;
                }
                Err(_e) => {}
            };
        }
        Err(_e) => {}
    };
    let addr = ([0, 0, 0, 0], port).into();

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(index))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

// fn main() {
//     pretty_env_logger::init();

//     let mut port: u16 = 8080;
//     match env::var("PORT") {
//         Ok(p) => {
//             match p.parse::<u16>() {
//                 Ok(n) => {
//                     port = n;
//                 }
//                 Err(_e) => {}
//             };
//         }
//         Err(_e) => {}
//     };
//     let addr = ([0, 0, 0, 0], port).into();

//     let new_service = || {
//         service_fn_ok(|_| {
//             let mut hello = "Hello ".to_string();
//             match env::var("TARGET") {
//                 Ok(target) => {
//                     hello.push_str(&target);
//                 }
//                 Err(_e) => hello.push_str("World"),
//             };

//             Response::new(Body::from(hello))
//         })
//     };

//     let server = Server::bind(&addr)
//         .serve(new_service)
//         .map_err(|e| eprintln!("server error: {}", e));

//     println!("Listening on http://{}", addr);

//     rt::run(server);
// }
