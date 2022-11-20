use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).text().not_null().primary_key())
                    .col(ColumnDef::new(User::Created).timestamp().not_null())
                    .col(
                        ColumnDef::new(User::Username)
                            .string_len(128)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::ResetToken).string())
                    .col(ColumnDef::new(User::ResetExpiry).timestamp())
                    .col(ColumnDef::new(User::EmailAddress).string().not_null())
                    .col(
                        ColumnDef::new(User::Verified)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(User::VerificationToken).string())
                    .col(
                        ColumnDef::new(User::Following)
                            .array(ColumnType::Text)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::Blocked)
                            .array(ColumnType::Text)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::Sessions)
                            .array(ColumnType::Uuid)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::Banned)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(User::Admin)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(User::NotificationSetting)
                            .tiny_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(User::CapWaived)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(User::BytesUsed)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(User::ProfilePicture).string())
                    .col(ColumnDef::new(User::ProfileBanner).string())
                    .col(ColumnDef::new(User::ProfileBio).string_len(4000))
                    .col(ColumnDef::new(User::TfaSecret).string())
                    .col(
                        ColumnDef::new(User::TfaEnabled)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(User::TfaBackup)
                            .array(ColumnType::Text)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::TokenGeofenced)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(User::TokenExpires)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(User::TokenIpLocked)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Created,
    Username,

    Password,
    ResetToken,
    ResetExpiry,

    EmailAddress,
    Verified,
    VerificationToken,

    Following,
    Blocked,
    Sessions,

    Banned,
    Admin,
    NotificationSetting,

    CapWaived,
    BytesUsed,

    ProfilePicture,
    ProfileBanner,
    ProfileBio,

    TfaSecret,
    TfaEnabled,
    TfaBackup,

    TokenGeofenced,
    TokenExpires,
    TokenIpLocked,
}
