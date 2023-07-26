use anchor_syn::{AccountField, AccountsStruct, ConstraintGroup, Field, Ty};
use anyhow::Result;
use proc_macro2::{Group, Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{ExprType, ItemStruct, LitStr};

pub fn declare_id(id_tokens: TokenStream) -> TokenStream {
    let account_id_str = syn::parse2::<LitStr>(id_tokens)
        .expect("declare_id should have a string argument")
        .value();
    let first_char = account_id_str.as_bytes()[0];
    quote! {
        #[doc = "this is an id"]
        pub static ID: Pubkey = Pubkey { t: [#first_char] };
        #[doc = "this is a function which returns an id"]
        pub fn id() -> Pubkey {
            ID
        }
    }
}

fn is_field_boxed(field: &Field) -> bool {
    let Ty::Account(account_ty) = &field.ty else {
        return false;
    };
    account_ty.boxed
}

fn get_valid_field(field: &AccountField) -> Option<&Field> {
    let AccountField::Field(f) = &field else {
        return None;
    };
    Some(f)
}

fn get_valid_constraints(field: &Field) -> Option<&ConstraintGroup> {
    let Ty::Account(_) = &field.ty else {
        return None;
    };
    Some(&field.constraints)
}

fn get_valid_ident_constraints(field: &AccountField) -> Option<(&Ident, &ConstraintGroup)> {
    let field = get_valid_field(field)?;
    let constraints = get_valid_constraints(field)?;
    Some((&field.ident, constraints))
}

fn create_constraints_checks(
    val: &AccountsStruct,
    arg_names: &[Ident],
    arg_types: &[syn::Type],
) -> TokenStream {
    let mut checks = vec![];
    let mut fields = vec![];

    for field in val.fields.iter() {
        let Some(f) = get_valid_field(field) else {
            continue;
        };

        fields.push(&f.ident);

        let Some(constraints) = get_valid_constraints(f) else {
            continue;
        };

        for c in constraints.raw.iter() {
            checks.push(&c.raw);
        }

        // TODO do has_one constraint here
    }

    let generics = &val.generics;
    let ident = &val.ident;

    if checks.is_empty() {
        quote! {
            impl #generics #ident #generics {
                pub fn __check_constraints(&self, #(#arg_names: #arg_types),*) -> bool {
                    true
                }
            }
        }
    } else {
        quote! {
            #[allow(unused_variables)]
            impl #generics #ident #generics {
                pub fn __check_constraints(&self, #(#arg_names: #arg_types),*) -> bool {
                    #(let #fields = &self.#fields;)*
                    #(#checks)&&*
                }
            }
        }
    }
}

fn create_pre_invariants(val: &AccountsStruct) -> TokenStream {
    let ident = &val.ident;
    let generics = &val.generics;
    let mut pre = vec![];
    for field in val.fields.iter() {
        if let Some((ident, constraints)) = get_valid_ident_constraints(field) {
            if constraints.init.is_some() {
                continue;
            } else {
                let invariant = quote! {
                    self.#ident.account._check_invariant()
                };
                pre.push(invariant);
            }
        }
    }

    if pre.is_empty() {
        quote! {
            impl #generics #ident #generics {
                pub fn __pre_invariants(&self) -> bool {
                    true
                }
            }
        }
    } else {
        quote! {
            impl #generics #ident #generics {
                pub fn __pre_invariants(&self) -> bool {
                    #(#pre)&&*
                }
            }
        }
    }
}

fn create_post_invariants(val: &AccountsStruct) -> TokenStream {
    let ident = &val.ident;
    let generics = &val.generics;
    let mut post = vec![];
    for field in val.fields.iter() {
        if let Some((ident, constraints)) = get_valid_ident_constraints(field) {
            if constraints.is_close() {
                continue;
            }
            let invariant = quote! {
                self.#ident.account._check_invariant()
            };
            post.push(invariant);
        }
    }

    if post.is_empty() {
        quote! {
            impl #generics #ident #generics {
                pub fn __post_invariants(&self) -> bool {
                    true
                }
            }
        }
    } else {
        quote! {
            impl #generics #ident #generics {
                pub fn __post_invariants(&self) -> bool {
                    #(#post)&&*
                }
            }
        }
    }
}

pub fn derive_accounts(item: TokenStream) -> Result<TokenStream> {
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
    #[allow(clippy::redundant_clone)]
    let ident = val.ident.clone();
    #[allow(clippy::redundant_clone)]
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

    let arbitrary_impl = quote! {
        impl #generics kani::Arbitrary for #ident #generics {
            fn any() -> Self {
                Self {
                    #(#fields),*
                }
            }
        }
    };

    let pre_invariant_impl = create_pre_invariants(&val);
    let post_invariant_impl = create_post_invariants(&val);
    let constraint_checks = create_constraints_checks(&val, &arg_names, &arg_types);

    let res = quote! {
        #arbitrary_impl
        #pre_invariant_impl
        #post_invariant_impl
        #constraint_checks
    };

    Ok(res)
}

pub fn account(_args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let item = syn::parse2::<ItemStruct>(input.clone())?;
    let ident = item.ident;

    let res = quote! {
        #[derive(Arbitrary, AnchorDeserialize, AnchorSerialize)]
        #input

        impl AccountSerialize for #ident {}
        impl AccountDeserialize for #ident {}
    };
    Ok(res)
}
