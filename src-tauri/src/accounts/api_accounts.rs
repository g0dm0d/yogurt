// Thank you https://gist.github.com/OverHash/a71b32846612ba09d8f79c9d775bfadf

use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::accounts::account::User;

// id_token: String, LEGACY???

/// The response from authenticating with Microsoft OAuth flow
#[derive(Deserialize, Serialize, Debug)]
struct AuthorizationTokenResponse {
    /// The type of token for authentication
    token_type: String,
    /// The scope we have access to
    scope: String,
    /// Seconds until the authentication token expires
    expires_in: u64,
    /// Seconds until the authentication token expires
    ext_expires_in: u32,
    /// The authentication token itself
    access_token: String,
    /// The token used for refreshing access
    refresh_token: String,
}

/// The response from Xbox when authenticating with a Microsoft token
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct XboxLiveAuthenticationResponse {
    /// An ISO-8601 timestamp of when the token was issued
    issue_instant: String,
    /// An ISO-8601 timestamp of when the token expires
    not_after: String,
    /// The xbox authentication token to use
    token: String,
    /// An object that contains a vec of `uhs` objects
    /// Looks like { "xui": [{"uhs": "xbl_token"}] }
    display_claims: HashMap<String, Vec<HashMap<String, String>>>,
}

/// The response from Minecraft when attempting to authenticate with an xbox token
#[derive(Deserialize, Serialize, Debug)]
struct MinecraftAuthenticationResponse {
    /// Some UUID of the account
    username: String,
    /// The minecraft JWT access token
    access_token: String,
    /// The type of access token
    token_type: String,
    /// How many seconds until the token expires
    expires_in: u64,
}

const CLIENT_ID: &str = "d8e1d9bf-287f-4773-a176-e012722257f4";

/// Function for refresh tokens by refresh token
/// Documentation: https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow#refresh-the-access-token
pub async fn update_access_token(code: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let authorization_token: AuthorizationTokenResponse = client
        .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
        .form(&vec![
            ("client_id", CLIENT_ID),
            ("refresh_token", code),
            ("grant_type", "refresh_token"),
        ])
        .send()
        .await?
        .json()
        .await?;

    get_minecraft_token(
        authorization_token.access_token,
        authorization_token.expires_in,
        authorization_token.refresh_token,
    )
    .await
}

/// get minecraft bearer/access token
/// https://wiki.vg/Microsoft_Authentication_Scheme
pub async fn get_access_token(code: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // step 2: convert authorization code into authorization token
    let authorization_token: AuthorizationTokenResponse = client
        .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
        .form(&vec![
            ("client_id", CLIENT_ID),
            ("code", code),
            ("grant_type", "authorization_code"),
            ("redirect_uri", "http://localhost:9397"),
        ])
        .send()
        .await?
        .json()
        .await?;
    get_minecraft_token(
        authorization_token.access_token,
        authorization_token.expires_in,
        authorization_token.refresh_token,
    )
    .await
}
pub async fn get_minecraft_token(
    access_token: String,
    access_exp: u64,
    refresh_token: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    // step 3: authenticate with xbox live
    let xbox_authenticate_json = json!({
        "Properties": {
            "AuthMethod": "RPS",
            "SiteName": "user.auth.xboxlive.com",
            "RpsTicket": &format!("d={}", access_token)
        },
        "RelyingParty": "http://auth.xboxlive.com",
        "TokenType": "JWT"
    });

    let xbox_resp: XboxLiveAuthenticationResponse = client
        .post("https://user.auth.xboxlive.com/user/authenticate")
        .json(&xbox_authenticate_json)
        .send()
        .await?
        .json()
        .await?;

    let xbox_token = &xbox_resp.token;
    let user_hash = &xbox_resp.display_claims["xui"][0]["uhs"];

    // step 4: convert xbox token into xbox security token
    let xbox_security_token_resp: XboxLiveAuthenticationResponse = client
        .post("https://xsts.auth.xboxlive.com/xsts/authorize")
        .json(&json!({
            "Properties": {
                "SandboxId": "RETAIL",
                "UserTokens": [xbox_token]
            },
            "RelyingParty": "rp://api.minecraftservices.com/",
            "TokenType": "JWT"
        }))
        .send()
        .await?
        .json()
        .await?;

    let xbox_security_token = &xbox_security_token_resp.token;

    // step 5: authenticate with minecraft
    let minecraft_resp: MinecraftAuthenticationResponse = client
        .post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .json(&json!({
            "identityToken":
                format!(
                    "XBL3.0 x={user_hash};{xsts_token}",
                    user_hash = user_hash,
                    xsts_token = xbox_security_token
                )
        }))
        .send()
        .await?
        .json()
        .await?;

    let mut user = User::new(
        String::new(),
        String::new(),
        refresh_token,
        access_token,
        // Holly crap, it looks disgusting, yeah I know. But according to the documentation as I understand it comes back as long as the token lives.
        // So in order to use this information I add the lifetime to the time now in the unix time stamp
        (access_exp
            + SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("error while generate unix time stamp")
                .as_secs()) as i64,
        minecraft_resp.access_token,
        (minecraft_resp.expires_in
            + SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("error while generate unix time stamp")
                .as_secs()) as i64,
    );

    user.get_info().await?;

    match user.save() {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
