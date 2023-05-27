use crate::prelude::{Result, *};
mod sql;
use sql::*;

use sqlx::mysql::MySqlPool;

use sqlx::*;

#[rocket::async_trait]
impl DBConnection for MySqlPool {
    async fn init(&self) -> Result {
        query(CREATE_TABLE).execute(self).await?;
        Ok(())
    }
    async fn create_user(&self, ident: &str, hash: &str, is_admin: bool) -> Result {
        query(INSERT_USER)
            .bind(ident)
            .bind(hash)
            .bind(is_admin)
            .execute(self)
            .await?;
        Ok(())
    }
    async fn update_user(&self, user: &User) -> Result {
        #[cfg(feature = "ident-email")]
        let ident = &user.email;
        #[cfg(feature = "ident-username")]
        let ident = &user.username;
        query(UPDATE_USER)
            .bind(ident)
            .bind(&user.password)
            .bind(user.is_admin)
            .bind(user.id)
            .execute(self)
            .await?;

        Ok(())
    }
    async fn delete_user_by_id(&self, user_id: i32) -> Result {
        query(REMOVE_BY_ID).bind(user_id).execute(self).await?;
        Ok(())
    }
    async fn delete_user_by_ident(&self, ident: &str) -> Result {
        query(REMOVE_BY_IDENT).bind(ident).execute(self).await?;
        Ok(())
    }
    async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
        let user = query_as(SELECT_BY_ID).bind(user_id).fetch_one(self).await?;

        Ok(user)
    }
    async fn get_user_by_ident(&self, ident: &str) -> Result<User> {
        let user = query_as(SELECT_BY_IDENT)
            .bind(ident)
            .fetch_one(self)
            .await?;
        Ok(user)
    }
}
