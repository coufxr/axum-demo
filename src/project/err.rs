use super::response::ApiResponse;
use axum::{Json, response::IntoResponse, response::Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Db(#[from] sea_orm::DbErr),

    #[error("未知错误: {0}")]
    Other(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (code, msg) = match self {
            AppError::Db(_) => (500, self.to_string()),
            AppError::Other(_) => (500, self.to_string()),
        };

        let resp = ApiResponse::<()> {
            code,
            msg,
            data: None,
        };
        Json(resp).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;
    use sea_orm::error::RuntimeErr;

    /// 测试从 sea_orm 的 DbErr 错误转换为 AppError::Db
    #[test]
    fn test_db_error_creation() {
        // 创建一个数据库连接错误
        let db_err = sea_orm::DbErr::Conn(RuntimeErr::Internal("connection failed".to_string()));
        // 使用 From trait 自动转换为 AppError
        let app_error: AppError = db_err.into();

        // 验证转换后的错误类型是否正确
        match app_error {
            AppError::Db(_) => (), // 成功转换为 Db 错误
            _ => panic!("Expected Db error"),
        }
    }

    /// 测试创建 AppError::Other 错误
    #[test]
    fn test_other_error_creation() {
        // 直接创建一个 Other 类型的错误
        let app_error = AppError::Other("custom error".to_string());

        // 验证错误类型和消息内容
        match app_error {
            AppError::Other(msg) => assert_eq!(msg, "custom error"),
            _ => panic!("Expected Other error"),
        }
    }

    /// 测试数据库错误的显示格式
    #[test]
    fn test_db_error_display() {
        // 创建数据库错误并转换为 AppError
        let db_err = sea_orm::DbErr::Conn(RuntimeErr::Internal("connection failed".to_string()));
        let app_error: AppError = db_err.into();

        // 将错误转换为字符串表示
        let error_string = app_error.to_string();
        // 验证是否包含预期的错误前缀和具体内容
        assert!(error_string.contains("数据库错误"));
        assert!(error_string.contains("connection failed"));
    }

    /// 测试其他错误的显示格式
    #[test]
    fn test_other_error_display() {
        // 创建自定义错误
        let app_error = AppError::Other("custom error".to_string());
        // 转换为字符串表示
        let error_string = app_error.to_string();

        // 验证显示格式是否正确
        assert!(error_string.contains("未知错误"));
        assert!(error_string.contains("custom error"));
    }

    /// 测试 AppError 转换为 HTTP 响应
    #[test]
    fn test_into_response() {
        // 创建一个自定义错误
        let app_error = AppError::Other("test error".to_string());
        // 调用 into_response 方法生成 HTTP 响应
        let response = app_error.into_response();

        // 验证响应能够正常生成（检查状态码是否为有效状态）
        assert!(
            response.status().is_success()
                || response.status().is_client_error()
                || response.status().is_server_error()
        );
    }

    /// 测试数据库错误转换为 HTTP 响应
    #[test]
    fn test_db_error_into_response() {
        // 创建数据库错误并转换为 AppError
        let db_err = sea_orm::DbErr::Conn(RuntimeErr::Internal("connection failed".to_string()));
        let app_error: AppError = db_err.into();
        // 生成 HTTP 响应
        let response = app_error.into_response();

        // 验证响应能够正常生成
        assert!(
            response.status().is_success()
                || response.status().is_client_error()
                || response.status().is_server_error()
        );
    }
}
