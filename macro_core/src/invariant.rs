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
                fn as_any(&self) -> &dyn std::any::Any {
                    self as _
                }

                fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                    self as _
                }

                fn check_invariant(&self) {
                    kani::assert(#attr, concat!(concat!("failed ", stringify!(#ident)), "invariant ", stringify!(#attr)));
                }

                fn check_transition_invariant(&self, other: &dyn ::shared::Invariant<anchor_lang::prelude::AccountInfo<'static>>, remaining_accounts: &[anchor_lang::prelude::AccountInfo]) {
                    let before = other.as_any().downcast_ref::<#ident>().unwrap();
                    self._check_transition_invariant(before, remaining_accounts);
                }
            }
        },
        Err(e) => { panic!("Error parsing invariant {:?}", e) },
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
                pub fn _check_transition_invariant<T>(&self, old: T, remaining_accounts: &[anchor_lang::prelude::AccountInfo])
                where T: Deref<Target = Self>, {
                    kani::assert(#attr, concat!(concat!("failed ", stringify!(#ident)), "transition invariant ", stringify!(#attr)));
                }
            }
        },
        Err(e) => {  panic!("Error parsing transition invariant: {:?}", e) },
    };
    Ok(res)
}
