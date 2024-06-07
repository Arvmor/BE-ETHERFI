pub use std::{io::Result, env, collections::HashMap};
pub use actix_web::{HttpServer, App, get, post, delete, patch, web, HttpResponse, Responder, middleware};
pub use actix_cors::Cors;
pub use dotenv::dotenv;
pub use tracing::level_filters::LevelFilter;
pub use tracing_subscriber::{fmt::layer, Registry, layer::SubscriberExt, Layer, util::SubscriberInitExt};
pub use mongodb::{options::{ClientOptions, ResolverConfig, FindOneAndUpdateOptions, ReturnDocument}, Client, Collection, bson::{Uuid, doc, Bson}};
pub use serde::{Serialize, Deserialize};
pub use chrono::{Utc, Duration};
pub use tracing::{error, info};
pub use serde_json::json;

// Our local modules
pub mod credentials;
pub use credentials::*;

pub mod auction;
pub use auction::*;

pub mod db;
pub use db::*;

pub mod helper;
pub use helper::*;

pub mod bid;
pub use bid::*;