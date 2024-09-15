/* Автор: Егор Трухин @egortrue

КОМАНДЫ:
- проверка: cargo clippy -- -W clippy::pedantic
- запуск: cargo watch -c -d 0 -x run
- запуск с PG: cargo watch -c -d 0 -x "run -- --pg-enable --pg-user myuser --pg-pass mypassword --pg-host localhost --pg-port 5432 --pg-name postgres"

КОММЕНТАРИИ:
- Все решение в одном файле - здесь
- Для поддержания чистоты кода, учитывая его кол-во, разбил все на модули в этом же файле
- Постарался придерживаться чистой архитектруры

ТЕМЫ:
[X] Axum (без middleware), разделяемое состояние
[X] Rust: логирование
[X] Rust: clippy
[ ] Rust: таймауты
[X] Сериализация/десериализация (serde, json)
[X] Разделяемые данные через Arc
[ ] Отдельный скрипт для публикации данных через API
[ ] Сохранение данных в случае ошибок/отказа сервиса
[ ] Опционально: Rust: unit-тесты
[ ] Опционально: Rust: стресс-тесты
[ ] Опционально: бенчмаркинг
[X] Опционально: обработка аргументов командной строки (clap)
[ ] Опционально: БД/транзакции
[ ] Опционально: Оптимизация кода, WRK и Vegeta

ССЫЛКИ:
- https://habr.com/ru/companies/timeweb/articles/789662/
- https://habr.com/ru/articles/784922/
- https://www.youtube.com/watch?v=q53xalVoc6w
- https://docs.rs/axum/latest/axum/struct.Router.html#method.with_state
- https://www.ncameron.org/blog/dyn-trait-and-impl-trait-in-rust/
- https://github.com/tokio-rs/axum/discussions/2652
- https://github.com/tokio-rs/axum/blob/main/examples/todos/src/main.rs
- https://github.com/tokio-rs/axum/blob/main/examples/tokio-postgres/src/main.rs
- https://github.com/rstropek/rust-api-fxs/blob/main/todo-axum/src/main.rs

- https://crates.io/crates/refinery
- https://crates.io/crates/utoipa
- https://crates.io/crates/utoipa-swagger-ui

*/

use clap::Parser;

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "localhost")]
    address: Option<String>,
    #[arg(short, long, default_value = "3000")]
    port: Option<String>,
    #[arg(long, requires = "pg")]
    pg_enable: bool,
    #[arg(long, group = "pg")]
    pg_user: Option<String>,
    #[arg(long, group = "pg")]
    pg_pass: Option<String>,
    #[arg(long, group = "pg")]
    pg_host: Option<String>,
    #[arg(long, group = "pg")]
    pg_port: Option<String>,
    #[arg(long, group = "pg")]
    pg_name: Option<String>,
}

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

    // Настройка сокета
    let socket = format!("{address}:{port}");
    let listener = tokio::net::TcpListener::bind(&socket).await?;

    // Настройка контроллера
    let controller = if !pg_enable {
        let state = repository::local::create();
        controllers::create_router(state)
    } else {
        let user = args.pg_user.expect("option '--pg-user' not defined");
        let pass = args.pg_pass.expect("option '--pg-pass' not defined");
        let host = args.pg_host.expect("option '--pg-host' not defined");
        let port = args.pg_port.expect("option '--pg-port' not defined");
        let name = args.pg_name.expect("option '--pg-name' not defined");

        let state = repository::postgres::create(&user, &pass, &host, &port, &name);
        controllers::create_router(state)
    };

    // Запуск
    log::info!("Listening on http://{}", &socket);
    axum::serve(listener, controller).await?;

    Ok(())
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
    use crate::repository::OrderRepo;
    use axum::extract::{Json, Path, State};
    use axum::http::{StatusCode, Uri};
    use axum::response::IntoResponse;
    use axum::routing::get;
    use axum::Router;

    /// Основной маппинг эндпоинтов
    pub fn create_router(state: Box<impl OrderRepo>) -> Router {
        Router::new()
            .route("/", get(index))
            .route("/order", get(get_orders).post(add_order))
            .route("/order/:id", get(get_order))
            .with_state(state)
        // .layer(tower_http::trace::TraceLayer::new_for_http()) // без middleware ☹️ реализуем логирование сами в каждом обработчике
    }

    // TODO: SwaggerUI -> https://crates.io/crates/utoipa-swagger-ui
    async fn index(route_path: Uri) -> impl IntoResponse {
        log::info!("GET {route_path} -> index() -> {}", StatusCode::OK);
        "Hello, WB!".into_response()
    }

    async fn get_orders(
        State(state): State<Box<impl OrderRepo>>,
        route_path: Uri,
    ) -> impl IntoResponse {
        log::trace!("GET {route_path} -> get_orders()");
        let orders = state.get_orders();
        let response = OrderListResponse {
            count: orders.len(),
            orders,
        };

        // TODO: Сделать с timeout. Не удается привязать handler к route
        // https://github.com/tokio-rs/axum/discussions/873

        // let max_duration = tokio::time::Duration::from_millis(1);
        // match tokio::time::timeout(max_duration, state.get_orders()).await {
        //     Ok(orders) => {
        //         status = StatusCode::OK;
        //         let response = OrderListResponse {
        //             count: orders.len(), // используем DTO - улучшаем клиентский опыт, добавив явное отображение кол-ва существующих записей
        //             orders: orders,
        //         };
        //         log::info!("GET {route_path} -> get_orders() -> {status}");
        //         (status, Json(response)).into_response()
        //     }
        //     Err(_) => {
        //         status = StatusCode::REQUEST_TIMEOUT;
        //         log::error!("GET {route_path} -> get_orders() -> {status}");
        //         status.into_response()
        //     }
        // }

        log::info!("GET {route_path} -> get_orders() -> {}", StatusCode::OK);
        (StatusCode::OK, Json(response))
    }

    async fn get_order(
        State(state): State<Box<impl OrderRepo>>,
        route_path: Uri,
        Path(order_uid): Path<String>,
    ) -> impl IntoResponse {
        log::trace!("GET {route_path} -> get_order({order_uid})");
        let status: StatusCode;

        if let Some(order) = state.get_order(&order_uid) {
            status = StatusCode::OK;
            log::info!("GET {route_path} -> get_order({order_uid}) -> {status}");
            (status, Json(order)).into_response()
        } else {
            status = StatusCode::NOT_FOUND;
            log::error!("GET {route_path} -> get_order({order_uid}) -> {status}",);
            status.into_response()
        }
    }

    async fn add_order(
        State(state): State<Box<impl OrderRepo>>,
        route_path: Uri,
        Json(payload): Json<Order>,
    ) -> impl IntoResponse {
        log::trace!("POST {route_path} -> add_order(payload)");
        let status: StatusCode;

        match state.add_order(&payload) {
            Ok(()) => {
                status = StatusCode::CREATED;
                log::info!("POST {route_path} -> add_order(payload) -> {status}");
                status.into_response()
            }
            Err(err) => {
                status = StatusCode::CONFLICT;
                log::error!("POST {route_path} -> add_order(payload) -> {status} [{err}]");
                (status, err).into_response()
            }
        }
    }
}

/////////////////////////////////////////////////////////////////////
/// Хранилища данных и взаимодействие с ними

pub mod repository {
    use crate::models::Order;

    // Основной интерфейс для работы контроллеров
    pub trait OrderRepo: Clone + Send + Sync + 'static {
        fn get_orders(&self) -> Vec<Order>;
        fn get_order(&self, order_uid: &str) -> Option<Order>;
        fn add_order(&self, order: &Order) -> Result<(), String>;
    }

    /// Реализует простейшее in-memory локальное хранилище в виде хеш-таблицы
    pub mod local {
        use super::OrderRepo;
        use crate::models::Order;
        use std::{
            collections::HashMap,
            sync::{Arc, RwLock},
        };

        pub type Db = Arc<RwLock<HashMap<String, Order>>>;

        pub fn create() -> Box<impl OrderRepo> {
            Box::new(Db::default())
        }

        impl OrderRepo for Db {
            fn get_orders(&self) -> Vec<Order> {
                self.read().unwrap().values().cloned().collect()
            }

            fn get_order(&self, order_uid: &str) -> Option<Order> {
                self.read().unwrap().get(order_uid).cloned()
            }

            fn add_order(&self, order: &Order) -> Result<(), String> {
                if self.read().unwrap().contains_key(&order.order_uid) {
                    Err(format!(
                        "Order with order_uid=\"{}\" already exists!",
                        &order.order_uid
                    ))
                } else {
                    self.write()
                        .unwrap()
                        .insert(order.order_uid.clone(), order.clone());
                    Ok(())
                }
            }
        }
    }

    /// Реализует взаимодействие с удаленной БД `PostgreSQL`
    pub mod postgres {
        use super::OrderRepo;
        use crate::models::Order;
        use bb8::{ManageConnection, Pool};
        use bb8_postgres::PostgresConnectionManager;
        use refinery::Migration;
        use tokio_postgres::{Connection, NoTls};
        refinery::embed_migrations!("migrations");

        pub type Db = Pool<PostgresConnectionManager<NoTls>>;

        // Провести миграцию БД (создание таблиц)
        // pub async fn migrate(connection: ) {
        //     migrations::runner().run_async(&mut connection).await;
        // }

        pub fn create(
            user: &str,
            pass: &str,
            host: &str,
            port: &str,
            name: &str,
        ) -> Box<impl OrderRepo> {
            let connection_string = format!("postgres://{user}:{pass}@{host}:{port}/{name}");
            let manager =
                PostgresConnectionManager::new_from_stringlike(connection_string, NoTls).unwrap();
            let pool = futures::executor::block_on(async { Pool::builder().build(manager).await })
                .unwrap();
            Box::new(pool)
        }

        impl OrderRepo for Db {
            fn get_orders(&self) -> Vec<Order> {
                todo!()
            }

            fn get_order(&self, order_uid: &str) -> Option<Order> {
                log::error!("not implemented {}", order_uid);
                todo!()
            }

            fn add_order(&self, order: &Order) -> Result<(), String> {
                log::error!("not implemented {}", order.order_uid);
                todo!()
            }
        }
    }
}
