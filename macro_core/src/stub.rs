use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Expr};

pub fn stub(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let return_expr = syn::parse2::<Expr>(attr).unwrap();
    let mut input_fn = syn::parse2::<ItemFn>(item).unwrap();

    // Replace the function body with a return statement
    input_fn.block = syn::parse_quote!({
        return #return_expr;
    });

    // Generate the new function with the modified body
    let output = quote! {
        #input_fn
    };

    // Return the generated tokens as a TokenStream
    output
}