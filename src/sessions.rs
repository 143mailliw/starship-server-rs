use crate::entities::prelude::Token;
use crate::entities::prelude::User;
use crate::entities::token;
use crate::entities::user;
use actix_web::http::header;
use actix_web::HttpRequest;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::env;
use std::net::SocketAddr;

pub struct Session {
    pub token: Option<token::Model>,
    pub user: Option<user::Model>,
    pub user_agent: Option<String>,
    pub ip_address: Option<SocketAddr>,
}

#[derive(Serialize, Deserialize)]
pub struct JWTLoginToken {
    pub token: String,
    pub user_id: String,
}

impl Session {
    pub async fn make_session_from_request(
        request: &HttpRequest,
        db: DatabaseConnection,
    ) -> Session {
        let headers = request.headers();

        let data = match headers.get(header::AUTHORIZATION) {
            Some(auth) => {
                let auth_string = match auth.to_str() {
                    Ok(value) => value,
                    Err(_error) => "",
                };

                if auth_string.starts_with("Bearer ") {
                    let secret = env::var("SECRET").unwrap();
                    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
                    let jwt_token_data: JWTLoginToken =
                        match auth_string.replace("Bearer ", "").verify_with_key(&key) {
                            Ok(token) => token,
                            Err(_error) => JWTLoginToken {
                                token: "".to_string(),
                                user_id: "".to_string(),
                            },
                        };

                    if jwt_token_data.token == *"" {
                        (None, None)
                    } else {
                        let data = Token::find_by_id(jwt_token_data.token)
                            .find_also_related(User)
                            .one(&db)
                            .await;

                        match data {
                            Ok(value) => match value {
                                Some(values) => match values.1 {
                                    Some(found_user) => (Some(values.0), Some(found_user)),
                                    None => (None, None),
                                },
                                None => (None, None),
                            },
                            Err(_error) => (None, None),
                        }
                    }
                } else {
                    (None, None)
                }
            }
            None => (None, None),
        };

        let user_agent = match headers.get(header::USER_AGENT) {
            Some(user_agent) => match user_agent.to_str() {
                Ok(value) => Some(value.to_string()),
                Err(_error) => None,
            },
            None => None,
        };

        Session {
            token: data.0,
            user: data.1,
            user_agent,
            ip_address: request.peer_addr(),
        }
    }
}
