use std::collections::HashMap;

use http::StatusCode;

// Re-export derive macro crate
#[allow(unused_imports)]
#[macro_use]
extern crate error_info_macros;
#[doc(hidden)]
pub use error_info_macros::*;

/// Trait containing common information for an error
pub trait ErrorInfo {
    /// Retrieves the status of the error message
    fn status(&self) -> StatusCode;
    /// Retrieves the code of the error message
    fn code(&self) -> &'static str;
    /// Retrieves the raw message of the error, without variable replacements
    fn raw_message(&self) -> &'static str;
    /// Retrieves the error fields used on the message
    fn fields(&self) -> HashMap<String, String>;
    /// Builds the final error message with the variable fields interpolated
    fn message(&self) -> String {
        let mut message = self.raw_message().to_string();
        for (k, v) in self.fields() {
            message = message.replace(&format!("{{{k}}}"), &v);
        }
        message
    }
}

#[derive(Debug, serde::Serialize)]
/// Summary of an [ErrorInfo]
pub struct ErrorInfoSummary {
    /// HTTP status code
    #[serde(serialize_with = "serialize_status")]
    pub status: StatusCode,
    /// Error code
    pub code: &'static str,
    /// Raw message, it might contain fields to be substituted.
    ///
    /// For example: `Missing field {field}`
    pub raw_message: &'static str,
}

/// Array to collect all linked error info summaries, it shouldn't be used directly. Instead use `error_info::summary`
#[linkme::distributed_slice]
pub static ERROR_INFO_SUMMARY: [fn() -> Vec<ErrorInfoSummary>] = [..];

/// Retrieves a summary of every [ErrorInfo] declared, sorted by status and code.
///
/// This could be exported to provide the base i18n file for errors.
pub fn summary() -> Vec<ErrorInfoSummary> {
    let mut ret = Vec::with_capacity(ERROR_INFO_SUMMARY.len());
    for summaries_fn in ERROR_INFO_SUMMARY {
        let mut summaries = summaries_fn();
        ret.append(&mut summaries);
    }
    ret.sort_by_key(|s| (s.status, s.code));
    ret
}

fn serialize_status<S>(status: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u16(status.as_u16())
}
