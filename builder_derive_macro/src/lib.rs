use proc_macro::TokenStream;
use quote::{format_ident, quote};

#[proc_macro_derive(Setters)]
pub fn add(input: TokenStream) -> TokenStream {
    let syntax_tree: syn::DeriveInput = syn::parse(input).unwrap();

    return impl_setter_macro(&syntax_tree);
}

fn impl_setter_macro(syntax_tree: &syn::DeriveInput) -> TokenStream {
    let name = &syntax_tree.ident;
    let fields = if let syn::Data::Struct(data_struct) = syntax_tree.data.clone() {
        if let syn::Fields::Named(fields_named) = data_struct.fields {
            fields_named.named
        } else {
            panic!();
        }
    } else {
        panic!();
    };
    let functions: Vec<_> = fields
        .iter()
        .map(|field| {
            let name = if let Option::Some(name) = field.ident.clone() {
                name
            } else {
                panic!()
            };
            let field_type = field.ty.clone();
            let function_name = format_ident!("with_{}", name);
            return quote! {
                pub fn #function_name(mut self, #name: #field_type) -> Self {
                    self.#name = #name;
                    return self;
                }
            };
        })
        .collect();
    let generated = quote! {
        impl #name {
            #(#functions)*
        }
    };
    generated.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_derive_macro_file() {}
}
