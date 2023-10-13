mod error_info;
mod input;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

/// Derives the `ErrorInfo` trait for an enum.
/// It's **highly recommended** to include a prefix on each variant, as the name is used to generate the code, which is
/// ofter required to be unique.
///
/// Each variant must provide an status and a message, which can use variant's fields.
///
/// If `summary` feature is enabled, it requires the `linkme` crate to be available.
///
/// ## Examples
/// ``` ignore
/// #[derive(Debug, ErrorInfo)]
/// #[allow(clippy::enum_variant_names)]
/// pub enum CustomErrorCode {
///     #[error(status = StatusCode::BAD_REQUEST, message = "Bad request: missing '{field}' field")]
///     BadRequest { field: String },
///     #[error(status = StatusCode::INTERNAL_SERVER_ERROR, message = "Internal server error")]
///     InternalServerError,
/// }
/// ```
#[proc_macro_error]
#[proc_macro_derive(ErrorInfo, attributes(error))]
pub fn error_info(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    error_info::r#impl(input).into()
}
