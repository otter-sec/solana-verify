#[cfg(feature = "anchor")]
use {anyhow::Result, proc_macro2::TokenStream, quote::quote, syn::ItemFn};

#[cfg(feature = "anchor")]
use crate::program::remove_verify_ignore_statements;

#[cfg(feature = "anchor")]
pub fn access_control(args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let mut args = args.to_string();
    args.retain(|c| !c.is_whitespace());
    let access_control: Vec<proc_macro2::TokenStream> = args
        .split(')')
        .filter(|ac| !ac.is_empty())
        .map(|ac| format!("{ac})")) // Put back on the split char.
        .map(|ac| format!("{ac}?;")) // Add `?;` syntax.
        .map(|ac| ac.parse().unwrap())
        .collect();

    let Ok(mut item_fn) = syn::parse2::<ItemFn>(input) else {
        panic!("use #[access_control] on a function")
    };

    remove_verify_ignore_statements(&mut item_fn);

    let fn_attrs = item_fn.attrs;
    let fn_vis = item_fn.vis;
    let fn_sig = item_fn.sig;
    let fn_block = item_fn.block;

    let fn_stmts = fn_block.stmts;

    Ok(quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {

            #(#access_control)*

            #(#fn_stmts)*
        }
    })
}
