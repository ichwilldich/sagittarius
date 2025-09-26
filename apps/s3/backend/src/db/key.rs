use entity::{key, prelude::*};
use sea_orm::{ActiveValue::Set, prelude::*};

pub struct KeyTable<'db> {
  db: &'db DatabaseConnection,
}

impl<'db> KeyTable<'db> {
  pub fn new(db: &'db DatabaseConnection) -> Self {
    Self { db }
  }

  pub async fn get_key_by_name(&self, name: String) -> Result<key::Model, DbErr> {
    let res = Key::find()
      .filter(key::Column::Name.eq(name))
      .one(self.db)
      .await?;

    res.ok_or(DbErr::RecordNotFound("Not Found".into()))
  }

  pub async fn create_key(&self, name: String, key: String, id: Uuid) -> Result<(), DbErr> {
    let model = key::ActiveModel {
      name: Set(name),
      private_key: Set(key),
      id: Set(id),
    };

    model.insert(self.db).await?;

    Ok(())
  }
}
