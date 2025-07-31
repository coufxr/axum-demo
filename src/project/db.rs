use sea_orm::{ConnectOptions, Database, DbConn};
use std::time::Duration;

pub async fn init() -> anyhow::Result<DbConn> {
    let mut opt = ConnectOptions::new("postgres://root:root@127.0.0.1:5432/nguc");

    opt.min_connections(5)
        .max_connections(10)
        .idle_timeout(Duration::from_secs(10))
        .max_lifetime(Duration::from_secs(30))
        .sqlx_logging(true);

    let conn = Database::connect(opt).await?;

    Ok(conn)
}
