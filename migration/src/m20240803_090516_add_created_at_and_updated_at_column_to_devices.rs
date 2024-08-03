use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .alter_table(
                Table::alter()
                    .table(Device::Table)
                    .add_column(
                        ColumnDef::new(Device::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .add_column(
                        ColumnDef::new(Device::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .extra("ON UPDATE CURRENT_TIMESTAMP")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .alter_table(
                Table::alter()
                    .table(Device::Table)
                    .drop_column(Alias::new("created_at"))
                    .drop_column(Alias::new("updated_at"))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Device {
    Table,
    CreatedAt,
    UpdatedAt,
}
