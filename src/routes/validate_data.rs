use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestUser {
  username: Option<String>, // this field is optional
  password: String,
}

pub async fn validate_data(Json(user): Json<RequestUser>) {
  dbg!(user);
}
