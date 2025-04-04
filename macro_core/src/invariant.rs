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

            impl ::shared::Invariant<anchor_lang::prelude::AccountInfo<'static>> for #ident {
                fn check_invariant(&self) -> bool {
                    #attr
                }

                fn check_transition_invariant(&self, _: &dyn ::shared::Invariant<anchor_lang::prelude::AccountInfo<'static>>, _: &[anchor_lang::prelude::AccountInfo]) -> bool {
                    todo!()
                }
            }
        },
        Err(_) => quote! {
            #item

            impl::shared::Invariant<anchor_lang::prelude::AccountInfo<'static>> for #ident {
                fn check_invariant(&self) -> bool {
                    true
                }

                fn check_transition_invariant(&self, _: &dyn ::shared::Invariant<anchor_lang::prelude::AccountInfo<'static>>, _: &[anchor_lang::prelude::AccountInfo]) -> bool {
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
                pub fn _check_transition_invariant<T>(&self, old: T, remaining_accounts: &[anchor_lang::prelude::AccountInfo]) -> bool
                where T: Deref<Target = Self>, {
                    #attr
                }
            }
        },
        Err(_) => quote! {
            #item

            impl #ident {
                pub fn _check_transition_invariant<T>(&self, old: T, remaining_accounts: &[anchor_lang::prelude::AccountInfo]) -> bool
                where T: Deref<Target = Self>, {
                    true
                }
            }
        },
    };
    Ok(res)
}
