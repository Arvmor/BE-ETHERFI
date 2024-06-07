pub use std::{io::Result, env};
pub use actix_web::{HttpServer, App, get, post, web, HttpResponse, Responder, middleware};
pub use actix_cors::Cors;
pub use dotenv::dotenv;
pub use tracing::level_filters::LevelFilter;
pub use tracing_subscriber::{fmt::layer, Registry, layer::SubscriberExt, Layer, util::SubscriberInitExt};
pub use mongodb::{options::{ClientOptions, ResolverConfig}, Client, Collection, bson::Uuid};

// Our local modules
pub mod credentials;
pub use credentials::*;

pub mod auction;
pub use auction::*;

pub mod db;
pub use db::*;