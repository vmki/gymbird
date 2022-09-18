use crate::error::Error;
use crate::handlers::*;
use crate::models::State;
use warp::hyper::{header, Method, Request};
use warp::{Filter, Rejection, Reply};
use serde_json::json;

pub fn routes(
    state: State,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone + Send {
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

    let fetch_user_path = warp::path("user")
        .and(warp::header("Authorization"))
        .and(state.clone())
        .and_then(fetch_user);

    let log_out_path = warp::path("logout")
        .and(warp::header("Authorization"))
        .and(state.clone())
        .and_then(log_out);

    let exercises_path = warp::path("exercises")
        .and(state.clone())
        .and_then(exercises);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::POST, Method::GET, Method::PUT])
        .allow_header("content-type")
        .allow_header("authorization");

    warp::path("api")
        .and(login_path.or(registration_path).or(fetch_user_path).or(log_out_path).or(exercises_path))
        .with(cors)
        .recover(|err: Rejection| { async move {
            if let Some(e) = err.find::<Error>() {
                let error_message = e.to_string();

                Ok(warp::reply::json(&serde_json::to_string(&json!({
                    "error": error_message,
                })).unwrap()))
            } else {
                Err(err)
            } 
        }})
}
