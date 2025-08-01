use axum::Extension;
use sea_orm::DbConn;
use std::sync::Arc;

pub struct AppState {
    pub db: DbConn,
}

pub type SharedAppState = Arc<AppState>;
pub type StateExt = Extension<SharedAppState>;
