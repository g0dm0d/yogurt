use warp::Filter;

fn handle_code(code: &str) {
    println!("Code: {}", code);
}

#[tokio::main]
async fn http_server() {
    let route = warp::get()
        .and(warp::path::end())
        .and(warp::query::<HashMap<String, String>>())
        .map(|query_params| {
            let code = query_params.get("code").unwrap_or(&"".to_string()).clone();
            handle_code(&code);
        });

    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
    let server = warp::serve(route).bind_with_graceful_shutdown(([127, 0, 0, 1], 9397), async {
        let _ = shutdown_rx.await;
    });

    let server_task = tokio::spawn(server);
    shutdown_tx.await;

    server_task.shutdown().await;
}
