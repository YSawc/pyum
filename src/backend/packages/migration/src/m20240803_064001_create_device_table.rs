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
                    .table(Device::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Device::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Device::AdminUserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("pk_device_admin_user_id")
                            .from(Device::Table, Device::AdminUserId)
                            .to(AdminUser::Table, AdminUser::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Device::Name).string().not_null())
                    .col(ColumnDef::new(Device::Image).string())
                    .col(ColumnDef::new(Device::DeletedAt).date_time())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Device::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Device {
    Table,
    Id,
    AdminUserId,
    Name,
    Image,
    DeletedAt,
}
