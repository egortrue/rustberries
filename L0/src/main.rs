/* Автор: Егор Трухин @egortrue

КОМАНДЫ:
- проверка: cargo clippy -- -W clippy::pedantic
- запуск: cargo run
- запуск с PG: cargo run -- --pg-enable --pg-user myuser --pg-pass mypassword --pg-host localhost --pg-port 5432 --pg-name postgres
- hotreload: cargo watch -c -d 0 -x run
- hotreload с PG: cargo watch -c -d 0 -x "run -- --pg-enable --pg-user myuser --pg-pass mypassword --pg-host localhost --pg-port 5432 --pg-name postgres"

КОММЕНТАРИИ:
- Все решение в одном файле - здесь
- Для поддержания чистоты кода, учитывая его кол-во, разбил все на модули в этом же файле
- Постарался придерживаться чистой архитектруры

ТЕМЫ:
[X] Axum (без middleware), разделяемое состояние
[X] Rust: логирование
[X] Rust: clippy
[X] Rust: таймауты
[X] Сериализация/десериализация (serde, json)
[X] Разделяемые данные через Arc
[ ] Отдельный скрипт для публикации данных через API
[X] Сохранение данных в случае ошибок/отказа сервиса
[ ] Опционально: Rust: unit-тесты
[ ] Опционально: Rust: стресс-тесты
[ ] Опционально: бенчмаркинг
[X] Опционально: обработка аргументов командной строки (clap)
[-] Опционально: БД/транзакции
[ ] Опционально: Оптимизация кода, WRK и Vegeta

ССЫЛКИ:
- https://habr.com/ru/companies/timeweb/articles/789662/
- https://habr.com/ru/articles/784922/
- https://www.youtube.com/watch?v=q53xalVoc6w
- https://docs.rs/axum/latest/axum/struct.Router.html#method.with_state
- https://www.ncameron.org/blog/dyn-trait-and-impl-trait-in-rust/
- https://rust-cli.github.io/book/in-depth/signals.html
- https://github.com/tokio-rs/axum/discussions/2652
- https://github.com/tokio-rs/axum/blob/main/examples/todos/src/main.rs
- https://github.com/tokio-rs/axum/blob/main/examples/tokio-postgres/src/main.rs
- https://github.com/rstropek/rust-api-fxs/blob/main/todo-axum/src/main.rs

- https://crates.io/crates/refinery
- https://crates.io/crates/utoipa
- https://crates.io/crates/utoipa-swagger-ui

*/

use clap::Parser;
use repository::OrderRepository;
use std::sync::Arc;

/////////////////////////////////////////////////////////////////////
/// Аргументы командной строки

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "localhost")]
    address: Option<String>,
    #[arg(short, long, default_value = "3000")]
    port: Option<String>,
    #[arg(long)]
    pg_enable: bool,
    #[arg(long, required_if_eq("pg_enable", "true"))]
    pg_user: Option<String>,
    #[arg(long, required_if_eq("pg_enable", "true"))]
    pg_pass: Option<String>,
    #[arg(long, required_if_eq("pg_enable", "true"))]
    pg_host: Option<String>,
    #[arg(long, required_if_eq("pg_enable", "true"))]
    pg_port: Option<String>,
    #[arg(long, required_if_eq("pg_enable", "true"))]
    pg_name: Option<String>,
}

/////////////////////////////////////////////////////////////////////
/// MAIN

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Парсинг агрументов командой строки
    let args = Args::parse();
    let address = args.address.unwrap();
    let port = args.port.unwrap();
    let pg_enable = args.pg_enable;

    // Настройка логирования
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_thread_ids(true)
        .init();

    // Настройка хранилища
    let db: Arc<dyn OrderRepository> = if !pg_enable {
        // ... в оперативной памяти - Hash-Map
        repository::local::create()
    } else {
        // ... во внешней БД - PostgeSQL
        let user = args.pg_user.expect("option '--pg-user' not defined");
        let pass = args.pg_pass.expect("option '--pg-pass' not defined");
        let host = args.pg_host.expect("option '--pg-host' not defined");
        let port = args.pg_port.expect("option '--pg-port' not defined");
        let name = args.pg_name.expect("option '--pg-name' not defined");
        repository::postgres::create(&user, &pass, &host, &port, &name)
    };

    // Настройка контроллера
    let socket = format!("{address}:{port}");
    let listener = tokio::net::TcpListener::bind(&socket).await?;
    let controller = controllers::create_router(Arc::clone(&db));

    // Сохранение данных в случае SIGINT (Ctrl-C)
    tokio::spawn({
        let local_db = Arc::clone(&db);
        async move {
            tokio::signal::ctrl_c().await.unwrap();
            local_db.save();
            log::warn!("Exited with SIGINT (Ctrl-C)");
            std::process::exit(0);
        }
    });

    // Запуск
    log::info!("Listening on http://{}", &socket);
    if let Err(error) = axum::serve(listener, controller).await {
        // Сохранение данных в случае ошибки
        db.save();
        log::error!("Exited with error: {error}");
        std::process::exit(1);
    } else {
        Ok(())
    }
}

/////////////////////////////////////////////////////////////////////
/// Атомарные элементы бизнес логики (Домен/Модели/Сущности)
/// Конвертация типов PG: `<https://docs.rs/postgres-types/latest/postgres_types/trait.ToSql.html>`

pub mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Order {
        pub order_uid: String,
        pub track_number: String,
        pub entry: String,
        pub delivery: Delivery,
        pub payment: Payment,
        pub items: Vec<Item>,
        pub locale: String,
        pub internal_signature: String,
        pub customer_id: String,
        pub delivery_service: String,
        pub shardkey: String,
        pub sm_id: u64,
        pub date_created: String,
        pub oof_shard: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Delivery {
        pub name: String,
        pub phone: String,
        pub zip: String,
        pub city: String,
        pub address: String,
        pub region: String,
        pub email: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Payment {
        pub transaction: String,
        pub request_id: String,
        pub currency: String,
        pub provider: String,
        pub amount: u64,
        pub payment_dt: u64,
        pub bank: String,
        pub delivery_cost: u64,
        pub goods_total: u64,
        pub custom_fee: u64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Item {
        pub chrt_id: u64,
        pub track_number: String,
        pub price: u64,
        pub rid: String,
        pub name: String,
        pub sale: u64,
        pub size: String,
        pub total_price: u64,
        pub nm_id: u64,
        pub brand: String,
        pub status: u64,
    }
}

/////////////////////////////////////////////////////////////////////
/// Объекты запросов API (Data Transfer Object)

pub mod dto {
    use crate::models::Order;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct OrderListResponse {
        pub count: usize,
        pub orders: Vec<Order>,
    }
}

/////////////////////////////////////////////////////////////////////
/// Обработчики запросов

pub mod controllers {
    use crate::dto::OrderListResponse;
    use crate::models::Order;
    use crate::repository::OrderRepository;
    use axum::extract::{Json, Path, State};
    use axum::http::{StatusCode, Uri};
    use axum::response::{IntoResponse, Response};
    use axum::routing::get;
    use axum::Router;
    use std::sync::Arc;

    /// Основной маппинг эндпоинтов
    pub fn create_router(state: Arc<dyn OrderRepository>) -> Router {
        Router::new()
            .route("/", get(index))
            .route("/order", get(get_orders).post(add_order))
            .route("/order/:id", get(get_order))
            // .layer(tower_http::trace::TraceLayer::new_for_http()) // без middleware ☹️ реализуем логирование сами в каждом обработчике
            .with_state(state)
    }

    // TODO: SwaggerUI -> https://crates.io/crates/utoipa-swagger-ui
    pub async fn index(route_path: Uri) -> impl IntoResponse {
        log::info!("GET {route_path} -> index() -> {}", StatusCode::OK);
        "Hello, WB!".into_response()
    }

    pub async fn get_orders(
        State(state): State<Arc<dyn OrderRepository>>,
        route_path: Uri,
    ) -> Result<Response, (StatusCode, String)> {
        log::trace!("GET {route_path} -> get_orders()");

        let timeout = tokio::time::Duration::from_millis(1);
        match tokio::time::timeout(timeout, state.get_orders()).await {
            Ok(orders) => {
                let status = StatusCode::OK;
                // используем DTO - улучшаем клиентский опыт, добавив явное отображение кол-ва существующих записей
                let response = OrderListResponse {
                    count: orders.len(),
                    orders: *orders,
                };
                log::info!("GET {route_path} -> get_orders() -> {status}");
                Ok((status, Json(response)).into_response())
            }
            Err(_) => {
                let status = StatusCode::REQUEST_TIMEOUT;
                log::error!("GET {route_path} -> get_orders() -> {status}");
                Err((status, format!("Timed out {timeout:?}")))
            }
        }
    }

    pub async fn get_order(
        State(state): State<Arc<dyn OrderRepository>>,
        route_path: Uri,
        Path(order_uid): Path<String>,
    ) -> Result<Response, (StatusCode, String)> {
        log::trace!("GET {route_path} -> get_order({order_uid})");

        let timeout = tokio::time::Duration::from_millis(1);
        match tokio::time::timeout(timeout, state.get_order(&order_uid)).await {
            Ok(result) => match result {
                Ok(order) => {
                    let status = StatusCode::OK;
                    log::info!("GET {route_path} -> get_order({order_uid}) -> {status}");
                    Ok((status, Json(order)).into_response())
                }
                Err(err) => {
                    let status = StatusCode::NOT_FOUND;
                    log::error!("GET {route_path} -> get_order({order_uid}) -> {status}",);
                    Err((status, err))
                }
            },
            Err(_) => {
                let status = StatusCode::REQUEST_TIMEOUT;
                log::error!("GET {route_path} -> get_order() -> {status}");
                Err((status, format!("Timed out {timeout:?}")))
            }
        }
    }

    pub async fn add_order(
        State(state): State<Arc<dyn OrderRepository>>,
        route_path: Uri,
        Json(payload): Json<Order>,
    ) -> Result<Response, (StatusCode, String)> {
        log::trace!("POST {route_path} -> add_order(payload)");

        let timeout = tokio::time::Duration::from_millis(1);
        match tokio::time::timeout(timeout, state.add_order(&payload)).await {
            Ok(result) => match result {
                Ok(_) => {
                    let status = StatusCode::CREATED;
                    log::info!("POST {route_path} -> add_order(payload) -> {status}");
                    Ok(status.into_response())
                }
                Err(err) => {
                    let status = StatusCode::CONFLICT;
                    log::error!("POST {route_path} -> add_order(payload) -> {status} [{err}]");
                    Err((status, err))
                }
            },
            Err(_) => {
                let status = StatusCode::REQUEST_TIMEOUT;
                log::error!("POST {route_path} -> add_order() -> {status}");
                Err((status, format!("Timed out {timeout:?}")))
            }
        }
    }
}

/////////////////////////////////////////////////////////////////////
/// Хранилища данных и взаимодействие с ними

pub mod repository {
    use crate::models::Order;

    static BACKUP_FILE: &str = "data/orders.bak";

    // Основной интерфейс для работы контроллеров
    #[axum::async_trait]
    pub trait OrderRepository: Sync + Send {
        // Основная работа с хранилищем
        async fn get_orders(&self) -> Box<Vec<Order>>;
        async fn get_order(&self, order_uid: &str) -> Result<Box<Order>, String>;
        async fn add_order(&self, order: &Order) -> Result<(), String>;

        // Локальное сохранение данных в файл
        fn save(&self);
    }

    /// Реализует простейшее in-memory локальное хранилище в виде хеш-таблицы
    pub mod local {
        use super::OrderRepository;
        use crate::models::Order;
        use std::{
            collections::HashMap,
            io::Write,
            sync::{Arc, RwLock},
        };

        pub type Db = Arc<RwLock<HashMap<String, Order>>>;

        // Создание новой hash-map
        pub fn create() -> Arc<dyn OrderRepository> {
            Arc::new(Db::default())
        }

        #[axum::async_trait]
        impl OrderRepository for Db {
            async fn get_orders(&self) -> Box<Vec<Order>> {
                Box::new(self.read().unwrap().values().cloned().collect())
            }

            async fn get_order(&self, order_uid: &str) -> Result<Box<Order>, String> {
                if let Some(order) = self.read().unwrap().get(order_uid) {
                    Ok(Box::new(order.clone()))
                } else {
                    Err(format!("Order with order_uid={order_uid} doesn't exists!"))
                }
            }

            async fn add_order(&self, order: &Order) -> Result<(), String> {
                let key = &order.order_uid;
                if self.read().unwrap().contains_key(key) {
                    Err(format!("Order with order_uid={key} already exists!"))
                } else {
                    self.write().unwrap().insert(key.clone(), order.clone());
                    Ok(())
                }
            }

            fn save(&self) {
                log::info!("Creating backup file from memory -> {}", super::BACKUP_FILE);
                let mut file = std::fs::File::create(super::BACKUP_FILE).unwrap();
                let orders: Vec<Order> = self.read().unwrap().values().cloned().collect();
                let serialized = serde_json::to_string(&orders).unwrap();
                file.write_all(serialized.as_bytes()).unwrap();
            }
        }
    }

    /// Реализует взаимодействие с удаленной БД `PostgreSQL`
    pub mod postgres {
        use super::OrderRepository;
        use crate::models::Order;
        use bb8::Pool;
        use bb8_postgres::PostgresConnectionManager;
        use std::sync::Arc;
        use tokio_postgres::NoTls;
        refinery::embed_migrations!("migrations");

        pub type Db = Pool<PostgresConnectionManager<NoTls>>;

        // TODO: Провести миграцию БД (создание таблиц)
        // pub async fn migrate(connection: ) {
        //     migrations::runner().run_async(&mut connection).await;
        // }

        // Создание пула подключений к БД
        pub fn create(
            user: &str,
            pass: &str,
            host: &str,
            port: &str,
            name: &str,
        ) -> Arc<dyn OrderRepository> {
            let connection_string = format!("postgres://{user}:{pass}@{host}:{port}/{name}");
            let manager =
                PostgresConnectionManager::new_from_stringlike(connection_string, NoTls).unwrap();
            let pool = futures::executor::block_on(async { Pool::builder().build(manager).await })
                .unwrap();
            Arc::new(pool)
        }

        #[axum::async_trait]
        impl OrderRepository for Db {
            async fn get_orders(&self) -> Box<Vec<Order>> {
                let _connection = self.get().await.unwrap();
                Box::new(vec![])
            }

            async fn get_order(&self, _order_uid: &str) -> Result<Box<Order>, String> {
                let _connection = self.get().await.unwrap();
                Err("intenal error".to_string())
            }

            async fn add_order(&self, _order: &Order) -> Result<(), String> {
                let _connection = self.get().await.unwrap();
                Err("intenal error".to_string())
            }

            fn save(&self) {
                log::info!("Creating backup file from PG -> {}", super::BACKUP_FILE);
            }
        }
    }
}
