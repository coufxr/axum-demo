mod views;

use axum::Router;
use axum::routing::get;

use views::ping;

pub fn common_routes() -> Router {
    Router::new().route("/ping", get(ping))
}
