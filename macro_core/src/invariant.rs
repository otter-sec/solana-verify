use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, ItemStruct};

pub fn invariant(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let item = syn::parse2::<ItemStruct>(item)?;
    #[allow(clippy::redundant_clone)]
    let ident = item.ident.clone();
    let res = match syn::parse2::<Expr>(attr) {
        Ok(attr) => quote! {
            #item

            impl #ident {
                pub fn _check_invariant(&self) -> bool {
                    #attr
                }
            }
        },
        Err(_) => quote! {
            #item

            impl #ident {
                pub fn _check_invariant(&self) -> bool {
                    true
                }
            }
        },
    };
    Ok(res)
}

pub fn transition_invariant(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let item = syn::parse2::<ItemStruct>(item)?;
    #[allow(clippy::redundant_clone)]
    let ident = item.ident.clone();
    let res = match syn::parse2::<Expr>(attr) {
        Ok(attr) => quote! {
            #item

            impl #ident {
                pub fn _check_transition_invariant<T>(&self, old: T) -> bool
                where T: Deref<Target = Self>, {
                    #attr
                }
            }
        },
        Err(_) => quote! {
            #item

            impl #ident {
                pub fn _check_transition_invariant<T>(&self, old: T) -> bool
                where T: Deref<Target = Self>, {
                    true
                }
            }
        },
    };
    Ok(res)
}
