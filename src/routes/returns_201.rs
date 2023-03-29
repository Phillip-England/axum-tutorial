use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};

pub async fn returns_201() -> Response {
  (
    StatusCode::CREATED,
    "This is a 201".to_owned(), // leave an empty () to send no data back
  )
    .into_response()
}
