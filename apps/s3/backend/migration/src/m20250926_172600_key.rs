use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Key::Table)
          .if_not_exists()
          .col(pk_uuid(Key::Id))
          .col(string(Key::Name))
          .col(string(Key::PrivateKey))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Key::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum Key {
  Table,
  Id,
  Name,
  #[allow(clippy::enum_variant_names)]
  PrivateKey,
}
