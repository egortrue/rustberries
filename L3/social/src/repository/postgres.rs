use crate::{
    domain::{post::Post, user::User},
    errors::{ErrorKind, Result},
    repository::SocialRepository,
};
use deadpool_postgres::{ManagerConfig, Pool, RecyclingMethod, Runtime};
use serde_json::Value;
use std::env;
use tokio_postgres::NoTls;
use uuid::Uuid;

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
    async fn add_user(&self, user: &User) -> Result<()> {
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(error) => return Err(ErrorKind::DbConnectionError(error)),
        };

        // Запрос
        static QUERY: &str = "
            INSERT INTO users (username, password_hash) VALUES ($1, $2);
        ";
        let result = client
            .query(QUERY, &[&user.username, &user.password_hash])
            .await;

        // Обработка ответа
        if let Err(error) = result {
            match error.as_db_error() {
                None => Err(ErrorKind::UndefinedError(error.to_string())),
                Some(error) => {
                    if error.code().code() == "23505" {
                        Err(ErrorKind::Conflict("Username is already taken".to_string()))
                    } else {
                        Err(ErrorKind::PostgresError(error.to_owned()))
                    }
                }
            }
        } else {
            Ok(())
        }
    }

    async fn get_user(&self, username: &String) -> Result<User> {
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(error) => return Err(ErrorKind::DbConnectionError(error)),
        };

        // Запрос
        static QUERY: &str = "
            SELECT json_agg(result) FROM
            (SELECT * FROM users WHERE username = $1)
            result;
        ";
        let result = client.query_one(QUERY, &[&username]).await;

        // Обработка ответа
        match result {
            Ok(row) => {
                let json: Option<Value> = row.get(0);
                let json = match json {
                    Some(json) => json,
                    None => return Err(ErrorKind::NotFound("User not found".into())),
                };

                let users: serde_json::Result<[User; 1]> = serde_json::from_value(json);
                let users = match users {
                    Ok(users) => users,
                    Err(error) => return Err(ErrorKind::SerializeError(error.to_string())),
                };

                Ok(users[0].clone())
            }
            Err(error) => match error.as_db_error() {
                Some(error) => Err(ErrorKind::PostgresError(error.to_owned())),
                None => Err(ErrorKind::UndefinedError(error.to_string())),
            },
        }
    }

    async fn add_post(&self, post: &Post) -> Result<()> {
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(error) => return Err(ErrorKind::DbConnectionError(error)),
        };

        // Запрос
        static QUERY: &str = "
            INSERT INTO posts (id, author, content, likes) VALUES ($1, $2, $3, $4);
        ";
        let result = client
            .query(
                QUERY,
                &[&post.id, &post.author, &post.content, &(post.likes as i32)],
            )
            .await;

        if let Err(error) = result {
            match error.as_db_error() {
                None => Err(ErrorKind::UndefinedError(error.to_string())),
                Some(error) => {
                    if error.code().code() == "23505" {
                        Err(ErrorKind::Conflict("Post already exists".to_string()))
                    } else {
                        Err(ErrorKind::PostgresError(error.to_owned()))
                    }
                }
            }
        } else {
            Ok(())
        }
    }

    async fn get_posts(&self) -> Result<Vec<Post>> {
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(error) => return Err(ErrorKind::DbConnectionError(error)),
        };

        // Запрос
        static QUERY: &str = "
            SELECT json_agg(result) FROM
            (SELECT * FROM posts)
            result;
        ";
        let result = client.query_one(QUERY, &[]).await;

        // Обработка ответа
        match result {
            Ok(data) => {
                let json: Option<Value> = data.get(0);
                let json = match json {
                    Some(json) => json,
                    None => return Err(ErrorKind::NotFound("Posts not found".into())),
                };

                let posts: serde_json::Result<Vec<Post>> = serde_json::from_value(json);
                let posts = match posts {
                    Ok(posts) => posts,
                    Err(error) => return Err(ErrorKind::SerializeError(error.to_string())),
                };

                Ok(posts)
            }
            Err(error) => match error.as_db_error() {
                Some(error) => Err(ErrorKind::PostgresError(error.to_owned())),
                None => Err(ErrorKind::UndefinedError(error.to_string())),
            },
        }
    }

    async fn get_post(&self, id: &Uuid) -> Result<Post> {
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(error) => return Err(ErrorKind::DbConnectionError(error)),
        };

        // Запрос
        static QUERY: &str = "
            SELECT json_agg(result) FROM
            (SELECT * FROM posts WHERE id = $1)
            result;
        ";
        let result = client.query_one(QUERY, &[id]).await;

        // Обработка ответа
        match result {
            Ok(data) => {
                let json: Option<Value> = data.get(0);
                let json = match json {
                    Some(json) => json,
                    None => return Err(ErrorKind::NotFound("Post not found".into())),
                };

                let posts: serde_json::Result<[Post; 1]> = serde_json::from_value(json);
                let posts = match posts {
                    Ok(posts) => posts,
                    Err(error) => return Err(ErrorKind::SerializeError(error.to_string())),
                };

                Ok(posts[0].clone())
            }
            Err(error) => match error.as_db_error() {
                Some(error) => Err(ErrorKind::PostgresError(error.to_owned())),
                None => Err(ErrorKind::UndefinedError(error.to_string())),
            },
        }
    }

    async fn delete_post(&self, id: &Uuid) -> Result<()> {
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(error) => return Err(ErrorKind::DbConnectionError(error)),
        };

        // Запрос
        static QUERY: &str = "
            DELETE FROM posts WHERE id = $1;
        ";
        let result = client.query(QUERY, &[id]).await;

        // Обработка ответа
        if let Err(error) = result {
            match error.as_db_error() {
                None => Err(ErrorKind::UndefinedError(error.to_string())),
                Some(error) => Err(ErrorKind::PostgresError(error.to_owned())),
            }
        } else {
            Ok(())
        }
    }

    async fn like_post(&self, id: &Uuid) -> Result<()> {
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(error) => return Err(ErrorKind::DbConnectionError(error)),
        };

        // Запрос
        static QUERY: &str = "
            UPDATE posts SET likes = likes + 1 WHERE id = $1;
        ";
        let result = client.query(QUERY, &[id]).await;

        // Обработка ответа
        if let Err(error) = result {
            match error.as_db_error() {
                None => Err(ErrorKind::UndefinedError(error.to_string())),
                Some(error) => Err(ErrorKind::PostgresError(error.to_owned())),
            }
        } else {
            Ok(())
        }
    }

    async fn add_likes(&self, post: &Uuid, username: &String) -> Result<()> {
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(error) => return Err(ErrorKind::DbConnectionError(error)),
        };

        // Запрос
        static QUERY: &str = "
            INSERT INTO likes (post, username) VALUES ($1, $2);
        ";
        let result = client.query(QUERY, &[post, username]).await;

        // Обработка ответа
        if let Err(error) = result {
            match error.as_db_error() {
                None => Err(ErrorKind::UndefinedError(error.to_string())),
                Some(error) => Err(ErrorKind::PostgresError(error.to_owned())),
            }
        } else {
            Ok(())
        }
    }

    async fn get_likes(&self, post: &Uuid) -> Result<Vec<String>> {
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(error) => return Err(ErrorKind::DbConnectionError(error)),
        };

        // Запрос
        static QUERY: &str = "
            SELECT username FROM likes WHERE post = $1;
        ";
        let result = client.query(QUERY, &[post]).await;

        // Обработка ответа
        match result {
            Ok(data) => {
                let mut liked: Vec<String> = vec![];
                for like in data {
                    let json: Option<String> = like.get(0);
                    match json {
                        Some(username) => liked.push(username),
                        None => return Ok(vec![]),
                    };
                }
                Ok(liked)
            }
            Err(error) => match error.as_db_error() {
                Some(error) => Err(ErrorKind::PostgresError(error.to_owned())),
                None => Err(ErrorKind::UndefinedError(error.to_string())),
            },
        }
    }
}
