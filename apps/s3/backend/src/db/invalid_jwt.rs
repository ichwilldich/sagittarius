use chrono::{DateTime, Utc};
use entity::{invalid_jwt, prelude::*};
use sea_orm::{ActiveValue::Set, prelude::*};

pub struct InvalidJwtTable<'db> {
  db: &'db DatabaseConnection,
}

impl<'db> InvalidJwtTable<'db> {
  pub fn new(db: &'db DatabaseConnection) -> Self {
    Self { db }
  }

  pub async fn invalidate_jwt(
    &self,
    token: String,
    exp: DateTime<Utc>,
    invalid_count: &mut i32,
  ) -> Result<(), DbErr> {
    let model = invalid_jwt::ActiveModel {
      token: Set(token),
      exp: Set(exp.naive_utc()),
      id: Set(Uuid::new_v4()),
    };
    model.insert(self.db).await?;

    if *invalid_count > 1000 {
      self.remove_expired().await?;
      *invalid_count = 0;
    } else {
      *invalid_count += 1;
    }

    Ok(())
  }

  pub async fn is_token_valid(&self, token: &str) -> Result<bool, DbErr> {
    let res = InvalidJwt::find()
      .filter(invalid_jwt::Column::Token.eq(token))
      .one(self.db)
      .await?;

    Ok(res.is_none())
  }

  pub async fn remove_expired(&self) -> Result<(), DbErr> {
    InvalidJwt::delete_many()
      .filter(invalid_jwt::Column::Exp.lt(Utc::now().naive_utc()))
      .exec(self.db)
      .await?;

    Ok(())
  }
}
