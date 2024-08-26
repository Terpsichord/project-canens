use http::{header, HeaderValue};
use serde_json::json;
use std::future::Future;
use tower::{Service, ServiceBuilder, ServiceExt};
use tower_http::cors::{Any, CorsLayer};
use tower_http::set_header::SetResponseHeaderLayer;
use vercel_runtime::{run, Body, Error as VercelError, Request, Response, StatusCode};

pub type Result<T> = std::result::Result<T, Error>;
pub struct Error(StatusCode, VercelError);

impl<S: Into<StatusCode>, E: Into<VercelError>> From<(S, E)> for Error {
    fn from(value: (S, E)) -> Self {
        Error(value.0.into(), value.1.into())
    }
}

impl From<VercelError> for Error {
    fn from(value: VercelError) -> Self {
        Error(StatusCode::INTERNAL_SERVER_ERROR, value.into())
    }
}

pub fn wrap_response<T: serde::Serialize>(
    res: Result<T>,
) -> std::result::Result<Response<Body>, VercelError> {
    Ok(res
        .map(|value| {
            Response::builder()
                .body(
                    serde_json::to_string(&value)
                        .expect("Failed to serialize response into JSON")
                        .into(),
                )
                .expect("Failed to build success response")
        })
        .unwrap_or_else(|Error(status, err)| {
            Response::builder()
                .status(status)
                .body(
                    json! ({
                        "error": err.to_string()
                    })
                    .to_string()
                    .into(),
                )
                .expect("Failed to build error response")
        }))
}

pub async fn handle<T, F, S>(handler: T) -> std::result::Result<(), VercelError>
where
    T: FnMut(Request) -> F,
    F: Future<Output = Result<S>>,
    S: serde::Serialize,
{
    let mut service = ServiceBuilder::new()
        .map_result(wrap_response)
        .layer(
            CorsLayer::new()
                .allow_origin(
                    Any, /* HeaderValue::from_str("https://localhost:8080")?*/
                )
                .allow_headers(Any),
        )
        .layer(SetResponseHeaderLayer::if_not_present(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        ))
        .service_fn(handler);

    let service = service.ready().await?;

    run(|req| service.call(req)).await
}

pub fn spotify_client_creds() -> rspotify::ClientCredsSpotify {
    let credentials =
        rspotify::Credentials::from_env().expect("Failed to get credentials from environment");
    rspotify::ClientCredsSpotify::new(credentials)
}
