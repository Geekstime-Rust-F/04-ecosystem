use std::{
    fmt,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use axum::{
    extract::State,
    routing::{get, patch},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{
    de::{MapAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Serialize,
};
use tokio::net::TcpListener;
use tracing::{info, instrument, level_filters::LevelFilter};
use tracing_subscriber::{
    fmt::{format::FmtSpan, Layer},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer as _,
};

#[derive(Debug, Clone)]
struct User {
    name: String,
    age: i32,
    date_of_birth: DateTime<Utc>,
    skills: Vec<String>,
}

#[derive(Debug)]
struct UserUpdate {
    age: Option<i32>,
    skills: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let console = Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry().with(console).init();

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    info!("App listne to: {:?}", addr);

    let user = User {
        name: "Alice".to_string(),
        age: 30,
        date_of_birth: Utc::now(),
        skills: vec!["Rust".to_string(), "Python".to_string()],
    };
    let user = Arc::new(Mutex::new(user));

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/", patch(update_handler))
        .with_state(user);
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[instrument]
async fn index_handler(State(user): State<Arc<Mutex<User>>>) -> Json<User> {
    (*user.lock().unwrap()).clone().into()
}

#[instrument]
async fn update_handler(
    State(user): State<Arc<Mutex<User>>>,
    Json(payload): Json<UserUpdate>,
) -> Json<User> {
    let mut user = user.lock().unwrap();
    if let Some(age) = payload.age {
        user.age = age;
    }
    if let Some(skills) = payload.skills {
        user.skills = skills;
    }
    (*user).clone().into()
}

// 也可以自己手动实现Serialize
impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("User", 4)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("age", &self.age)?;
        state.serialize_field("date_of_birth", &self.date_of_birth)?;
        state.serialize_field("skills", &self.skills)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for UserUpdate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(UserUpdateVisitor)
    }
}

struct UserUpdateVisitor;
impl<'de> Visitor<'de> for UserUpdateVisitor {
    type Value = UserUpdate;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct UserUpdate")
    }

    fn visit_map<V>(self, mut map: V) -> std::result::Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut age = None;
        let mut skills = None;

        while let Some(key) = map.next_key()? {
            match key {
                "age" => {
                    if age.is_some() {
                        return Err(serde::de::Error::duplicate_field("age"));
                    }
                    age = Some(map.next_value()?);
                }
                "skills" => {
                    if skills.is_some() {
                        return Err(serde::de::Error::duplicate_field("skills"));
                    }
                    skills = Some(map.next_value()?);
                }
                _ => {
                    return Err(serde::de::Error::unknown_field(key, &["age", "skills"]));
                }
            }
        }

        Ok(UserUpdate { age, skills })
    }
}
