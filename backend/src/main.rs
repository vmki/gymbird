use backend::database::{Credentials, Database};
use serde_json::to_string;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::hyper::{header, Method};
use warp::{Filter, Rejection, Reply};

async fn get_user(uuid: String, db: Arc<Mutex<Database>>) -> anyhow::Result<impl Reply, Rejection> {
    let db = db.lock().await;
    let user = db.get_user(uuid).await.unwrap();

    Ok(warp::reply::json(&serde_json::to_string(&user).unwrap()))
}

fn get_var(var: &str) -> String {
    env::var(var).expect(&format!("Couldn't find environment variable ${}.", var))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let credentials = Credentials {
        username: &get_var("DB_USERNAME"),
        db_name: &get_var("DB_NAME"),
        host: &get_var("DB_HOST"),
    };

    println!("{:#?}", credentials);

    let db = Arc::new(Mutex::new(Database::connect(credentials).await?));

    let db = warp::any().map(move || db.clone());

    let task = warp::path("test")
        .and(warp::path::param::<String>())
        .and(db.clone())
        .and_then(get_user);

    let api_routes = warp::path("api").and(task);

    println!("Starting warp.");

    Ok(warp::serve(api_routes).run(([127, 0, 0, 1], 8080)).await)
}
