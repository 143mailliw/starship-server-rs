use crate::errors;
use crate::sessions::Session;
use async_graphql::{async_trait, Context, Error, Guard};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum SessionType {
    Token,
    User,
    NotBanned,
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
        let session = ctx
            .data::<Session>()
            .map_err(|_| errors::create_internal_server_error(None, "NO_SESSION_ERROR"))?;

        let user = session.user.as_ref().ok_or(errors::create_forbidden_error(
            Some("Not logged in."),
            "NOT_LOGGED_IN",
        ))?;

        if self.session_type == SessionType::Admin && !user.admin {
            Err(errors::create_forbidden_error(None, "NOT_GLOBAL_ADMIN"))
        } else if self.session_type == SessionType::NotBanned && user.banned {
            Err(errors::create_forbidden_error(None, "USER_BANNED"))
        } else if self.session_type != SessionType::Token && !session.verified {
            Err(errors::create_forbidden_error(None, "UNVERIFIED_TOKEN"))
        } else {
            Ok(())
        }
    }
}
