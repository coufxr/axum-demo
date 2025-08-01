use super::schemas::{AccountListReq, AccountListResp};
use crate::project::err::AppError;
use crate::project::response::{ApiResponse, AppResult};
use crate::project::state::StateExt;
use axum::Extension;
use axum::extract::Query;
use entity::{account, prelude::*};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

pub async fn account_list(
    Extension(state): StateExt,
    Query(query): Query<AccountListReq>,
) -> AppResult<Vec<AccountListResp>> {
    if let Some(num) = query.phone.clone() {
        let num = num.parse::<u64>();
        if num.is_err() {
            return Err(AppError::Other("请输入正确的手机号码".to_string()));
        }
    }

    let data: Vec<AccountListResp> = Account::find()
        .select_only()
        .columns([
            account::Column::Uid,
            account::Column::Phone,
            account::Column::IsActive,
            account::Column::LastLoginAt,
        ])
        .filter(
            Condition::all().add_option(
                query
                    .phone
                    .clone()
                    .map(|phone| account::Column::Phone.contains(phone)),
            ),
        )
        .order_by_desc(account::Column::Id)
        .into_model()
        .all(&state.db)
        .await?;

    if query.phone.is_some() && data.is_empty() {
        return ApiResponse::err(404, "未找到数据");
    }

    ApiResponse::ok(data)
}
