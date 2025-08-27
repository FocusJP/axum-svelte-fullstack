use axum::{Extension, Json, Router, extract::State, routing::post};

use crate::{
    database,
    error::AppError,
    model::{
        auth::UserAuth,
        ssa::{request::GetNameStatsQuery, response::GetNameStatsResponse},
    },
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new().route("/name-stats", post(get_name_stats))
}

#[utoipa::path(
    get,
    path = "/ssa/name-stats",
    responses(
        (status = 200, description = "Yearly count of SSA registered names", body = GetNameStatsResponse),
        (status = BAD_REQUEST),
        (status = UNAUTHORIZED),
        (status = UNSUPPORTED_MEDIA_TYPE),
        (status = UNPROCESSABLE_ENTITY),
        (status = INTERNAL_SERVER_ERROR)
    ),
    request_body(content = GetNameStatsQuery, content_type = "application/json")
)]
#[tracing::instrument(skip(state))]
async fn get_name_stats(
    Extension(user_auth): Extension<UserAuth>,
    State(state): State<AppState>,
    Json(query): Json<GetNameStatsQuery>,
) -> Result<Json<GetNameStatsResponse>, AppError> {
    if !user_auth.permissions.iter().any(|item| item.as_str() == "read:ssa_stats") {
        return Err(AppError::Forbidden(
            "read:ssa_stats permission required".into(),
        ));
    }
    
    if query.start_year > query.end_year {
        return Err(AppError::UnprocessableEntity(
            "Start year must not be after end year".into(),
        ));
    }

    let database_connection = state.get_database_connection().await?;

    let name_stats = database::ssa::get_name_stats(database_connection, query).await?;

    Ok(Json(name_stats))
}
