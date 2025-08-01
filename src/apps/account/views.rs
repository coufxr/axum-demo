use super::schemas::AccountListResp;
use crate::project::state::StateExt;
use axum::{Extension, Json};
use entity::{account, prelude::*};
use sea_orm::{EntityTrait, QueryOrder, QuerySelect};
use serde_json::{Value, json};

pub async fn account_list(Extension(state): StateExt) -> Json<Value> {
    let data: Vec<AccountListResp> = Account::find()
        .select_only()
        .columns([
            account::Column::Uid,
            account::Column::Phone,
            account::Column::IsActive,
            account::Column::LastLoginAt,
        ])
        .order_by_desc(account::Column::Id)
        .into_model()
        .all(&state.db)
        .await
        .expect("查询失败!");

    Json(json!(&data))
}
