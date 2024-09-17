use sea_orm_migration::prelude::*;

use crate::m20240904_121407_create_sensor::Sensor;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Capture::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Capture::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Capture::SensorId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("pk_capture_sensor_id")
                            .from(Capture::Table, Capture::SensorId)
                            .to(Sensor::Table, Sensor::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Capture::CaptureVal).integer().not_null())
                    .col(ColumnDef::new(Capture::ShiftDigit).integer())
                    .col(
                        ColumnDef::new(Capture::CreatedAt)
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Capture::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Capture {
    Table,
    Id,
    SensorId,
    CaptureVal,
    ShiftDigit,
    CreatedAt,
}
