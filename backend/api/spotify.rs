use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

use http::{HeaderMap, HeaderName, HeaderValue};
use once_cell::sync::Lazy;
use reqwest::Client;
use serde_json::{json, Value};
use tap::Pipe;
use url::{Host, Url};
use vercel_runtime::{Body, Error, Request, Response, StatusCode};

static ACCOUNTS_URL: Lazy<Host> = Lazy::new(|| Host::Domain("accounts.spotify.com".into()));

#[allow(unused)]
pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    println!("{:?}", std::env::var("RSPOTIFY_CLIENT_ID"));
    let url = Url::parse(&req.uri().to_string())?;
    let spotify_data = get_spotify_data(url, req);

    Ok(spotify_data
        .await
        .map(|value| {
            Response::builder()
                .body(value.to_string().into())
                .expect("Failed to build success response")
        })
        .unwrap_or_else(|(status, err)| {
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

async fn get_spotify_data(req_url: Url, request: Request) -> Result<Value, (StatusCode, Error)> {
    let query_params = req_url
        .query_pairs()
        .into_owned()
        .collect::<HashMap<_, _>>();

    let spotify_url: Url = query_params
        .get("url")
        .ok_or((StatusCode::BAD_REQUEST, "URL parameter is missing".into()))?
        .pipe_deref(Url::parse)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid URL parameter".into()))?;

    let client = Client::new();
    let auth_request = authorised_request(spotify_url, request)?;
    println!("req: {:?}", auth_request);

    let response = client
        .execute(auth_request)
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, e.into()))?;
    println!("resp: {:?}", response);
    println!("content: {:?}", response.text().await);
    return Err((StatusCode::INTERNAL_SERVER_ERROR, "debug".into()));

    let json = response
        .json()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.into()))?;
    println!("json: {:?}", json);

    Ok(json)
}

fn authorised_request(
    spotify_url: Url,
    request: Request,
) -> Result<reqwest::Request, (StatusCode, Error)>
where
{
    let host = spotify_url
        .host()
        .ok_or((StatusCode::BAD_REQUEST, "Invalid URL parameter".into()))?;

    if host != *ACCOUNTS_URL {
        Err((
            StatusCode::BAD_REQUEST,
            format!(
                "Expected \"url\" parameter to have hostname \"{}\" (got {})",
                *ACCOUNTS_URL, host
            )
            .into(),
        ))?;
    }

    let credentials =
        rspotify::Credentials::from_env().expect("Failed to get credentials from environment");

    let cred_headers: HeaderMap = credentials
        .auth_headers()
        .expect("Expected credentials to have secret")
        .iter()
        .map(|(name, value)| {
            (
                HeaderName::from_str(name).expect("Invalid authorization header name"),
                HeaderValue::from_str(value).expect("Invalid authorization header value"),
            )
        })
        .collect();

    let body = request.body().deref().to_owned();
    let http_request = http::Request::from_parts(request.into_parts().0, body);

    let mut request = reqwest::Request::try_from(http_request)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.into()))?;

    request.headers_mut().extend(cred_headers);
    *request.url_mut() = spotify_url;

    Ok(request)
}
