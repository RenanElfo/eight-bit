use proc_macro::TokenStream;

type ParsedFields = syn::punctuated::Punctuated<syn::Field, syn::token::Comma>;
pub fn get_struct_fields(syntax_tree: &syn::DeriveInput) -> Result<ParsedFields, TokenStream> {
    let fields = if let syn::Data::Struct(data_struct) = syntax_tree.data.clone() {
        if let syn::Fields::Named(fields_named) = data_struct.fields {
            fields_named.named
        } else {
            return Err(syn::Error::new_spanned(
                syntax_tree,
                "Setters derive macro can only be used for structs with named fields",
            )
            .to_compile_error()
            .into());
        }
    } else {
        return Err(syn::Error::new_spanned(
            syntax_tree,
            "Setters derive macro can only be used for structs",
        )
        .to_compile_error()
        .into());
    };
    return Ok(fields);
}
