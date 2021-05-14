use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

pub fn impl_readable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = match &ast.data {
        syn::Data::Struct(s) => match &s.fields {
            syn::Fields::Named(f) => {
                let recurse = f.named.iter().map(|_if| {
                    let fname = &_if.ident;
                    let ftype = &_if.ty;
                    quote_spanned! { _if.span() =>
                        #fname: <#ftype>::read_from(_buf)?
                    }
                });
                quote! {
                    #(#recurse),*
                }
            }
            syn::Fields::Unnamed(f) => {
                let recurse = f.unnamed.iter().map(|_if| {
                    let ftype = &_if.ty;
                    quote_spanned! { _if.span() =>
                        <#ftype>::read_from(_buf)?
                    }
                });
                quote! {
                    #(#recurse),*
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
        impl #tetsu_path::serialization::Readable for #name {
            #[inline]
            fn read_from<__T: ::std::io::Read>(_buf: &mut __T) -> TetsuResult<Self> {
                Ok(Self {
                    #fields
                })
            }
        }
    };

    gen.into()
}
