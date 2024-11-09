use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Items::Table)
                    .if_not_exists()
                    .col(pk_auto(Items::Id))
                    .col(string(Items::Title))
                    .col(string(Items::Description))
                    .col(integer(Items::Price))
                    .to_owned(),
            )
            .await?;
        manager.create_table(
            Table::create()
                .table(Users::Table)
                .if_not_exists()
                .col(pk_uuid(Users::Id))
                .col(string_uniq(Users::Email))
                .col(string(Users::Password))
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Items::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Items {
    Table,
    Id,
    Title,
    Description,
    Price
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Email,
    Password
}
