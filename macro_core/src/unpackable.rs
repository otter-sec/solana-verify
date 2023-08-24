use anyhow::Result;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use quote::ToTokens;
pub fn unpackable(arg: TokenStream) -> Result<TokenStream> {
    // Parse the array of types in arg
    let types = syn::parse2::<syn::ExprArray>(arg)?;
    let types = types
        .elems
        .into_iter()
        .map(|t| {
            let ty = syn::parse2::<syn::Type>(t.into_token_stream()).unwrap();
            let t = ty.to_token_stream().to_string();
            (ty, t)
        })
        .collect::<Vec<_>>();

    // For each type, emit a implementation of the Verify trait
    let mut harnesses = vec![];
    let mut inits = vec![];
    for (ty, t_str) in types.iter() {
        let t_upper = syn::Ident::new(&t_str.to_uppercase(), Span::call_site());
        let t_upper_plural = syn::Ident::new(&format!("{}S", t_upper), Span::call_site());
        let t_upper_next = syn::Ident::new(&format!("{}_NEXT", t_upper), Span::call_site());
        let t_upper_next_pack =
            syn::Ident::new(&format!("{}_NEXT_PACK", t_upper), Span::call_site());
        let harness = quote! {
            static mut #t_upper_plural: Option<Vec<#ty>> = None;
            static mut #t_upper_next: usize = 0;
            static mut #t_upper_next_pack: usize = 0;
            impl Verify for #ty {
                fn get_next() -> Self {
                    unsafe {
                        assert!(#t_upper_next < #t_upper_plural.as_ref().unwrap().len());
                        let result = #t_upper_plural.as_ref().unwrap()[#t_upper_next];
                        #t_upper_next += 1;
                        result
                    }
                }
                // TODO: We could instead just offer a way to access
                // a mut slice of the global
                fn expect_unpack(x: Self) {
                    unsafe {
                        #t_upper_plural.as_mut().unwrap().push(x);
                    }
                }
                fn num_used() -> usize {
                    unsafe {
                        #t_upper_next
                    }
                }
                fn repack(x: Self) {
                    unsafe {
                        #t_upper_plural.as_mut().unwrap()[#t_upper_next_pack] = x;
                        #t_upper_next_pack += 1;
                    }
                }
                fn get_packed(i: usize) -> Self {
                    unsafe {
                        assert!(i < #t_upper_next_pack);
                        let result = #t_upper_plural.as_ref().unwrap()[i];
                        result
                    }
                }
            }
        };
        let init = quote! {
            unsafe {
                #t_upper_plural = Some(Vec::new());
                #t_upper_next = 0;
                #t_upper_next_pack = 0;
            }
        };
        harnesses.push(harness);
        inits.push(init);
    }

    let res = quote! {
        #(#harnesses)*
        pub fn init() {
            #(#inits)*
        }
    };
    Ok(res)
}
