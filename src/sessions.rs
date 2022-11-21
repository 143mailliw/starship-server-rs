use crate::entities::prelude::Token;
use crate::entities::prelude::User;
use crate::entities::token;
use crate::entities::user;
use actix_web::http::header;
use actix_web::HttpRequest;
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};
use std::net::SocketAddr;

pub struct Session {
    pub token: Option<token::Model>,
    pub user: Option<user::Model>,
    pub user_agent: Option<String>,
    pub ip_address: Option<SocketAddr>,
}

impl Session {
    pub async fn make_session_from_request(
        request: &HttpRequest,
        db: DatabaseConnection,
    ) -> Session {
        let headers = request.headers();

        // TODO: TFA Support
        let token = match headers.get(header::AUTHORIZATION) {
            Some(auth) => {
                let auth_string = match auth.to_str() {
                    Ok(value) => value,
                    Err(_error) => "",
                };

                if auth_string.starts_with("Bearer ") {
                    let token = Token::find_by_id(auth_string.replace("Bearer ", ""))
                        .one(&db)
                        .await;

                    match token {
                        Ok(value) => value,
                        Err(_error) => None,
                    }
                } else {
                    None
                }
            }
            None => None,
        };

        let user = match token.clone() {
            Some(token) => match token.find_related(User).one(&db).await {
                Ok(value) => value,
                Err(_error) => None,
            },
            None => None,
        };

        let user_agent = match headers.get(header::USER_AGENT) {
            Some(user_agent) => match user_agent.to_str() {
                Ok(value) => Some(value.to_string()),
                Err(_error) => None,
            },
            None => None,
        };

        Session {
            token,
            user,
            user_agent,
            ip_address: request.peer_addr(),
        }
    }
}
