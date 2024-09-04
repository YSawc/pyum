use super::m20240803_064001_create_device_table::Device;
use super::m20240903_212851_create_sensor_purpose::SensorPurpose;
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
                    .table(Sensor::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Sensor::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Sensor::DeviceId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("pk_sensor_device_id")
                            .from(Sensor::Table, Sensor::DeviceId)
                            .to(Device::Table, Device::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Sensor::SensorPurposeId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("pk_sensor_sensor_purpose_id")
                            .from(Sensor::Table, Sensor::SensorPurposeId)
                            .to(SensorPurpose::Table, SensorPurpose::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Sensor::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Sensor::TriggerLimitVal).float().not_null())
                    .col(
                        ColumnDef::new(Sensor::TriggerLimitSequenceCount)
                            .integer()
                            .not_null()
                            .default(Expr::val(1)),
                    )
                    .col(
                        ColumnDef::new(Sensor::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Sensor::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .extra("ON UPDATE CURRENT_TIMESTAMP"),
                    ),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table(Sensor::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Sensor {
    Table,
    Id,
    DeviceId,
    SensorPurposeId,
    TriggerLimitVal,
    TriggerLimitSequenceCount,
    CreatedAt,
    UpdatedAt,
}
