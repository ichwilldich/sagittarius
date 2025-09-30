use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(InvalidJwt::Table)
          .if_not_exists()
          .col(pk_uuid(InvalidJwt::Id))
          .col(string(InvalidJwt::Token))
          .col(date_time(InvalidJwt::Exp))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(InvalidJwt::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum InvalidJwt {
  Table,
  Id,
  Token,
  Exp,
}
