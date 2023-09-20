use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, strum_macros::AsRefStr)]
pub enum Error {
    LoginFail,

    // AUTH errors
    AuthFailNoAuthTokenCookie,

    AuthFailTokenWrongFormat,

    AuthFailCtxNotInRequestExt,

    //model errors
    TicketDeleteFailIdNotFound { id: u64 },
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            // -- Auth
            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInRequestExt => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            //-- Model
            Self::TicketDeleteFailIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }
            //-- Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");
        //create a placeholder Axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        //insert the Error into the response
        response.extensions_mut().insert(self);

        response
    }
}
