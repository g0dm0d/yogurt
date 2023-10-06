use http::{Request, Response};
use hyper::{server::conn::Http, service::service_fn, Body};
use std::{convert::Infallible, net::SocketAddr};
use tokio::net::TcpListener;

/// This function starts the http server and waits for a response to localhost:9397 with the "code" parameter.
/// This is necessary to obtain a token to access the ms account
/// After that, the server stops and starts the function to get the minecraft token, user name, uuid
#[tauri::command(async)]
pub async fn add_account() -> Result<(), String> {
    let addr: SocketAddr = ([127, 0, 0, 1], 9397).into();
    let tcp_listener = TcpListener::bind(addr)
        .await
        .map_err(|err| err.to_string())?;
    let (tcp_stream, _) = tcp_listener.accept().await.map_err(|err| err.to_string())?;
    tokio::task::spawn(async move {
        if let Err(http_err) = Http::new()
            .http1_only(true)
            .http1_keep_alive(true)
            .serve_connection(tcp_stream, service_fn(code_grab))
            .await
        {
            return Err(http_err.to_string());
        }
        Ok(())
    });
    Ok(())
}

/// This function is performed after receiving a get request
/// And get the "code" from the url parameter
/// https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow#request-an-authorization-code
async fn code_grab(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    if let Some(query) = req.uri().query() {
        for (k, v) in form_urlencoded::parse(query.as_bytes()) {
            if k == "code" {
                let code = v.into_owned();
                println!("{}", code);
                match crate::accounts::api_accounts::get_access_token(&code).await {
                    Ok(_) => {}
                    Err(err) => println!("{}", err),
                }
            }
        }
    }

    Ok(Response::new(Body::from("U can close browser")))
}
