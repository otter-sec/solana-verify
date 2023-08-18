use anchor_syn::AccountsStruct;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub fn derive_init_space(item: TokenStream) -> Result<TokenStream> {
    let item = item.to_token_stream();
    let val = syn::parse2::<AccountsStruct>(item)?;
    let ident = val.ident.clone();
    let generics = val.generics.clone();

    let res = quote! {
        impl #generics Space for #ident #generics {
            const INIT_SPACE: usize = 0;
        }
    };

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_init_space() {
        let input = quote! {
            #[derive(Clone, Default)]
            pub struct MyStruct {
                pub my_field: u64,
            }
        };

        let expected = quote! {
            impl Space for MyStruct {
                const INIT_SPACE: usize = 0;
            }
        };

        let res = derive_init_space(input).unwrap();
        assert_eq!(res.to_string(), expected.to_string());
    }
}
