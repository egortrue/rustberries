use crate::domain::user::User;
use crate::errors::{Result, ServerError};

use super::SocialRepository;
use deadpool_postgres::{ManagerConfig, Pool, RecyclingMethod, Runtime};
use serde_json::Value;
use std::env;
use tokio_postgres::NoTls;

pub struct State {
    pool: Pool,
}

impl Default for State {
    fn default() -> Self {
        let mut config = deadpool_postgres::Config::new();
        config.dbname = Some(env::var("PG_DBNAME").expect("ENV variable not found: PG_DBNAME"));
        config.host = Some(env::var("PG_HOST").expect("ENV variable not found: PG_HOST"));
        config.user = Some(env::var("PG_USER").expect("ENV variable not found: PG_USER"));
        config.password = Some(env::var("PG_PASS").expect("ENV variable not found: PG_PASS"));
        config.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });

        let pool = config
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .expect("Couldn't create pool for database");

        Self { pool }
    }
}

#[axum::async_trait]
impl SocialRepository for State {
    async fn insert_user(&self, user: &User) -> Result<()> {
        let client = match self.pool.get().await {
            Ok(a) => a,
            Err(e) => return Err(ServerError::DatabaseError(e.to_string())),
        };

        static QUERY: &str = "
            INSERT INTO users
            (username,
            password_hash)
            VALUES ($1, $2);
        ";

        match client
            .query(QUERY, &[&user.username, &user.password_hash])
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(ServerError::DatabaseError(e.to_string())),
        }
    }

    async fn get_user(&self, user: &User) -> Result<()> {
        let client = match self.pool.get().await {
            Ok(a) => a,
            Err(_) => return Err(ServerError::DatabaseError(String::new())),
        };

        static QUERY: &str = "
            SELECT json_agg(result) FROM
            (SELECT * FROM users WHERE login = $1)
            result;
        ";

        match client.query_one(QUERY, &[&user.username]).await {
            Ok(row) => {
                let data: Option<Value> = row.get(0);
                println!("{}", data.unwrap());

                Ok(())
            }
            Err(e) => Err(ServerError::DatabaseError(e.to_string())),
        }
    }
}
