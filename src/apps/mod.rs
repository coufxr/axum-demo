use axum::Router;

mod account;
mod common;

use account::account_routes;
use common::common_routes;

pub fn v1_routes() -> Router {
    Router::new()
        .nest("/common", common_routes())
        .nest("/account", account_routes())
}
