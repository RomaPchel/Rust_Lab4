use actix_web::{web, HttpResponse, Responder};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde_json::json;
use warp::http::StatusCode;
use warp::{reply, Rejection};
use warp::reply::with_status;
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


pub async fn login(body: LoginRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let user = get_user_by_user_name(body.username).await;

    println!("{}", body.password);

    match user {
        Ok(Some(user)) => {
            // Verify the password using bcrypt (hashing comparison)
            match verify_password(&body.password, &user.password) {
                Ok(true) => {
                    // Password is correct, create JWT token
                    let expiration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 3600;
                    let claims = Claims {
                        sub: user,
                        exp: expiration as usize,
                    };

                    let encoding_key = EncodingKey::from_secret("secret_key".as_ref());
                    let header = Header::default();

                    match encode(&header, &claims, &encoding_key) {
                        Ok(token) => {
                            Ok(reply::json(&token))  // Return the token as a JSON response
                        }
                        Err(_) => {
                            let error_message = "Error creating JWT";
                            Ok(reply::json(&error_message))
                        }
                    }
                }
                Ok(false) => {
                    let error_message = "Invalid credentials";
                    Ok(reply::json(&error_message))
                }
                Err(_) => {
                    let error_message = "Error verifying password";
                    Ok(reply::json(&error_message))
                }
            }
        }
        Ok(None) => {
            let error_message = "User not found";
            Ok(reply::json(&error_message))
        }
        Err(_) => {
            let error_message = "Error querying database";
            Ok(reply::json(&error_message))
        }
    }
}
pub fn verify_password(plain_password: &str, hashed_password: &str) -> Result<bool, BcryptError> {
    verify(plain_password, hashed_password)  // Use bcrypt to check if the password matches the hash
}