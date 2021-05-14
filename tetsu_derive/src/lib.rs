//! Provides derive macros for tetsu's `Writable` and Readable traits.

use proc_macro::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::Ident;

mod readable;
mod writable;

/// Get the path to the tetsu crate.
pub(crate) fn get_tetsu_path() -> proc_macro2::TokenStream {
    match crate_name("tetsu").expect("`tetsu` is present in `Cargo.toml`") {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, proc_macro2::Span::call_site());
            quote!( #ident )
        }
    }
}

#[proc_macro_derive(ReadableStruct)]
pub fn readable_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    readable::impl_readable(&ast)
}

#[proc_macro_derive(WritableStruct)]
pub fn writable_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    writable::impl_writable(&ast)
}
