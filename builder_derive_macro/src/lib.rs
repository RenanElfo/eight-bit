use proc_macro::TokenStream;

mod create_getters_and_setters;
mod create_finalize;
mod get_struct_fields;

#[proc_macro_derive(Setters, attributes(skip_setter))]
pub fn create_setters_proc_macro(input: TokenStream) -> TokenStream {
    create_getters_and_setters::create_getters_and_setters(input)
}

#[proc_macro_derive(Finalize, attributes(bounds))]
pub fn create_finalize(input: TokenStream) -> TokenStream {
    create_finalize::create_finalize(input)
}
