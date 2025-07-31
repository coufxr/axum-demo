mod apps;
mod project;

use axum::{Router, routing::get};

use migration::MigratorTrait;
use project::db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // init
    let db = db::init().await?;
    migration::Migrator::up(&db, None)
        .await
        .expect("执行迁移失败");

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" })) // 根路由
        .nest("/api/v1", apps::v1_routes()); // 路由嵌套方式

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Listening on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;
    Ok(())
}
