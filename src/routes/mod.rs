use mongodb::{Client};

mod always_errors;
mod get_json;
mod hello_world;
mod middleware_message;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_custom_header;
mod mirror_user_agent;
mod path_variables;
mod query_params;
mod read_middleware_custom_header;
mod returns_201;
mod validate_data;

use crate::middleware::set_custom_header::set_custom_header;
use always_errors::always_errors;
use get_json::get_json;
use hello_world::hello_world;
use middleware_message::middleware_message;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_header::mirror_custom_header;
use mirror_user_agent::mirror_user_agent;
use path_variables::path_variables;
use query_params::query_params;
use read_middleware_custom_header::read_middleware_custom_header;
use returns_201::returns_201;
use validate_data::validate_data;

use axum::{
  http::Method,
  middleware::from_fn,
  routing::{get, post},
  Extension, Router,
};
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct SharedData {
  pub message: String,
}

#[derive(Clone)]
pub struct HeaderMessage(pub String);

pub fn create_routes(client: Client) -> Router {
  let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
    .allow_origin(Any);

  let shared_data = SharedData {
    message: "hello from shared data".to_owned(),
  };

  Router::new()
    // routes that run a unique middleware layer must be positioned above standard routes
    .route(
      "/read_middleware_custom_header",
      get(read_middleware_custom_header),
    )
    .route_layer(from_fn(set_custom_header))
    .route("/", get(hello_world))
    .route("/mirror_body_string", post(mirror_body_string))
    .route("/mirror_body_json", post(mirror_body_json))
    .route("/mirror_user_agent", get(mirror_user_agent))
    .route("/path_variables/:id", get(path_variables))
    .route("/query_params", get(query_params))
    .route("/mirror_custom_header", get(mirror_custom_header))
    .route("/middleware_message", get(middleware_message))
    .route("/always_errors", get(always_errors))
    .route("/returns_201", post(returns_201))
    .route("/get_json", get(get_json))
    .route("/validate_data", post(validate_data))
    // middleware which applies to all route must sit below all routes
    .layer(cors)
    .layer(Extension(shared_data))
}
