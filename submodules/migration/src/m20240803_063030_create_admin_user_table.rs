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
                    .table(AdminUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AdminUser::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AdminUser::Name).string().not_null())
                    .col(
                        ColumnDef::new(AdminUser::EncryptedPassword)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AdminUser::CreatedAt)
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(AdminUser::UpdatedAt)
                            .date_time()
                            .default(Expr::current_timestamp())
                            .extra("ON UPDATE CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(AdminUser::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AdminUser {
    Table,
    Id,
    Name,
    EncryptedPassword,
    CreatedAt,
    UpdatedAt,
}
