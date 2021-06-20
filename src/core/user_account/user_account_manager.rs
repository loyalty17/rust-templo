use super::user_account_data::UserAccountData;
use crate::core::{
    file_system::{paths::USER_ACCOUNT_AUTH_PATH, ProtternFileSystem},
    requester::{Method, ProtternRequester},
};
use serde_derive::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterResponse {
    pub registered: bool,
    pub user: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    pub authenticated: bool,
    pub user: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthRequestBody {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAccountKey {
    pub username: String,
    pub email: String,
    pub password: String,
    pub key: String,
}

pub struct UserAccountManager {}

impl UserAccountManager {
    pub fn save_user_account(user_account: UserAccountKey) -> Result<(), Error> {
        let content = serde_json::to_string(&user_account)?;
        ProtternFileSystem::write_base64_file(USER_ACCOUNT_AUTH_PATH, content)
    }

    pub fn get_user_account_data() -> Result<UserAccountKey, Error> {
        let user_account = ProtternFileSystem::read_base64_file(USER_ACCOUNT_AUTH_PATH)?;
        match serde_json::from_str(&user_account) {
            Err(e) => {
                let err = Error::new(ErrorKind::Other, e.to_string());
                Err(err)
            }
            Ok(u) => Ok(u),
        }
    }

    pub async fn register_user_account(
        user_account: &UserAccountData,
    ) -> Result<RegisterResponse, Error> {
        let body = serde_json::to_string(user_account)?;
        let req = ProtternRequester::build_request("/user/register", Method::POST, body);
        let response = ProtternRequester::request(req).await?;
        Ok(serde_json::from_str(&response).unwrap())
    }

    pub async fn authenticate_user_account(
        username: String,
        password: String,
    ) -> Result<AuthResponse, Error> {
        let body = serde_json::to_string(&AuthRequestBody { username, password }).unwrap();
        let req = ProtternRequester::build_request("/user/login", Method::POST, body);
        let response = ProtternRequester::request(req).await?;

        Ok(serde_json::from_str(&response)?)
    }
}
