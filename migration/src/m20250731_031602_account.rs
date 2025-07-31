use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(pk_auto(Account::Id))
                    .col(uuid_uniq(Account::Uid))
                    .col(string(Account::Phone))
                    .col(string(Account::Password))
                    .col(tiny_unsigned(Account::IsActive).default(0))
                    .col(date_time_null(Account::LastLoginAt))
                    .col(date_time(Account::CreatedAt).default(Expr::current_timestamp()))
                    .col(date_time(Account::UpdatedAt).default(Expr::current_timestamp()))
                    .col(date_time_null(Account::DeletedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Account {
    Table,
    Id,
    Uid,
    Phone,
    Password,
    IsActive,
    LastLoginAt,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
