use crate::http::extractor::{AuthExtractor, MastodonAuthExtractor};
use axum::{debug_handler, extract::State};
use http::StatusCode;
use kitsune_error::Result;
use kitsune_service::notification::NotificationService;

#[debug_handler(state = crate::state::Zustand)]
pub async fn post(
    State(notification_service): State<NotificationService>,
    AuthExtractor(user_data): MastodonAuthExtractor,
) -> Result<StatusCode> {
    notification_service.clear_all(user_data.account.id).await?;

    Ok(StatusCode::OK)
}
