use http::{header, HeaderValue};
use tower::{Service, ServiceBuilder, ServiceExt};
use tower_http::cors::{Any, CorsLayer};
use tower_http::set_header::SetResponseHeaderLayer;
use vercel_runtime::{bundled_api, run, Body, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut service = ServiceBuilder::new()
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

#[bundled_api]
pub async fn handler(req: Request) -> Result<Response<Body>, Error> {}
