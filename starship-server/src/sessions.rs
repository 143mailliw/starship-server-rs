use crate::entities::prelude::Token;
use crate::entities::prelude::User;
use crate::entities::token;
use crate::entities::user;
use actix_web::http::header;
use actix_web::HttpRequest;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::env;
use std::net::SocketAddr;

pub struct Session {
    pub token: Option<token::Model>,
    pub user: Option<user::Model>,
    pub user_agent: Option<String>,
    pub ip_address: Option<SocketAddr>,
    pub verified: bool,
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

        let data = if let Some(auth) = headers.get(header::AUTHORIZATION) {
            let auth_string = auth.to_str().unwrap_or("");

            if auth_string.starts_with("Bearer ") {
                let secret = env::var("SECRET").unwrap();
                let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
                let jwt_token_data: JWTLoginToken = auth_string
                    .replace("Bearer ", "")
                    .verify_with_key(&key)
                    .unwrap_or(JWTLoginToken {
                        token: String::new(),
                        user_id: String::new(),
                    });

                if jwt_token_data.token.is_empty() {
                    (None, None, false)
                } else {
                    let data = Token::find_by_id(jwt_token_data.token)
                        .find_also_related(User)
                        .one(&db)
                        .await
                        .ok()
                        .flatten();

                    if let Some(data) = data {
                        (
                            data.1.as_ref().map(|_| data.0.clone()),
                            data.1,
                            data.0.verified,
                        )
                    } else {
                        (None, None, false)
                    }
                }
            } else {
                (None, None, false)
            }
        } else {
            (None, None, false)
        };

        let user_agent = headers
            .get(header::USER_AGENT)
            .and_then(|v| v.to_str().map(std::string::ToString::to_string).ok());

        Session {
            token: data.0,
            user: data.1,
            verified: data.2,
            user_agent,
            ip_address: request.peer_addr(),
        }
    }
}
