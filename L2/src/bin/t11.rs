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

/// Код HTTP сервера
mod controller {
    use crate::repository::CalendarRepository;
    use crate::{domain::Event, dto::UpdateEvent};
    use axum::{
        extract::State,
        response::IntoResponse,
        routing::{get, post},
        Json, Router,
    };
    use reqwest::StatusCode;
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
            .route("/events_for_day", post(events_for_day))
            .route("/events_for_week", post(events_for_week))
            .route("/events_for_month", post(events_for_month))
            .with_state(state)
    }

    pub async fn index() -> impl IntoResponse {
        "Hello, L2.11!".into_response()
    }

    pub async fn create_event(
        State(state): State<Arc<dyn CalendarRepository>>,
        Json(payload): Json<Event>,
    ) -> impl IntoResponse {
        match state.create_event(payload).await {
            Ok(_) => {
                let content = json!({"result": "success"});
                return (StatusCode::OK, Json(content));
            }
            Err(e) => {
                let content = json!({"error": e});
                return (StatusCode::SERVICE_UNAVAILABLE, Json(content));
            }
        }
    }

    pub async fn update_event(
        State(state): State<Arc<dyn CalendarRepository>>,
        Json(payload): Json<UpdateEvent>,
    ) -> impl IntoResponse {
        match state.update_event(&payload.id, &payload.data).await {
            Ok(_) => {
                let content = json!({"result": "success"});
                return (StatusCode::OK, Json(content));
            }
            Err(e) => {
                let content = json!({"error": e});
                return (StatusCode::SERVICE_UNAVAILABLE, Json(content));
            }
        }
    }

    pub async fn delete_event(
        State(state): State<Arc<dyn CalendarRepository>>,
        Json(payload): Json<Uuid>,
    ) -> impl IntoResponse {
        match state.delete_event(&payload).await {
            Ok(_) => {
                let content = json!({"result": "success"});
                return (StatusCode::OK, Json(content));
            }
            Err(e) => {
                let content = json!({"error": e});
                return (StatusCode::SERVICE_UNAVAILABLE, Json(content));
            }
        }
    }

    pub async fn events_for_day(
        State(state): State<Arc<dyn CalendarRepository>>,
    ) -> impl IntoResponse {
    }

    pub async fn events_for_week(
        State(state): State<Arc<dyn CalendarRepository>>,
    ) -> impl IntoResponse {
    }

    pub async fn events_for_month(
        State(state): State<Arc<dyn CalendarRepository>>,
    ) -> impl IntoResponse {
    }
}

/// Работа с хранилищем данных.
/// Прослойка между кодом HTTP сервера и бизнес логикой
mod repository {
    use crate::domain::Event;
    use uuid::Uuid;

    /// Универсальный интерфейс для работы контроллеров
    #[axum::async_trait]
    pub trait CalendarRepository: Sync + Send {
        async fn create_event(&self, event: Event) -> Result<(), String>;
        async fn update_event(&self, id: &Uuid, other: &Event) -> Result<(), String>;
        async fn delete_event(&self, id: &Uuid) -> Result<(), String>;
    }

    /// Реализация взаимодействия с хранилищем через Arc + RwLock
    pub mod default {
        use super::CalendarRepository;
        use crate::domain::{Calendar, Event};
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
        }
    }
}

/// Data Transfer Objects.
/// Вспомогательные объекты и функции
mod dto {
    use crate::domain::Event;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Serialize, Deserialize)]
    pub struct UpdateEvent {
        pub id: Uuid,
        pub data: Event,
    }
}

/// Бизнес логика
mod domain {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use uuid::Uuid;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct Event {
        title: String,
        description: String,
        date: DateTime<Utc>,
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

        pub fn find_events(&mut self, start: &DateTime<Utc>, end: &DateTime<Utc>) -> Vec<&Event> {
            vec![]
        }
    }
}
