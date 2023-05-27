use crate::prelude::*;
mod sql;
use std::convert::{TryFrom, TryInto};
use tokio_postgres::Client;
#[rocket::async_trait]
impl DBConnection for Client {
    async fn init(&self) -> Result {
        self.execute(sql::CREATE_TABLE, &[]).await?;
        Ok(())
    }
    async fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<(), Error> {
        self.execute(sql::INSERT_USER, &[&email, &hash, &is_admin])
            .await?;
        Ok(())
    }
    #[cfg(feature = "ident-email")]
    async fn update_user(&self, user: &User) -> Result {
        self.execute(
            sql::UPDATE_USER,
            &[&user.email, &user.password, &user.is_admin],
        )
        .await?;
        Ok(())
    }
    #[cfg(feature = "ident-username")]
    async fn update_user(&self, user: &User) -> Result {
        self.execute(
            sql::UPDATE_USER,
            &[&user.username, &user.password, &user.is_admin],
        )
        .await?;
        Ok(())
    }
    async fn delete_user_by_id(&self, user_id: i32) -> Result {
        self.execute(sql::REMOVE_BY_ID, &[&user_id]).await?;
        Ok(())
    }
    async fn delete_user_by_ident(&self, ident: &str) -> Result {
        self.execute(sql::REMOVE_BY_IDENT, &[&ident]).await?;
        Ok(())
    }
    async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
        let user = self.query_one(sql::SELECT_BY_ID, &[&user_id]).await?;
        user.try_into()
    }

    async fn get_user_by_ident(&self, ident: &str) -> Result<User> {
        let user = self.query_one(sql::SELECT_BY_IDENT, &[&ident]).await?;
        user.try_into()
    }
}

impl TryFrom<tokio_postgres::Row> for User {
    type Error = Error;
    #[throws(Error)]
    #[cfg(feature = "ident-email")]
    fn try_from(row: tokio_postgres::Row) -> User {
        User {
            id: row.get(0),
            email: row.get(1),
            password: row.get(2),
            is_admin: row.get(3),
        }
    }
    #[throws(Error)]
    #[cfg(feature = "ident-username")]
    fn try_from(row: tokio_postgres::Row) -> User {
        User {
            id: row.get(0),
            username: row.get(1),
            password: row.get(2),
            is_admin: row.get(3),
        }
    }
}
