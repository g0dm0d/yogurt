use std::fs::{self, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use toml::{self, Value};


use crate::tools::path;


/// This func save account info
/// 
/// Save username, UUID, ms account access token
/// all storage in ~/.yogurt/accounts.toml
fn save_account(
    username: &str,
    id: &str,
    uuid: &str,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path::get_path(Path::new("accounts.toml")))?; // need rewrite file location

    let mut config_contents = String::new();
    config_file.read_to_string(&mut config_contents)?;

    let mut config: Value = toml::from_str(&config_contents)?;

    let account = config
        .as_table_mut()
        .unwrap()
        .entry(username)
        .or_insert(Value::Table(Default::default()));

    account
        .as_table_mut()
        .unwrap()
        .insert("uuid".to_owned(), Value::String(uuid.to_owned()));
    account
        .as_table_mut()
        .unwrap()
        .insert("token".to_owned(), Value::String(token.to_owned()));

    let mut config_file = OpenOptions::new().write(true).open(path::get_path(Path::new("accounts.toml")))?; // need rewrite file location
    config_file.write_all(toml::to_string(&config).unwrap().as_bytes())?;

    Ok(())
}
