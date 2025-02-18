use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Expr};

pub fn stub(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let return_expr = syn::parse2::<Expr>(attr).unwrap();
    let mut input_fn = syn::parse2::<ItemFn>(item).unwrap();

    // Check if the return type is a Result
    let is_result_type = if let syn::ReturnType::Type(_, ty) = &input_fn.sig.output {
        if let syn::Type::Path(type_path) = &**ty {
            type_path.path.segments.last().unwrap().ident == "Result"
        } else {
            false
        }
    } else {
        false
    };

    // Replace the function body with a return statement
    input_fn.block = if is_result_type {
        syn::parse_quote!({
            return Ok(#return_expr);
        })
    } else {
        syn::parse_quote!({
            return #return_expr;
        })
    };

    // Generate the new function with the modified body
    let output = quote! {
        #input_fn
    };

    // Return the generated tokens as a TokenStream
    output
}