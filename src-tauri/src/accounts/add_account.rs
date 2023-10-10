use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use super::api_accounts::get_access_token;

const HTTP_STATUS_OK: &str = "200 OK";

/// This function starts the http server and waits for a response to localhost:9397 with the "code" parameter.
/// This is necessary to obtain a token to access the ms account
/// After that, the server stops and starts the function to get the minecraft token, user name, uuid
#[tauri::command(async)]
pub async fn add_account() -> Result<(), String> {
    let listener = TcpListener::bind("127.0.0.1:9397").unwrap();

    // Accept 1 time so that the process does not hang in the background after the user logs in.
    let stream = listener.accept();
    let stream = stream.map_err(|err| err.to_string())?;

    // get here only tcp stream
    let result = code_grab(stream.0).await;
    match result {
        Ok(_) => return Ok(()),
        Err(err) => return Err(err),
    }
}

/// This function is performed after receiving a get request
/// And get the "code" from the url parameter
/// https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow#request-an-authorization-code
async fn code_grab(mut stream: TcpStream) -> Result<(), String> {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // first line is always method + path + version of protocol
    // reference: https://www.rfc-editor.org/rfc/rfc9110.html#name-example-message-exchange
    let url_param = http_request.get(0).unwrap();

    let query = get_query_params(&url_param);
    let code = query.get("code").ok_or("code not found")?;
    let result = get_access_token(code).await;
    match result {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }

    let answ = "<h1>Account would have been added, you can close the tab!</h1>";
    write_answ(stream, HTTP_STATUS_OK, answ);
    Ok(())
}

/// this function converts the request parameter into a hashmap
///
/// # Examples
///
/// ```
/// use yogurt::accounts::add_account::get_query_params;
/// let url_param = "GET /path?code=super.s3cret&foo=bar HTTP/1.1";
/// let query = get_query_params(url_param);
///
/// assert_eq!(&"bar", query.get("foo").unwrap());
/// assert_eq!(&"super.s3cret", query.get("code").unwrap());
/// ```
pub fn get_query_params(path: &str) -> HashMap<&str, &str> {
    let parsed_path: Vec<&str> = path.split_whitespace().collect();
    // [0]=GET [1]=/some?path= [2]=HTTP/1.1
    let url_path_query = parsed_path.get(1).unwrap().split_once('?').unwrap();
    let url_query: Vec<&str> = url_path_query.1.split('&').collect();

    let mut query: HashMap<&str, &str> = HashMap::new();
    for param in url_query {
        let values: Vec<&str> = param.split('=').collect();
        if values.len() == 2 {
            query.insert(values[0], values[1]);
        }
    }
    return query;
}

/// this function returns a response to the client
pub fn write_answ(mut stream: TcpStream, status: &str, body: &str) {
    let resp = format!(
        "HTTP/1.1 {status}\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    );

    stream.write_all(resp.as_bytes()).unwrap();
}
