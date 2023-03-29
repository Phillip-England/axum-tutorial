use axum::extract::Path;

pub async fn path_variables(Path(id): Path<String>) -> String {
  id.to_string()
}
