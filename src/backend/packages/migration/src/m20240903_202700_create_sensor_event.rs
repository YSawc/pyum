use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SensorEvent::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SensorEvent::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SensorEvent::Description).string().not_null())
                    .col(ColumnDef::new(SensorEvent::Image).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SensorEvent::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SensorEvent {
    Table,
    Id,
    Description,
    Image,
}
