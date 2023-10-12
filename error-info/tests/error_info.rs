use std::collections::HashMap;

use error_info::ErrorInfo;
use http::StatusCode;

#[derive(Debug, Clone, ErrorInfo)]
pub enum CustomErrorCode {
    #[error(status = StatusCode::BAD_REQUEST, message = "Bad request: {reason}")]
    BadRequest { reason: &'static str },
    #[error(status = StatusCode::INTERNAL_SERVER_ERROR, message = "Internal server error")]
    InternalServerError,
}

#[test]
fn test_custom_error_code() {
    let bad_request = CustomErrorCode::BadRequest {
        reason: "invalid parameter",
    };
    assert_eq!(bad_request.status(), StatusCode::BAD_REQUEST);
    assert_eq!(bad_request.code(), "BAD_REQUEST");
    assert_eq!(bad_request.raw_message(), "Bad request: {reason}");
    assert_eq!(bad_request.message(), "Bad request: invalid parameter");
    assert_eq!(
        bad_request.fields(),
        HashMap::from([("reason".into(), "invalid parameter".to_string())])
    );
}

#[test]
fn test_custom_error_code_summaries() {
    let summary = error_info::summary();
    assert_eq!(summary.len(), 2);
    assert_eq!(summary.get(0).unwrap().status, StatusCode::BAD_REQUEST);
    assert_eq!(summary.get(0).unwrap().code, "BAD_REQUEST");
    assert_eq!(summary.get(1).unwrap().status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(summary.get(1).unwrap().code, "INTERNAL_SERVER_ERROR");
}
