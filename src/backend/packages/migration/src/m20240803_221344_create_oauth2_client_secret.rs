use crate::m20240803_063030_create_admin_user_table::AdminUser;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Oauth2ClientSecret::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Oauth2ClientSecret::ClientId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Oauth2ClientSecret::AdminUserId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("pk_oauth2_client_secret_admin_user_id")
                            .from(Oauth2ClientSecret::Table, Oauth2ClientSecret::AdminUserId)
                            .to(AdminUser::Table, AdminUser::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Oauth2ClientSecret::ClientSecret)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Oauth2ClientSecret::DeletedAt).date_time())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Oauth2ClientSecret::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Oauth2ClientSecret {
    Table,
    ClientId,
    AdminUserId,
    ClientSecret,
    DeletedAt,
}
