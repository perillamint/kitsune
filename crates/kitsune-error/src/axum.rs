use crate::{Error, ErrorType};
use axum_core::response::{IntoResponse, Response};
use http::StatusCode;

#[inline]
fn to_response<B>(status_code: StatusCode, maybe_body: Option<B>) -> Response
where
    B: IntoResponse,
{
    maybe_body.map_or_else(
        || status_code.into_response(),
        |body| (status_code, body).into_response(),
    )
}

impl From<Error> for Response {
    #[inline]
    fn from(value: Error) -> Self {
        value.into_response()
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!(error = ?self.inner);

        if let Some(garde_report) = self.inner.downcast_ref::<garde::Report>() {
            let body = match sonic_rs::to_string(&garde_report) {
                Ok(body) => body,
                Err(error) => return Error::from(error).into_response(),
            };

            return to_response(StatusCode::BAD_REQUEST, Some(body));
        }

        let maybe_body = self.ctx.body.into_inner();
        match self.ctx.ty {
            ErrorType::BadRequest => to_response(StatusCode::BAD_REQUEST, maybe_body),
            ErrorType::Forbidden => to_response(StatusCode::FORBIDDEN, maybe_body),
            ErrorType::NotFound => to_response(StatusCode::NOT_FOUND, maybe_body),
            ErrorType::Unauthorized => to_response(StatusCode::UNAUTHORIZED, maybe_body),
            ErrorType::UnsupportedMediaType => {
                to_response(StatusCode::UNSUPPORTED_MEDIA_TYPE, maybe_body)
            }
            ErrorType::Other => to_response(StatusCode::INTERNAL_SERVER_ERROR, maybe_body),
        }
    }
}
