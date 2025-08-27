mod controller;
mod database;
mod error;
mod extract;
mod middleware;
mod model;
mod observability;
mod service;
mod state;

use std::sync::Arc;

use axum::{Router, http::HeaderValue};
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    controller::ssa,
    error::AppError,
    middleware::auth::auth_middleware,
    observability::{trace_layer_make_span, trace_layer_on_response},
    state::AppStateInner,
};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    #[cfg(debug_assertions)]
    dotenv::dotenv()?;

    observability::init()?;

    let app_state = AppStateInner::new().await?;
    let app_state = Arc::new(app_state);

    let auth_middleware_layer =
        axum::middleware::from_fn_with_state(app_state.clone(), auth_middleware);

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace_layer_make_span)
        .on_response(trace_layer_on_response);

    let cors_allowed_origin: HeaderValue = std::env::var("CORS_ALLOWED_ORIGIN")?.parse()?;
    let cors_layer = CorsLayer::permissive().allow_origin(cors_allowed_origin);

    let compression_layer = CompressionLayer::new();

    let app = Router::new()
        .nest("/ssa", ssa::router())
        .route_layer(auth_middleware_layer)
        .layer(trace_layer)
        .layer(cors_layer)
        .layer(compression_layer)
        .with_state(app_state);

    #[cfg(debug_assertions)]
    let app = app
        .merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", controller::ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
