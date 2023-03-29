use axum::http::StatusCode;

pub async fn always_errors() -> Result<(), StatusCode> {
  Err(StatusCode::BAD_REQUEST)
}
