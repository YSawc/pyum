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
                    .table(SensorPurpose::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SensorPurpose::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SensorPurpose::AdminUserId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("pk_sensor_purpose_admin_user_id")
                            .from(SensorPurpose::Table, SensorPurpose::AdminUserId)
                            .to(AdminUser::Table, AdminUser::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(SensorPurpose::Description)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SensorPurpose::CreatedAt)
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(SensorPurpose::UpdatedAt)
                            .date_time()
                            .default(Expr::current_timestamp())
                            .extra("ON UPDATE CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SensorPurpose::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SensorPurpose {
    Table,
    Id,
    AdminUserId,
    Description,
    CreatedAt,
    UpdatedAt,
}
