use backend::models;
use backend::routes;
use backend::util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state: models::State = models::new_state().await?;

    let routes = routes::routes(state.clone());
    println!("Starting warp.");

    util::insert_exercises(state.clone()).await;

    Ok(warp::serve(routes).run(([127, 0, 0, 1], 8080)).await)
}
