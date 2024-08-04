use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Oauth2ClientSecret::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Oauth2ClientSecret::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Oauth2ClientSecret::ClientId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Oauth2ClientSecret::ClientSecret)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Oauth2ClientSecret::IsDeleted)
                            .tiny_integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Oauth2ClientSecret::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Oauth2ClientSecret {
    Table,
    Id,
    ClientId,
    ClientSecret,
    IsDeleted,
}
