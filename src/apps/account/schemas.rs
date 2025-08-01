use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use sea_orm::prelude::Uuid;
use serde::Serialize;

#[derive(Debug, Serialize, FromQueryResult)]
pub struct AccountListResp {
    uid: Uuid,
    phone: String,
    is_active: i16,
    last_login_at: Option<NaiveDateTime>,
}
