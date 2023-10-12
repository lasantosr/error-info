use darling::{ast::Data, FromDeriveInput};
use heck::ToShoutySnakeCase;
use macro_field_utils::{VariantsCollector, VariantsHelper};
use once_cell::sync::Lazy;
use proc_macro2::TokenStream;
use proc_macro_error::{abort_call_site, abort_if_dirty, emit_error};
use quote::{format_ident, quote};
use regex::Regex;
use syn::DeriveInput;

use crate::input::*;

static VARIABLES_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{([^}]+)\}").unwrap());

pub(crate) fn r#impl(input: DeriveInput) -> TokenStream {
    let crate_expr = quote!(error_info);

    // Parse input
    let opts = match ErrorInfoOpts::from_derive_input(&input) {
        Ok(o) => o,
        Err(e) => {
            return e.write_errors();
        }
    };

    // Retrieve variants
    let Data::Enum(variants) = opts.data else {
        abort_call_site!("Only enums are supported.")
    };

    // Verify messages
    for v in variants.iter() {
        // Non-empty
        if v.message.is_empty() {
            emit_error!(v.message.span(), "The message can't be empty");
        }
        let variables = VARIABLES_REGEX
            .captures_iter(&v.message)
            .map(|c| c.get(1).unwrap().as_str())
            .collect::<Vec<_>>();

        // Every variable from the message has the corresponding field
        for variable in variables.iter() {
            if !v
                .fields
                .iter()
                .any(|f| f.ident.as_ref().expect("enum_named") == variable)
            {
                emit_error!(v.message.span(), "Missing a field for the variable '{}'", variable);
            }
        }

        // Every field is being used
        for field in v.fields.iter().map(|f| f.ident.as_ref().expect("enum_named")) {
            if !variables.iter().any(|variable| field == variable) {
                emit_error!(field, "The field is not being used");
            }
        }
    }

    abort_if_dirty();

    // Base variables
    let enum_ident = opts.ident;
    let (impl_generics, ty_generics, where_clause) = opts.generics.split_for_impl();

    // Error status
    let match_status = VariantsHelper::new(&variants)
        .left_collector(VariantsCollector::variant_fields_collector(quote!(Self)))
        .right_collector(|v, _fields| {
            let status = &v.status;
            quote!(#status)
        })
        .collect();

    // Error code
    let match_code = VariantsHelper::new(&variants)
        .left_collector(VariantsCollector::variant_fields_collector(quote!(Self)))
        .right_collector(|v, _fields| {
            let code = &v.ident.to_string().to_shouty_snake_case();
            quote!(#code)
        })
        .collect();

    // Error raw message
    let match_raw_message = VariantsHelper::new(&variants)
        .left_collector(VariantsCollector::variant_fields_collector(quote!(Self)))
        .right_collector(|v, _fields| {
            let raw_message = v.message.as_ref();
            quote!(#raw_message)
        })
        .collect();

    // Error fields
    let match_fields = VariantsHelper::new(&variants)
        .left_collector(VariantsCollector::variant_fields_collector(quote!(Self)))
        .right_collector(|_v, fields| {
            if fields.is_empty() {
                quote!(Default::default())
            } else {
                let fields_expr = fields.into_vec().into_iter().map(|f| {
                    let field_ident = f.ident.as_ref().expect("enum_named");
                    let field_name = f.ident.as_ref().expect("enum_named").to_string();
                    quote!((#field_name.into(), #field_ident.to_string()))
                });
                quote!(::std::collections::HashMap::from([#( #fields_expr ),*]))
            }
        })
        .collect();

    // Variant's summary
    let summary_expr = variants.iter().map(|v| {
        let status = &v.status;
        let code = &v.ident.to_string().to_shouty_snake_case();
        let raw_message = v.message.as_ref();

        quote!(
            #crate_expr::ErrorInfoSummary {
                status: #status,
                code: #code,
                raw_message: #raw_message,
            }
        )
    });

    let enum_ident_snake = enum_ident.to_string().to_shouty_snake_case();
    let enum_ident_snake = format_ident!("{enum_ident_snake}");

    // Implement trait
    quote!(
        #[automatically_derived]
        #[allow(non_shorthand_field_patterns)]
        impl #impl_generics #crate_expr::ErrorInfo for #enum_ident #ty_generics #where_clause {
            fn status(&self) -> ::http::StatusCode {
                match self #match_status
            }

            fn code(&self) -> &'static str {
                match self #match_code
            }

            fn raw_message(&self) -> &'static str {
                match self #match_raw_message
            }

            fn fields(&self) -> std::collections::HashMap<String, String> {
                match self #match_fields
            }
        }

        #[automatically_derived]
        impl #impl_generics #enum_ident #ty_generics #where_clause {
            fn summaries() -> Vec<#crate_expr::ErrorInfoSummary> {
                vec![
                    #( #summary_expr ),*
                ]
            }
        }

        #[::linkme::distributed_slice(#crate_expr::ERROR_INFO_SUMMARY)]
        static #enum_ident_snake: fn() -> Vec<#crate_expr::ErrorInfoSummary> = #enum_ident::summaries;
    )
}
