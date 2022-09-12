use crate::models;
use warp::{Filter, Rejection, Reply};

pub async fn login(
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

pub async fn register(
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
