use crate::errors;
use crate::sessions::Session;
use async_graphql::{async_trait, Context, Error, Guard};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum SessionType {
    User,
    Admin,
}

pub struct SessionGuard {
    session_type: SessionType,
}

impl SessionGuard {
    pub fn new(session_type: SessionType) -> Self {
        Self { session_type }
    }
}

#[async_trait::async_trait]
impl Guard for SessionGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<(), Error> {
        let session = ctx.data::<Session>();

        match session {
            Ok(session) => match session.user.clone() {
                Some(user) => {
                    if self.session_type == SessionType::Admin && !user.admin {
                        Err(errors::create_forbidden_error(None, "NOT_GLOBAL_ADMIN"))
                    } else {
                        Ok(())
                    }
                }
                None => Err(errors::create_forbidden_error(
                    Some("Not logged in."),
                    "NOT_LOGGED_IN",
                )),
            },
            Err(_error) => Err(errors::create_internal_server_error(
                None,
                "NO_SESSION_ERROR",
            )),
        }
    }
}