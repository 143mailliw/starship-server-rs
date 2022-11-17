#![allow(non_snake_case)]
use super::super::user::Model;
use async_graphql::types::ID;
use chrono::NaiveDateTime;

#[async_graphql::Object]
impl Model {
    async fn id(&self) -> ID {
        ID(self.id.clone())
    }

    async fn username(&self) -> String {
        self.username.clone()
    }

    async fn admin(&self) -> bool {
        self.admin
    }

    async fn profilePicture(&self) -> Option<String> {
        self.profile_picture.clone()
    }

    async fn profileBanner(&self) -> Option<String> {
        self.profile_banner.clone()
    }

    async fn profileBio(&self) -> Option<String> {
        self.profile_bio.clone()
    }

    async fn banned(&self) -> bool {
        self.banned
    }

    // TODO: guard behind authentication
    async fn following(&self) -> Vec<String> {
        // TODO: return a vec of Planets instead
        self.following.clone()
    }

    // TODO: guard behind authentication
    // this function is obsolete
    #[graphql(deprecation = "memberOf is deprecated in favor of the role system, use following")]
    async fn memberOf(&self) -> Vec<String> {
        // TODO: return a vec of Planets instead
        vec![]
    }

    async fn createdAt(&self) -> NaiveDateTime {
        self.created
    }

    // TODO: guard behind authentication
    async fn usedBytes(&self) -> f64 {
        self.bytes_used as f64
    }

    // TODO: guard behind authentication
    async fn capWaived(&self) -> bool {
        self.cap_waived
    }

    // TODO: guard behind authentication
    async fn tfaEnabled(&self) -> bool {
        self.tfa_enabled
    }

    // TODO: guard behind authentication
    async fn blockedUsers(&self) -> Vec<String> {
        // TODO: return a vec of Users instead
        self.blocked.clone()
    }

    async fn online(&self) -> bool {
        self.sessions.len() > 0
    }

    // TODO: guard behind authentication
    async fn notificationSetting(&self) -> i16 {
        self.notification_setting
    }
}
