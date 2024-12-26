use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

use warp::{reply, Rejection};
use crate::utils::UserUtil::User;
use bcrypt::{verify, BcryptError};

use crate::utils::UserUtil::get_user_by_user_name;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: User,
    exp: usize,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}


use warp::http::StatusCode;

pub async fn login(body: LoginRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let user = get_user_by_user_name(body.username).await;

    println!("{}", body.password);

    match user {
        Ok(Some(user)) => {
            match verify_password(&body.password, &user.password) {
                Ok(true) => {
                    let expiration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 3600;
                    let claims = Claims {
                        sub: user,
                        exp: expiration as usize,
                    };

                    let encoding_key = EncodingKey::from_secret("secret_key".as_ref());
                    let header = Header::default();

                    match encode(&header, &claims, &encoding_key) {
                        Ok(token) => Ok(warp::reply::with_status(warp::reply::json(&token), StatusCode::OK)),
                        Err(_) => Ok(warp::reply::with_status(
                            warp::reply::json(&"Error creating JWT"),
                            StatusCode::INTERNAL_SERVER_ERROR,
                        )),
                    }
                }
                Ok(false) => Ok(warp::reply::with_status(
                    warp::reply::json(&"Invalid credentials"),
                    StatusCode::BAD_REQUEST,
                )),
                Err(_) => Ok(warp::reply::with_status(
                    warp::reply::json(&"Error verifying password"),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )),
            }
        }
        Ok(None) => Ok(warp::reply::with_status(
            warp::reply::json(&"User not found"),
            StatusCode::BAD_REQUEST,
        )),
        Err(_) => Ok(warp::reply::with_status(
            warp::reply::json(&"Error querying database"),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

pub fn verify_password(plain_password: &str, hashed_password: &str) -> Result<bool, BcryptError> {
    verify(plain_password, hashed_password)
}