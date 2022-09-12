use backend::models;
use backend::routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state: models::State = models::new_state().await?;

    let routes = routes::routes(state);
    println!("Starting warp.");

    Ok(warp::serve(routes).run(([127, 0, 0, 1], 8080)).await)
}
