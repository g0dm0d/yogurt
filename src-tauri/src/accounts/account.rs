use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use toml::Value;

use crate::tools::path;

use super::api_accounts::{get_access_token, get_minecraft_token};

/// The response from Minecraft when attempting to retrieve a users profile
#[derive(Serialize, Deserialize, Debug)]
struct MinecraftProfileResponse {
    /// The UUID of the account
    id: String,
    /// The name of the user
    name: String,
}

#[derive(Clone, Debug, Deserialize)]
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
    pub access_exp: i64,
    /// minecraft bearer access token
    pub minecraft_token: String,
    /// minecraft bearer access token expiration date
    pub minecraft_exp: i64,
}

/// get all user names from the accounts.toml file
#[tauri::command]
pub fn get_all_users() -> Vec<String> {
    let path = path::get_path("accounts.toml");
    if !path.exists() {
        return Vec::new();
    }
    let config_file = fs::read_to_string(path).unwrap();
    let toml: Value = config_file.parse().unwrap();

    let mut accounts = Vec::new();
    for (key, _) in toml.as_table().unwrap().iter() {
        accounts.push(key.to_owned());
    }
    return accounts;
}

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(flatten)]
    users: std::collections::HashMap<String, User>,
}

#[tauri::command]
pub fn delete_account(name: String) {
    let mut accounts_toml: Value = fs::read_to_string(path::get_path("accounts.toml"))
        .unwrap()
        .parse()
        .unwrap();
    accounts_toml.as_table_mut().unwrap().remove(&name);
    fs::write(path::get_path("accounts.toml"), accounts_toml.to_string()).unwrap();
}

pub fn get_user(username: &str) -> User {
    let toml = fs::read_to_string(path::get_path("accounts.toml")).unwrap();
    let config: Config = toml::from_str(&toml).unwrap();
    let user_ref = config.users.get(username).unwrap();
    return user_ref.clone();
}

impl User {
    pub fn new(
        uuid: String,
        username: String,
        refresh_token: String,
        access_token: String,
        access_exp: i64,
        minecraft_token: String,
        minecraft_exp: i64,
    ) -> Self {
        User {
            uuid,
            username,
            refresh_token,
            access_token,
            access_exp,
            minecraft_token,
            minecraft_exp,
        }
    }

    /// save user to accounts.toml in launcher default foldert
    pub fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut config_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path::get_path("accounts.toml"))?;

        let mut config_contents = String::new();
        config_file.read_to_string(&mut config_contents)?;
        let mut accounts_toml: Value = toml::from_str(&config_contents)?;

        let account = accounts_toml
            .as_table_mut()
            .unwrap()
            .entry(&self.username)
            .or_insert(Value::Table(Default::default()));

        let account_table = account.as_table_mut().unwrap();
        account_table.insert("uuid".to_owned(), Value::String(self.uuid.to_owned()));
        account_table.insert(
            "refresh_token".to_owned(),
            Value::String(self.refresh_token.to_owned()),
        );
        account_table.insert(
            "username".to_owned(),
            Value::String(self.username.to_owned()),
        );
        account_table.insert(
            "access_token".to_owned(),
            Value::String(self.access_token.to_owned()),
        );
        account_table.insert("access_exp".to_owned(), Value::Integer(self.access_exp));
        account_table.insert(
            "minecraft_token".to_owned(),
            Value::String(self.minecraft_token.to_owned()),
        );
        account_table.insert(
            "minecraft_exp".to_owned(),
            Value::Integer(self.minecraft_exp),
        );

        config_file.write_all(toml::to_string(&accounts_toml).unwrap().as_bytes())?;

        Ok(())
    }

    /// get user uuid and username by minecraft bearer token
    /// https://wiki.vg/Microsoft_Authentication_Scheme#Getting_the_profile
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

    /// This function checks if the minecraft_token has expired, if it has, it checks if the access_token has expired, and if it has, it uses the refresh token before.
    pub async fn verify_minecraft_token(&mut self) -> User {
        if self.minecraft_exp
            < SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("error while generate unix time stamp")
                .as_secs() as i64
        {
            self.verify_access_token().await;
        }
        match self.get_info().await {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e)
            }
        }
        get_user(&self.username)
    }

    async fn verify_access_token(&self) {
        if self.access_exp
            < SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("error while generate unix time stamp")
                .as_secs() as i64
        {
            match get_minecraft_token(
                self.access_token.clone(),
                self.access_exp as u64,
                self.refresh_token.clone(),
            )
            .await
            {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e)
                }
            }
        } else {
            match get_access_token(&self.refresh_token).await {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e)
                }
            }
        }
    }
}
