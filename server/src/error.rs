use axum::{
    http::StatusCode,
    response::{
        IntoResponse,
        Response,
    },
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum HttpResponse {
    #[error("Not Found")]
    NotFound {
        message: String,
    },
}

impl IntoResponse for HttpResponse {
    fn into_response(self) -> Response {
        match self {
            HttpResponse::NotFound {
                message,
            } => (StatusCode::NOT_FOUND, Json(json!({"message": message}))).into_response(),
        }
    }
}

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self.0.downcast_ref::<HttpResponse>() {
            Some(response) => response.clone().into_response(),
            None => {
                let error = self.0;
                let body = json!(
                    {
                        "error": format!("{:#}", error),
                        "backtrace": if cfg!(debug_assertions) {
                            Some(error.backtrace().to_string())
                        } else {
                            None
                        }
                    }
                );
                (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
            }
        }
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self { Self(err.into()) }
}
