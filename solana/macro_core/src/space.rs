use anchor_syn::{AccountField, AccountsStruct, Field, Ty};
use anyhow::Result;
use proc_macro2::{Group, Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{ExprType, ItemStruct};

fn is_field_boxed(field: &Field) -> bool {
    let Ty::Account(account_ty) = &field.ty else {
        return false;
    };
    account_ty.boxed
}

pub fn derive_init_space(item: TokenStream) -> Result<TokenStream> {
    let arg_item = syn::parse2::<ItemStruct>(item.clone())?;
    let mut arg_names: Vec<Ident> = vec![];
    let mut arg_types: Vec<syn::Type> = vec![];

    for t in arg_item.attrs {
        if t.path.to_token_stream().to_string() == "instruction" {
            let g = syn::parse2::<Group>(t.tokens)?;
            for arg in g.stream().to_string().split(',') {
                let parsed_arg = syn::parse_str::<ExprType>(arg.trim())?;
                arg_names.push(Ident::new(
                    &parsed_arg.expr.to_token_stream().to_string(),
                    Span::call_site(),
                ));
                arg_types.push(syn::parse_str::<syn::Type>(
                    &parsed_arg.ty.to_token_stream().to_string(),
                )?);
            }
        }
    }

    let item = item.to_token_stream();
    let val = syn::parse2::<AccountsStruct>(item)?;
    let ident = val.ident.clone();
    let generics = val.generics.clone();

    let fields = val
        .fields
        .iter()
        .map(|field| {
            let (boxed, ident) = match field {
                AccountField::Field(field) => (is_field_boxed(field), &field.ident),
                AccountField::CompositeField(c_field) => (false, &c_field.ident),
            };

            if boxed {
                quote! {
                    #ident: Box::new(kani::any())
                }
            } else {
                quote! {
                    #ident: kani::any()
                }
            }
        })
        .collect::<Vec<TokenStream>>();

    let res = quote! {
        impl #generics kani::Arbitrary for #ident #generics {
            fn any() -> Self {
                Self {
                    #(#fields),*
                }
            }
        }
    };

    Ok(res)
}
