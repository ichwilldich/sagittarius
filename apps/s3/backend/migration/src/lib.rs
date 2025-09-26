pub use sea_orm_migration::prelude::*;

mod m20250926_172600_key;
mod m20250926_172626_invalid_jwt;
mod m20250926_173213_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
      Box::new(m20250926_172600_key::Migration),
      Box::new(m20250926_172626_invalid_jwt::Migration),
      Box::new(m20250926_173213_user::Migration),
    ]
  }
}
