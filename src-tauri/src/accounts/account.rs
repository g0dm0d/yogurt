use std::{fs::{OpenOptions, self}, path::Path, io::{Read, Write}};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use toml::Value;

use crate::tools::path;

/// The response from Minecraft when attempting to retrieve a users profile
#[derive(Serialize, Deserialize, Debug)]
struct MinecraftProfileResponse {
    /// The UUID of the account
    id: String,
    /// The name of the user
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct User {
    /// uuid playr
    pub uuid: String,
    /// nickname player
    pub username: String,
    /// ms account refresh token
    pub refresh_token: String,
    /// ms account access token
    pub access_token: String,
    /// ms access token expiration date
    pub access_exp: String,
    /// minecraft bearer access token
    pub minecraft_token: String,
    /// minecraft bearer access token expiration date
    pub minecraft_exp: String
}

pub async fn get_all_users() -> Vec<String> {
    let config_file = fs::read_to_string(crate::tools::path::get_path(Path::new("accounts.toml"))).unwrap();
    let toml: Value = config_file.parse().unwrap();

    let mut accounts = Vec::new();
    for (key, _) in toml.as_table().unwrap().iter() {
        accounts.push(key.to_owned());
    }
    return accounts;
}

impl User {
    pub fn new(uuid: String, username: String, refresh_token: String, access_token: String, access_exp: String, minecraft_token: String, minecraft_exp: String) -> Self {
        User { uuid, username, refresh_token, access_token, access_exp, minecraft_token, minecraft_exp}
    }
    pub fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut config_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path::get_path(Path::new("accounts.toml")))?;

        let mut config_contents = String::new();
        config_file.read_to_string(&mut config_contents)?;

        let mut config: Value = toml::from_str(&config_contents)?;

        let account = config
            .as_table_mut()
            .unwrap()
            .entry(&self.username)
            .or_insert(Value::Table(Default::default()));

        let account_table = account.as_table_mut().unwrap();
        account_table
            .insert("uuid".to_owned(), Value::String(self.uuid.to_owned()));
        account_table
            .insert("refresh_token".to_owned(), Value::String(self.refresh_token.to_owned()));
        account_table
            .insert("access_token".to_owned(), Value::String(self.access_token.to_owned()));
        account_table
            .insert("access_exp".to_owned(), Value::String(self.access_exp.to_owned()));
        account_table
            .insert("minecraft_token".to_owned(), Value::String(self.minecraft_token.to_owned()));
        account_table
            .insert("minecraft_exp".to_owned(), Value::String(self.minecraft_exp.to_owned()));

        let mut config_file = OpenOptions::new().write(true).open(path::get_path(Path::new("accounts.toml")))?;
        config_file.write_all(toml::to_string(&config).unwrap().as_bytes())?;

        Ok(())
    }

    pub async fn get_info(&mut self) -> Result<&mut User, Box<dyn std::error::Error>> {
        let client = Client::new();
        let minecraft_profile_resp: MinecraftProfileResponse = client
            .get("https://api.minecraftservices.com/minecraft/profile")
            .bearer_auth(&self.minecraft_token)
            .send()
            .await?
            .json()
            .await?;
            serde_json::to_string(&minecraft_profile_resp)?;
        self.uuid = minecraft_profile_resp.id;
        self.username = minecraft_profile_resp.name;
        Ok(self)
    }
}
