use proc_macro::TokenStream;
use quote::{format_ident, quote};

use crate::get_struct_fields::get_struct_fields;

pub fn create_getters_and_setters(input: TokenStream) -> TokenStream {
    let syntax_tree: syn::DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &syntax_tree.ident;

    let fields = match get_struct_fields(&syntax_tree) {
        Ok(parsed_fields) => parsed_fields,
        Err(compile_error) => {
            return compile_error;
        }
    };

    let (getter_functions, setter_functions, new_args, constructor_args): (
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
    ) = fields
        .clone()
        .into_iter()
        .map(|field| {
            let skip: bool = field
                .attrs
                .into_iter()
                .find(|attr| attr.path().is_ident("skip_setter"))
                .is_some();
            if skip {
                return (quote! {}, quote! {}, quote! {}, quote! {});
            }
            let field_name = field.ident.unwrap();
            let field_type = field.ty;
            let getter_function_name = format_ident!("get_{}", field_name);
            let getter_functions = quote! {
                pub fn #getter_function_name(&self) -> &#field_type {
                    return &self.#field_name;
                }
            };
            let setter_function_name = format_ident!("with_{}", field_name);
            let setter_functions = quote! {
                pub fn #setter_function_name(mut self, #field_name: #field_type) -> Self {
                    self.#field_name = #field_name;
                    return self;
                }
            };
            let new_args = quote! {
                #field_name: #field_type,
            };
            let constructor_args = quote! {
                #field_name,
            };
            return (
                getter_functions,
                setter_functions,
                new_args,
                constructor_args,
            );
        })
        .collect();

    let skip: bool = fields.into_iter().any(|field| {
        field
            .attrs
            .into_iter()
            .find(|attr| attr.path().is_ident("skip_setter"))
            .is_some()
    });
    let new_function = if skip {
        quote! {}
    } else {
        quote! {
            pub fn new(
                #(#new_args)*
            ) -> Self {
                return Self {
                    #(#constructor_args)*
                };
            }
        }
    };

    let generated = quote! {
        impl #name {
            #new_function

            #(#getter_functions)*

            #(#setter_functions)*
        }
    };

    return generated.into();
}
