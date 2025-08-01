mod schemas;
mod views;

use axum::Router;
use axum::routing::get;

use views::account_list;

pub fn account_routes() -> Router {
    Router::new().route("/", get(account_list))
}
