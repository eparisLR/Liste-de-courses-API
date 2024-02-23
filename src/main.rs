use axum::{
    routing::get,
    Router,
};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use ldc_api::controller::recipes::get_recipes;
use utoipa_swagger_ui::SwaggerUi;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().unwrap();
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "example_diesel_async_postgres=debug".into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

    let db_url = env::var("DATABASE_URL").unwrap();

    // set up connection pool
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/recipes/", get(get_recipes))
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}