// L2.11

/*

Usage: t11.exe [OPTIONS]

Options:
  -p, --port <PORT>  [default: 80]
  -h, --help         Print help

*/

use clap::Parser;
use repository::CalendarRepository;
use std::sync::Arc;

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 80)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Парсинг агрументов командой строки
    let args = Args::parse();

    // Настройка логирования
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_thread_ids(true)
        .init();

    // Настройка хранилища
    let repository: Arc<dyn CalendarRepository> = repository::default::create();

    // Настройка контроллера
    let socket = format!("localhost:{}", &args.port);
    let listener = tokio::net::TcpListener::bind(&socket).await?;
    let controller = controller::create_router(Arc::clone(&repository));

    // Запуск
    log::info!("Listening on {socket}");
    axum::serve(listener, controller).await.unwrap();

    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Код HTTP сервера
mod controller {
    use crate::domain::Event;
    use crate::dto::{response, QueryDate, RequestUpdateEvent};
    use crate::repository::CalendarRepository;
    use axum::extract::{Query, Request};
    use axum::middleware::{self, Next};
    use axum::{
        extract::State,
        http::StatusCode,
        response::IntoResponse,
        routing::{get, post},
        Json, Router,
    };
    use chrono::{Days, Months};
    use log::{error, info};
    use serde_json::json;
    use std::sync::Arc;
    use uuid::Uuid;

    /// Основной маппинг эндпоинтов
    pub fn create_router(state: Arc<dyn CalendarRepository>) -> Router {
        Router::new()
            .route("/", get(index))
            .route("/create_event", post(create_event))
            .route("/update_event", post(update_event))
            .route("/delete_event", post(delete_event))
            .route("/events_for_day", get(events_for_day))
            .route("/events_for_week", get(events_for_week))
            .route("/events_for_month", get(events_for_month))
            .with_state(state)
            .layer(middleware::from_fn(logger))
    }

    /// Middleware - логирование
    async fn logger(request: Request, next: Next) -> impl IntoResponse {
        let method = request.method().clone();
        let uri = request.uri().clone();
        let response = next.run(request).await;
        let status = response.status();
        match status {
            StatusCode::OK => info!("{method} {uri} -> {status}"),
            _ => error!("{method} {uri} -> {status}"),
        };

        response
    }

    async fn index() -> impl IntoResponse {
        "Hello, L2.11!".into_response()
    }

    async fn create_event(
        State(state): State<Arc<dyn CalendarRepository>>,
        Json(payload): Json<Event>,
    ) -> impl IntoResponse {
        match state.create_event(payload).await {
            Ok(_) => (StatusCode::OK, response(Ok("successfully created"))),
            Err(e) => (StatusCode::SERVICE_UNAVAILABLE, response(Err(&e))),
        }
    }

    async fn update_event(
        State(state): State<Arc<dyn CalendarRepository>>,
        Json(payload): Json<RequestUpdateEvent>,
    ) -> impl IntoResponse {
        match state.update_event(&payload.id, &payload.data).await {
            Ok(_) => (StatusCode::OK, response(Ok("successfully updated"))),
            Err(e) => (StatusCode::SERVICE_UNAVAILABLE, response(Err(&e))),
        }
    }

    async fn delete_event(
        State(state): State<Arc<dyn CalendarRepository>>,
        Json(payload): Json<Uuid>,
    ) -> impl IntoResponse {
        match state.delete_event(&payload).await {
            Ok(_) => (StatusCode::OK, response(Ok("successfully deleted"))),
            Err(e) => (StatusCode::SERVICE_UNAVAILABLE, response(Err(&e))),
        }
    }

    async fn events_for_day(
        State(state): State<Arc<dyn CalendarRepository>>,
        Query(query): Query<QueryDate>,
    ) -> impl IntoResponse {
        let date = match query.create_for_day() {
            Ok(date) => date,
            Err(e) => return (StatusCode::BAD_REQUEST, response(Err(&e))),
        };

        let events = state.find_events(&date, &date).await;
        return (StatusCode::OK, Json(json!( { "success": events } )));
    }

    async fn events_for_week(
        State(state): State<Arc<dyn CalendarRepository>>,
        Query(query): Query<QueryDate>,
    ) -> impl IntoResponse {
        let start = match query.create_for_week() {
            Ok(date) => date,
            Err(e) => return (StatusCode::BAD_REQUEST, response(Err(&e))),
        };
        let end = start + Days::new(7);

        let events = state.find_events(&start, &end).await;
        return (StatusCode::OK, Json(json!( { "success": events } )));
    }

    async fn events_for_month(
        State(state): State<Arc<dyn CalendarRepository>>,
        Query(query): Query<QueryDate>,
    ) -> impl IntoResponse {
        let start = match query.create_for_month() {
            Ok(date) => date,
            Err(e) => return (StatusCode::BAD_REQUEST, response(Err(&e))),
        };
        let end = start + Months::new(1);

        let events = state.find_events(&start, &end).await;
        return (StatusCode::OK, Json(json!( { "success": events } )));
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Работа с хранилищем данных.
/// Прослойка между кодом HTTP сервера и бизнес логикой
mod repository {
    use crate::domain::Event;
    use chrono::NaiveDate;
    use uuid::Uuid;

    /// Универсальный интерфейс для работы контроллеров
    #[axum::async_trait]
    pub trait CalendarRepository: Sync + Send {
        async fn create_event(&self, event: Event) -> Result<(), String>;
        async fn update_event(&self, id: &Uuid, other: &Event) -> Result<(), String>;
        async fn delete_event(&self, id: &Uuid) -> Result<(), String>;
        async fn find_events(&self, start: &NaiveDate, end: &NaiveDate) -> Vec<Event>;
    }

    /// Реализация взаимодействия с хранилищем через Arc + RwLock
    pub mod default {
        use super::CalendarRepository;
        use crate::domain::{Calendar, Event};
        use chrono::NaiveDate;
        use std::sync::Arc;
        use tokio::sync::RwLock;
        use uuid::Uuid;

        pub type Db = Arc<RwLock<Calendar>>;

        pub fn create() -> Arc<dyn CalendarRepository> {
            Arc::new(Db::default())
        }

        #[axum::async_trait]
        impl CalendarRepository for Db {
            async fn create_event(&self, event: Event) -> Result<(), String> {
                self.write().await.create_event(event)
            }

            async fn update_event(&self, id: &Uuid, other: &Event) -> Result<(), String> {
                self.write().await.update_event(id, other)
            }

            async fn delete_event(&self, id: &Uuid) -> Result<(), String> {
                self.write().await.delete_event(id)
            }

            async fn find_events(&self, start: &NaiveDate, end: &NaiveDate) -> Vec<Event> {
                self.read().await.find_events(start, end)
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Data Transfer Objects.
/// Вспомогательные объекты и функции
mod dto {
    use crate::domain::Event;
    use axum::Json;
    use chrono::{NaiveDate, Weekday};
    use serde::Deserialize;
    use serde_json::json;
    use uuid::Uuid;

    #[derive(Deserialize)]
    pub struct RequestUpdateEvent {
        pub id: Uuid,
        pub data: Event,
    }

    #[derive(Deserialize)]
    pub struct QueryDate {
        pub year: i32,
        pub month: Option<u32>,
        pub week: Option<u32>,
        pub day: Option<u32>,
    }

    impl QueryDate {
        pub fn create_for_day(&self) -> Result<NaiveDate, &str> {
            let day = match self.day {
                Some(day) => day,
                None => return Err("no day were provided"),
            };

            let date = match self.month {
                Some(month) => NaiveDate::from_ymd_opt(self.year, month, day),
                None => NaiveDate::from_yo_opt(self.year, day),
            };

            match date {
                Some(date) => return Ok(date),
                None => Err("invalid date"),
            }
        }

        pub fn create_for_week(&self) -> Result<NaiveDate, &str> {
            match self.week {
                Some(week) => match NaiveDate::from_isoywd_opt(self.year, week, Weekday::Mon) {
                    Some(date) => Ok(date),
                    None => Err("invalid date"),
                },
                None => Err("no week were provided"),
            }
        }

        pub fn create_for_month(&self) -> Result<NaiveDate, &str> {
            match self.month {
                Some(month) => match NaiveDate::from_ymd_opt(self.year, month, 1) {
                    Some(date) => Ok(date),
                    None => Err("invalid date"),
                },
                None => Err("no month were provided"),
            }
        }
    }

    pub fn response(message: Result<&str, &str>) -> Json<serde_json::Value> {
        match message {
            Ok(message) => Json(json!( { "success": message } )),
            Err(message) => Json(json!( { "error": message } )),
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Бизнес логика
mod domain {
    use chrono::NaiveDate;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use uuid::Uuid;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct Event {
        title: String,
        description: String,
        date: NaiveDate,
    }

    #[derive(Default)]
    pub struct Calendar {
        events: HashMap<Uuid, Event>,
    }

    impl Calendar {
        pub fn create_event(&mut self, event: Event) -> Result<(), String> {
            match self.events.iter().find(|(_, value)| **value == event) {
                None => {
                    self.events.insert(Uuid::new_v4(), event);
                    Ok(())
                }
                Some(x) => Err(format!("This such event already exists with id: {}", x.0)),
            }
        }

        pub fn update_event(&mut self, id: &Uuid, other: &Event) -> Result<(), String> {
            match self.events.get_mut(id) {
                Some(event) => {
                    event.title = other.title.clone();
                    event.description = other.description.clone();
                    event.date = other.date.clone();
                    Ok(())
                }
                None => Err(format!("Cannot find event with id: {id}")),
            }
        }

        pub fn delete_event(&mut self, id: &Uuid) -> Result<(), String> {
            match self.events.remove(id) {
                Some(_) => Ok(()),
                None => Err(format!("Cannot find event with id: {id}")),
            }
        }

        pub fn find_events(&self, start: &NaiveDate, end: &NaiveDate) -> Vec<Event> {
            let mut result = vec![];
            for (_, event) in self.events.iter() {
                if start <= &event.date && &event.date <= end {
                    result.push(event.clone());
                }
            }
            result
        }
    }
}
