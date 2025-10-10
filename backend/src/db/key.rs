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

#[cfg(test)]
mod test {
  use super::*;
  use crate::db::test::test_db;

  #[tokio::test]
  async fn test_key_table() {
    let db = test_db().await;
    let key_table = db.key();

    let key_name = "test_key".to_string();
    let key_value = "private_key_value".to_string();
    let key_id = Uuid::new_v4();

    // Test create_key
    key_table
      .create_key(key_name.clone(), key_value.clone(), key_id)
      .await
      .expect("Failed to create key");

    // Test get_key_by_name
    let fetched_key = key_table
      .get_key_by_name(key_name.clone())
      .await
      .expect("Failed to fetch key by name");

    assert_eq!(fetched_key.name, key_name);
    assert_eq!(fetched_key.private_key, key_value);
    assert_eq!(fetched_key.id, key_id);
  }
}
