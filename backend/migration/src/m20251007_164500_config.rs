use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Config::Table)
          .if_not_exists()
          .col(pk_uuid(Config::Id))
          .col(json_binary(Config::Config))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Config::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum Config {
  Table,
  Id,
  #[allow(clippy::enum_variant_names)]
  Config,
}
