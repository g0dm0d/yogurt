use http::{Request, Response};
use hyper::{server::conn::Http, service::service_fn, Body};
use std::{net::SocketAddr, convert::Infallible};
use tokio::net::TcpListener;

#[tauri::command(async)]
pub async fn add_account() {
    run_http_server().await;
}

// #[tokio::main]
async fn run_http_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 9397).into();
    let tcp_listener = TcpListener::bind(addr).await?;
        let (tcp_stream, _) = tcp_listener.accept().await?;
        tokio::task::spawn(async move {
            if let Err(http_err) = Http::new()
                    .http1_only(true)
                    .http1_keep_alive(true)
                    .serve_connection(tcp_stream, service_fn(code_grab))
                    .await {
                eprintln!("Error while serving HTTP connection: {}", http_err);
            }
        });
    Ok(())
}

async fn code_grab(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    if let Some(query) = req.uri().query() {
        for (k, v) in form_urlencoded::parse(query.as_bytes()) {
            if k == "code" {
                let code = v.into_owned();
                println!("{}", code);
                match crate::accounts::api_accounts::get_minecraft_token(&code).await {
                    Ok(token) => {
                        println!("{}", token);
                    },
                    Err(err) => println!("{}", err)
                }
            }
        }
    }


   Ok(Response::new(Body::from("U can close browser")))
}
