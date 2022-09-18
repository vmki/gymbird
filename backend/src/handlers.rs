use crate::models;
use crate::database::SessionToken;
use serde_json::json;
use warp::{Filter, Rejection, Reply};

pub async fn login(
    params: models::LoginParameters,
    state: models::State,
) -> anyhow::Result<impl Reply, Rejection> {
    println!("POST /api/login: {:?}", params);
    let state = state.lock().await;

    match state.login(params).await {
        Ok(token) => {
            let json = json!({
                "session_token": token,
            }); 

            Ok(warp::reply::json(&serde_json::to_string(&json).unwrap()))
        }
        Err(e) => Err(warp::reject::custom(e))
    }

}

pub async fn register(
    params: models::RegistrationParameters,
    state: models::State,
) -> anyhow::Result<impl Reply, Rejection> {
    println!("POST /api/register: {:?}", params);
    let state = state.lock().await;

    let session_token = state.create_user(params).await.unwrap();

    let json = json!({
        "session_token": session_token,
    }); 

    Ok(warp::reply::json(&serde_json::to_string(&json).unwrap()))
}

pub async fn fetch_user(
    token: SessionToken,
    state: models::State,
) -> anyhow::Result<impl Reply, Rejection> {
    println!("GET /api/user: {}", token);
    let state = state.lock().await;

    let user_account = state.fetch_user(token).await?;

    Ok(warp::reply::json(
        &serde_json::to_string(&user_account).unwrap(),
    ))
}

pub async fn log_out(
    token: SessionToken,
    state: models::State,
) -> anyhow::Result<impl Reply, Rejection> {
    println!("GET /api/logout: {}", token);
    let state = state.lock().await;

    state.log_out(token).await;

    Ok(warp::reply::reply())
}

pub async fn exercises(state: models::State) -> anyhow::Result<impl Reply, Rejection> {
    let state = state.lock().await;

    let exercises = state.get_all_exercises().await;

    Ok(warp::reply::json(&serde_json::to_string(&exercises).unwrap()))
}
