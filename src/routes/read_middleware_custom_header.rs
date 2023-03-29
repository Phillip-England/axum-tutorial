use crate::routes::HeaderMessage;
use axum::Extension;

pub async fn read_middleware_custom_header(Extension(message): Extension<HeaderMessage>) -> String {
  message.0
}
