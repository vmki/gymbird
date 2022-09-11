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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state: models::State = models::new_state().await?;

    let state = warp::any().map(move || state.clone());

    let login_path = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and(state.clone())
        .and_then(login);

    let api_routes = warp::path("api").and(login_path).with(
        warp::cors()
            .allow_any_origin()
            .allow_methods(&[Method::POST, Method::GET, Method::PUT])
            .allow_header("content-type"),
    );

    println!("Starting warp.");

    Ok(warp::serve(api_routes).run(([127, 0, 0, 1], 8080)).await)
}

mod models {
    use backend::database::{Credentials, Database};
    use serde::{Deserialize, Serialize};
    use std::env;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    pub type State = Arc<Mutex<Database>>;

    fn get_var(var: &str) -> String {
        env::var(var).expect(&format!("Couldn't find environment variable ${}.", var))
    }

    pub async fn new_state() -> anyhow::Result<State> {
        let credentials = Credentials {
            username: &get_var("DB_USERNAME"),
            db_name: &get_var("DB_NAME"),
            host: &get_var("DB_HOST"),
        };

        println!("{:#?}", credentials);

        Ok(Arc::new(Mutex::new(Database::connect(credentials).await?)))
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LoginParameters {
        pub email: String,
        pub password: String,
    }
}
