# ErrorInfo

Centralized error information ready for internationalization.

The main export for this crate is the trait and derive macro `ErrorInfo` :

```rs
#[derive(ErrorInfo)]
pub enum CustomErrorCode {
    #[error(status = StatusCode::BAD_REQUEST, message = "Bad request: {reason}")]
    BadRequest { reason: &'static str },
    #[error(status = StatusCode::INTERNAL_SERVER_ERROR, message = "Internal server error")]
    InternalServerError,
}
```

Then you should be able to retrieve error info:

```rs
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
```

Or collect every error declared in any crate (with `summary` feature enabled), which simplifies the error management for
web services:

```rs
let summary = error_info::summary();
fs::write(
    "./assets/error-codes.json",
    serde_json::to_string_pretty(&summary)?,
)?

// Writes:
//
// [
//   {
//     "status": 400,
//     "code": "BAD_REQUEST",
//     "raw_message": "Bad request: {reason}",
//   },
//   {
//     "status": 500,
//     "code": "INTERNAL_SERVER_ERROR",
//     "raw_message": "Internal server error",
//   }
// ]

```

You can also export that data using your preferred localization format and share it with the frontend team.
