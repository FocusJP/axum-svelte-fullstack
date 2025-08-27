use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

use crate::{error::AppError, model::auth::UserAuth, state::AppState};

impl FromRequestParts<AppState> for UserAuth {
    type Rejection = AppError;

    #[tracing::instrument(name = "extract_user_auth", skip_all)]
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await?;

        let token = bearer.token().into();

        let user_auth = state.auth_service.parse(token).await?;

        Ok(user_auth)
    }
}
