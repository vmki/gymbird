use backend::models;
use serde_json::to_string;
use warp::http::header::{HeaderMap, HeaderValue};
use warp::hyper::{header, Method, Request};
use warp::{Filter, Rejection, Reply};

async fn login(
    params: models::LoginParameters,
    state: models::State,
) -> anyhow::Result<impl Reply, Rejection> {
    println!("POST /api/login: {:?}", params);
    let state = state.lock().await;

    let user_account = state.get_user_by_email(params.email).await.unwrap();

    Ok(warp::reply::json(
        &serde_json::to_string(&user_account).unwrap(),
    ))
}

async fn register(
    params: models::RegistrationParameters,
    state: models::State,
) -> anyhow::Result<impl Reply, Rejection> {
    println!("POST /api/register: {:?}", params);
    let state = state.lock().await;

    let user_account = state.create_user(params).await.unwrap();

    Ok(warp::reply::json(
        &serde_json::to_string(&user_account).unwrap(),
    ))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state: models::State = models::new_state().await?;

    let state = warp::any().map(move || state.clone());

    let login_path = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and(state.clone())
        .and_then(login);

    let registration_path = warp::path("register")
        .and(warp::post())
        .and(warp::body::json())
        .and(state.clone())
        .and_then(register);

    let api_routes = warp::path("api")
        .and(login_path.or(registration_path))
        .with(
            warp::cors()
                .allow_any_origin()
                .allow_methods(&[Method::POST, Method::GET, Method::PUT])
                .allow_header("content-type"),
        );

    println!("Starting warp.");

    Ok(warp::serve(api_routes).run(([127, 0, 0, 1], 8080)).await)
}
