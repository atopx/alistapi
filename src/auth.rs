use super::Response;
use reqwest;
use serde_json::json;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize)]
struct AuthResponse {
    token: String,
}

pub async fn login(server: &str, username: &str, password: &str) -> Result<String, String> {
    let url = format!("{}/api/auth/login/hash", server);
    let resp: Response<AuthResponse> = reqwest::Client::new()
        .post(url)
        .json(&json!({
            "username": username,
            "password": sha256(password),
        }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    if resp.code != 200 {
        return Err(resp.message);
    }
    Ok(resp.data.unwrap().token)
}

pub fn sha256(value: &str) -> String {
    let value = Sha256::digest(format!("{}-https://github.com/alist-org/alist", value));
    format!("{:x}", value)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub base_path: String,
    pub role: i64,
    pub disabled: bool,
    pub permission: i64,
    pub sso_id: String,
    pub otp: bool,
}

pub async fn get_user_info(server: &str, token: &str) -> Result<UserInfo, String> {
    let url = format!("{}/api/me", server);
    let resp: Response<UserInfo> = reqwest::Client::new()
        .get(url)
        .header("Authorization", token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    if resp.code != 200 {
        return Err(resp.message);
    }
    Ok(resp.data.unwrap())
}
