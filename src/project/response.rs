use super::err::AppError;
use axum::Json;
use serde::Serialize;

pub type AppResult<T> = Result<Json<ApiResponse<T>>, AppError>;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> AppResult<T> {
        let res = Self {
            code: 0,
            msg: "success".into(),
            data: Some(data),
        };
        Ok(Json(res))
    }

    pub fn err(code: i32, msg: &str) -> AppResult<T> {
        let res = Self {
            code,
            msg: msg.into(),
            data: None,
        };

        Ok(Json(res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Serialize)]
    struct TestData {
        id: u32,
        name: String,
    }

    #[test]
    fn test_api_response_ok() {
        let test_data = TestData {
            id: 1,
            name: "test".to_string(),
        };

        let result = ApiResponse::ok(test_data);
        assert!(result.is_ok());

        let json_response = result.unwrap();
        assert_eq!(json_response.0.code, 0);
        assert_eq!(json_response.0.msg, "success");
        assert!(json_response.0.data.is_some());
    }

    #[test]
    fn test_api_response_err() {
        let result: AppResult<TestData> = ApiResponse::err(404, "not found");
        assert!(result.is_ok()); // 注意：AppResult 是 Ok(Json<ApiResponse>)，不是 Err

        let json_response = result.unwrap();
        assert_eq!(json_response.0.code, 404);
        assert_eq!(json_response.0.msg, "not found");
        assert!(json_response.0.data.is_none());
    }

    #[test]
    fn test_api_response_with_different_error_codes() {
        let error_cases = vec![
            (400, "bad request"),
            (401, "unauthorized"),
            (500, "internal server error"),
        ];

        for (code, msg) in error_cases {
            let result: AppResult<TestData> = ApiResponse::err(code, msg);
            assert!(result.is_ok());

            let json_response = result.unwrap();
            assert_eq!(json_response.0.code, code);
            assert_eq!(json_response.0.msg, msg);
            assert!(json_response.0.data.is_none());
        }
    }
}
