use std::{
    fs::{self, OpenOptions},
    io::Read,
};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::tools::path;

use super::api_accounts::{get_minecraft_token, update_access_token};

/// The response from Minecraft when attempting to retrieve a users profile
#[derive(Serialize, Deserialize, Debug)]
struct MinecraftProfileResponse {
    /// The UUID of the account
    id: String,
    /// The name of the user
    name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Serialize, Clone, Deserialize, Debug)]
#[serde(transparent)]
struct Accounts {
    users: Vec<User>,
}

const USERS_FILE: &str = "accounts.json";

/// get all user names from the accounts.json file
#[tauri::command]
pub fn get_all_users() -> Result<Vec<String>, String> {
    let users = load_users().map_err(|err| err.to_string())?;

    let mut accounts = Vec::new();
    for user in users.users {
        accounts.push(user.username);
    }
    Ok(accounts)
}

// delete some user by username
#[tauri::command]
pub fn delete_user(name: &str) -> Result<(), String> {
    let mut users = load_users().map_err(|err| err.to_string())?;

    let index = users
        .users
        .iter()
        .position(|u| u.username == name)
        .ok_or("User not found")?;
    users.users.remove(index);

    let users_json = serde_json::to_string(&users).map_err(|err| err.to_string())?;
    fs::write(path::get_path(USERS_FILE), users_json).map_err(|err| err.to_string())?;

    Ok(())
}

pub fn get_user(username: &str) -> Result<User, String> {
    let users = load_users().map_err(|err| err.to_string())?;

    let index = users
        .users
        .iter()
        .position(|u| &u.username == username)
        .ok_or("User not found")?;
    Ok(users.users[index].clone())
}

fn load_users() -> Result<Accounts, String> {
    let mut config_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path::get_path(USERS_FILE))
        .map_err(|err| err.to_string())?;

    let mut config_contents = String::new();
    config_file
        .read_to_string(&mut config_contents)
        .map_err(|err| err.to_string())?;

    if config_contents == "" {
        return Ok(Accounts { users: vec![] });
    };

    let users: Accounts = serde_json::from_str(&config_contents).map_err(|err| err.to_string())?;

    Ok(users)
}

fn update_user(user: &User) -> Result<(), String> {
    let mut users = load_users().map_err(|err| err.to_string())?;
    let index = users
        .users
        .iter()
        .position(|u| u.username == user.username)
        .ok_or("User not found")?;

    users.users[index] = user.clone();
    Ok(())
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

    /// save user to accounts.json in launcher default foldert
    pub fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if update_user(&*self).is_ok() {
            return Ok(());
        }

        let user = self.clone();
        let mut accounts = load_users()?;
        accounts.users.push(user);

        fs::write(
            path::get_path(USERS_FILE),
            serde_json::to_string(&accounts)?,
        )?;

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
    pub async fn verify_minecraft_token(&mut self) -> Result<User, String> {
        if self.minecraft_exp
            < SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("error while generate unix time stamp")
                .as_secs() as i64
        {
            self.verify_access_token().await?;
        }
        match self.get_info().await {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e)
            }
        }
        let user = get_user(&self.username)?;
        Ok(user)
    }

    async fn verify_access_token(&self) -> Result<(), String> {
        if self.access_exp
            > SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("error while generate unix time stamp")
                .as_secs() as i64
        {
            get_minecraft_token(
                self.access_token.clone(),
                self.access_exp as u64,
                self.refresh_token.clone(),
            )
            .await
            .map_err(|err| err.to_string())?;
        } else {
            update_access_token(&self.refresh_token)
                .await
                .map_err(|err| err.to_string())?;
        }
        Ok(())
    }
}
