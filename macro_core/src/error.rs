use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemEnum;

pub fn error_code(_args: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let val = syn::parse2::<ItemEnum>(item.clone())?;
    let ident = val.ident;
    let generics = val.generics;

    let res = quote! {
        #[derive(Debug, thiserror::Error)]
        #item

        impl #generics std::fmt::Display for #ident #generics {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                <Self as std::fmt::Debug>::fmt(self, f)
            }
        }

        impl From<#ident> for anchor_lang::prelude::Error {
            fn from(value: #ident) -> Self {
                anchor_lang::prelude::Error::CustomError(value.to_string())
            }
        }
    };
    Ok(res)
}
