use backend::database::{Credentials, Database};
use std::env;
use warp::hyper::{header, Method};
use warp::{Filter, Rejection, Reply};

async fn test(num: i32) -> anyhow::Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&num))
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

    let _db = Database::connect(credentials).await?;

    let task = warp::path("test")
        .and(warp::path::param::<i32>())
        .and_then(test);

    let api_routes = warp::path("api").and(task);

    println!("Starting warp.");

    Ok(warp::serve(api_routes).run(([127, 0, 0, 1], 8080)).await)
}
