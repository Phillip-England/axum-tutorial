use axum::Json;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonBody {
  message: String,
}

#[derive(Serialize)]
pub struct JsonResponse {
  message: String,
  message_from_server: String,
}

pub async fn mirror_body_json(Json(body): Json<JsonBody>) -> Json<JsonResponse> {
  Json(JsonResponse {
    message: body.message,
    message_from_server: "Server message".to_owned(),
  })
}
