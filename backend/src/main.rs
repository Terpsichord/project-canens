mod error;
mod spotify;

use anyhow::Context;
use axum::http::HeaderValue;
use axum::routing::get;
use axum::Router;
use rspotify::{ClientCredsSpotify, Credentials};
use shuttle_axum::ShuttleAxum;
use shuttle_runtime::SecretStore;
use tower_http::cors::{Any, CorsLayer};

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> ShuttleAxum {
    let spotify_client_creds = ClientCredsSpotify::new(Credentials::new(
        &secrets
            .get("RSPOTIFY_CLIENT_ID")
            .context("spotify client id not found")?,
        &secrets
            .get("RSPOTIFY_CLIENT_SECRET")
            .context("spotify client secret not found")?,
    ));
    spotify_client_creds
        .request_token()
        .await
        .map_err(|e| anyhow::Error::from(e))?;

    let app = Router::new()
        .route("/spotify/search", get(spotify::search))
        .route("/spotify/song", get(spotify::song))
        .with_state(spotify_client_creds)
        .layer(
            CorsLayer::new()
                // TODO: Change to url of deployed frontend
                .allow_origin(HeaderValue::from_str("https://localhost:8080").unwrap())
                .allow_headers(Any),
        );

    Ok(app.into())
}
