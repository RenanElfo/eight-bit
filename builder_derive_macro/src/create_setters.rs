use proc_macro::TokenStream;
use quote::{format_ident, quote};

use crate::get_struct_fields::get_struct_fields;

pub fn create_setters(input: TokenStream) -> TokenStream {
    let syntax_tree: syn::DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &syntax_tree.ident;

    let fields = match get_struct_fields(&syntax_tree) {
        Ok(parsed_fields) => parsed_fields,
        Err(compile_error) => {
            return compile_error;
        }
    };

    let (setter_functions, new_args, constructor_args): (Vec<_>, Vec<_>, Vec<_>) = fields
        .into_iter()
        .map(|field| {
            let field_name = field.ident.unwrap();
            let field_type = field.ty;
            let function_name = format_ident!("with_{}", field_name);
            let setter_functions = quote! {
                pub fn #function_name(mut self, #field_name: #field_type) -> Self {
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
            return (setter_functions, new_args, constructor_args);
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

            #(#setter_functions)*
        }
    };

    return generated.into();
}
