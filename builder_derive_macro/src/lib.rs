use proc_macro::TokenStream;
use quote::{format_ident, quote};

type TokenStream2 = proc_macro2::TokenStream;

#[proc_macro_derive(Setters)]
pub fn create_setters(input: TokenStream) -> TokenStream {
    let syntax_tree: syn::DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &syntax_tree.ident;

    let fields = match get_struct_fields(&syntax_tree) {
        Ok(parsed_fields) => parsed_fields,
        Err(compile_error) => {
            return compile_error;
        }
    };

    let functions: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_name = if let Option::Some(name) = field.ident.clone() {
                name
            } else {
                panic!()
            };
            let field_type = field.ty.clone();
            let function_name = format_ident!("with_{}", field_name);
            return quote! {
                pub fn #function_name(mut self, #field_name: #field_type) -> Self {
                    self.#field_name = #field_name;
                    return self;
                }
            };
        })
        .collect();

    let new_args: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_name = if let Option::Some(name) = field.ident.clone() {
                name
            } else {
                panic!()
            };
            let field_type = field.ty.clone();
            return quote! {
                #field_name: #field_type,
            };
        })
        .collect();

    let constructor_args: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_name = if let Option::Some(name) = field.ident.clone() {
                name
            } else {
                panic!()
            };
            return quote! {
                #field_name,
            };
        })
        .collect();

    let generated = quote! {
        impl #name {
            pub fn new(
                #(#new_args)*
            ) -> Self {
                return Self {
                    #(#constructor_args)*
                };
            }

            #(#functions)*
        }
    };

    return generated.into();
}

#[proc_macro_derive(Finalize, attributes(bounds))]
pub fn create_finalize(input: TokenStream) -> TokenStream {
    let syntax_tree: syn::DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);

    let struct_name = &syntax_tree.ident;

    let fields = match get_struct_fields(&syntax_tree) {
        Ok(parsed_fields) => parsed_fields,
        Err(compile_error) => {
            return compile_error;
        }
    };

    let validators: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_name = if let Option::Some(name) = &field.ident {
                name
            } else {
                panic!()
            };
            let field_type = &field.ty;
            let function_name = format_ident!("validate_{}", field_name);
            let bounds_attributes_results: &Vec<_> = &field
                .attrs
                .iter()
                .filter(|attr| attr.path().is_ident("bounds"))
                .map(|attr| {
                    let parsed_bounds = attr.parse_args_with(
                        syn::punctuated::Punctuated::<syn::Lit, syn::Token![,]>::parse_terminated,
                    );
                    let bounds = match parsed_bounds {
                        Ok(parsed) => parsed,
                        Err(_parsing_error) => {
                            let error: TokenStream = syn::Error::new_spanned(
                                &syntax_tree,
                                "Invalid syntax for bounds attribute",
                            )
                            .to_compile_error()
                            .into();
                            return error;
                        }
                    };
                    println!("{}", bounds.len());
                    println!("{:?}", bounds);
                    if bounds.len() != 2 {
                        println!("{}", bounds.len());
                        return syn::Error::new_spanned(
                            &syntax_tree,
                            format!(
                                "Expected two values for lower and upper bounds, got {}",
                                bounds.len()
                            ),
                        )
                        .to_compile_error()
                        .into();
                    }
                    return quote! {}.into();
                })
                .collect();
            // .collect();
            return quote! {
                // #(#bounds_attributes)*
                //
                // pub fn #function_name(mut self, #field_name: #field_type) -> Self {
                //     self.#field_name = #field_name;
                //     return self;
                // }
            };
        })
        .collect();

    let generated = quote! {
        // pub fn validate() ->
    };
    return generated.into();
}

type ParsedFields = syn::punctuated::Punctuated<syn::Field, syn::token::Comma>;
fn get_struct_fields(syntax_tree: &syn::DeriveInput) -> Result<ParsedFields, TokenStream> {
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_derive_macro_file() {}
}
