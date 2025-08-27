use axum::{extract::Request, middleware::Next, response::Response};

use crate::{error::AppError, model::auth::UserAuth};

#[tracing::instrument(skip(request, next))]
pub async fn auth_middleware(
    user_auth: UserAuth,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    request.extensions_mut().insert(user_auth);
    Ok(next.run(request).await)
}
