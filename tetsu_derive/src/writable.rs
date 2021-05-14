use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

pub fn impl_writable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = match &ast.data {
        syn::Data::Struct(s) => match &s.fields {
            syn::Fields::Named(f) => {
                let recurse = f.named.iter().map(|_if| {
                    let fname = &_if.ident;
                    quote_spanned! { _if.span() =>
                        self.#fname.write_to(_buf)?
                    }
                });
                quote! {
                    #(#recurse;)*
                }
            }
            syn::Fields::Unnamed(f) => {
                let recurse = f.unnamed.iter().map(|_if| {
                    let fname = &_if.ident;
                    quote_spanned! { _if.span() =>
                       self.#fname.write_to(_buf)?
                    }
                });
                quote! {
                    #(#recurse;)*
                }
            }
            syn::Fields::Unit => {
                quote! {}
            }
        },
        _ => panic!("Expected struct."),
    };

    let tetsu_path = crate::get_tetsu_path();

    let gen = quote! {
        impl #tetsu_path::serialization::Writable for #name {
            #[inline]
            fn write_to<__T: ::std::io::Write>(&self, _buf: &mut __T) -> TetsuResult<()> {
                #fields
                Ok(())
            }
        }
    };

    gen.into()
}
