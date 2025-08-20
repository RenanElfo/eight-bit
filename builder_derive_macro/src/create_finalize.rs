use itertools::Itertools;
use proc_macro::TokenStream;
use quote::{format_ident, quote};

use crate::get_struct_fields;

type TokenStream2 = proc_macro2::TokenStream;

pub fn create_finalize(input: TokenStream) -> TokenStream {
    let syntax_tree: syn::DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);

    let struct_name = &syntax_tree.ident;

    let fields = match get_struct_fields::get_struct_fields(&syntax_tree) {
        Ok(parsed_fields) => parsed_fields,
        Err(compile_error) => {
            return compile_error;
        }
    };

    let validators: Vec<Result<TokenStream2, syn::Error>> = fields
        .into_iter()
        .map(|field| {
            let field_name = field
                .ident
                .expect("Should have returned with an error for fields without names");
            let field_type = field.ty;
            let function_name = format_ident!("validate_{}", field_name);
            let bounds_attributes_results: Vec<Result<TokenStream2, syn::Error>> = field
                .attrs
                .into_iter()
                .filter(|attr| attr.path().is_ident("bounds"))
                .map(|attr| {
                    let (lower_bound, upper_bound) = parse_attr_args(attr)?;
                    return Ok(quote! {}.into());
                })
                .collect();

            let (bounds_attributes, bounds_attributes_errors): (Vec<_>, Vec<_>) =
                bounds_attributes_results.into_iter().partition_result();

            let errors = bounds_attributes_errors
                .into_iter()
                .reduce(|mut accumulator, error| {
                    accumulator.combine(error);
                    return accumulator;
                });
            if let Option::Some(error) = errors {
                return Err(error);
            }

            return Ok(quote! {});
        })
        .collect();

    let (validators, validators_errors): (Vec<_>, Vec<_>) =
        validators.into_iter().partition_result();

    let errors = validators_errors
        .into_iter()
        .reduce(|mut accumulator, error| {
            accumulator.combine(error);
            return accumulator;
        });
    if let Option::Some(error) = errors {
        return error.into_compile_error().into();
    }

    let generated = quote! {
        // pub fn validate() ->
    };
    return generated.into();
}

fn parse_attr_args(attr: syn::Attribute) -> Result<(syn::Lit, syn::Lit), syn::Error> {
    let parsed_bounds = attr
        .parse_args_with(syn::punctuated::Punctuated::<syn::Lit, syn::Token![,]>::parse_terminated);
    let bounds = match parsed_bounds {
        Ok(parsed) => parsed,
        Err(_parsing_error) => {
            return Err(syn::Error::new_spanned(
                &attr,
                "Invalid syntax for bounds attribute",
            ));
        }
    };
    if bounds.len() != 2 {
        return Err(syn::Error::new_spanned(
            &attr,
            format!(
                "Expected 2 values for lower and upper bounds, got {}",
                bounds.len()
            ),
        ));
    }
    let lower_bound = bounds[0].clone();
    let upper_bound = bounds[1].clone();
    return Ok((lower_bound, upper_bound));
}
