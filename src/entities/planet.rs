//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "planet")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: String,
    pub name: String,
    pub created: DateTime,
    pub owner: String,
    pub private: bool,
    pub follower_count: i32,
    pub featured: bool,
    pub verified: bool,
    pub partnered: bool,
    pub featured_description: String,
    pub banned: Vec<String>,
    pub css: String,
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Owner",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
