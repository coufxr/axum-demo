use axum::Router;

mod common;

use common::common_routes;

pub fn v1_routes() -> Router {
    Router::new().nest("/common", common_routes())
}
