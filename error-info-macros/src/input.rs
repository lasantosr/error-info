use darling::{
    ast::{Data, Fields},
    util::SpannedValue,
    FromDeriveInput, FromField, FromVariant,
};

#[derive(FromDeriveInput)]
#[darling(supports(enum_named, enum_unit))]
pub(super) struct ErrorInfoOpts {
    /// The identifier of the passed-in type
    pub ident: syn::Ident,
    /// The generics of the passed-in type
    pub generics: syn::Generics,
    /// The body of the passed-in type
    pub data: Data<VariantReceiver, FieldReceiver>,
}

#[derive(FromField, Clone)]
pub(super) struct FieldReceiver {
    /// The identifier of the passed-in field, or [None] for tuple fields
    pub ident: Option<syn::Ident>,
    /// The visibility of the passed-in field
    pub vis: syn::Visibility,
    /// The type of the passed-in field
    pub ty: syn::Type,
}
macro_field_utils::field_info!(FieldReceiver);

#[derive(FromVariant, Clone)]
#[darling(attributes(error))]
pub(super) struct VariantReceiver {
    /// The identifier of the passed-in variant
    pub ident: syn::Ident,
    /// For a variant such as `Example = 2`, the `2`
    pub discriminant: Option<syn::Expr>,
    /// The fields associated with the variant
    pub fields: Fields<FieldReceiver>,

    /// The status code
    pub status: syn::Expr,
    /// The error message
    pub message: SpannedValue<String>,
}
macro_field_utils::variant_info!(VariantReceiver, FieldReceiver);
