use entity::user;
use sea_orm::prelude::*;

pub struct UserTable<'db> {
  db: &'db DatabaseConnection,
}

impl<'db> UserTable<'db> {
  pub fn new(db: &'db DatabaseConnection) -> Self {
    Self { db }
  }

  pub async fn get_user(&self, id: Uuid) -> Result<entity::user::Model, sea_orm::DbErr> {
    let res = entity::user::Entity::find_by_id(id).one(self.db).await?;

    res.ok_or(sea_orm::DbErr::RecordNotFound("Not Found".into()))
  }

  pub async fn get_user_by_name(
    &self,
    name: String,
  ) -> Result<entity::user::Model, sea_orm::DbErr> {
    let res = entity::user::Entity::find()
      .filter(user::Column::Name.eq(name))
      .one(self.db)
      .await?;

    res.ok_or(sea_orm::DbErr::RecordNotFound("Not Found".into()))
  }

  pub async fn create_user(&self, user: user::Model) -> Result<(), sea_orm::DbErr> {
    let user: user::ActiveModel = user.into();

    user.insert(self.db).await?;

    Ok(())
  }

  pub async fn delete_user(&self, id: Uuid) -> Result<(), sea_orm::DbErr> {
    entity::user::Entity::delete_by_id(id).exec(self.db).await?;

    Ok(())
  }

  pub async fn list_users(&self) -> Result<Vec<user::Model>, sea_orm::DbErr> {
    let res = entity::user::Entity::find().all(self.db).await?;

    Ok(res)
  }
}
