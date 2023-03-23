//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: String,
    pub created: DateTime,
    #[sea_orm(unique)]
    pub username: String,
    pub password: String,
    pub reset_token: Option<String>,
    pub reset_expiry: Option<DateTime>,
    pub email_address: String,
    pub verified: bool,
    pub verification_token: Option<String>,
    pub blocked: Vec<String>,
    pub sessions: Vec<Uuid>,
    pub banned: bool,
    pub admin: bool,
    pub notification_setting: i16,
    pub cap_waived: bool,
    pub bytes_used: i64,
    pub profile_picture: Option<String>,
    pub profile_banner: Option<String>,
    pub profile_bio: Option<String>,
    pub tfa_secret: Option<String>,
    pub tfa_enabled: bool,
    pub tfa_backup: Vec<String>,
    pub token_geofenced: bool,
    pub token_expires: bool,
    pub token_ip_locked: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::custom_emoji::Entity")]
    CustomEmoji,
    #[sea_orm(has_many = "super::planet::Entity")]
    Planet,
    #[sea_orm(has_many = "super::planet_member::Entity")]
    PlanetMember,
    #[sea_orm(has_many = "super::token::Entity")]
    Token,
}

impl Related<super::custom_emoji::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CustomEmoji.def()
    }
}

impl Related<super::planet::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Planet.def()
    }
}

impl Related<super::planet_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlanetMember.def()
    }
}

impl Related<super::token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Token.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
