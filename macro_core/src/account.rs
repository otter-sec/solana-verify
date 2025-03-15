use anchor_syn::{AccountField, AccountsStruct, ConstraintGroup, Field, Ty, parser as anchor_parser};
use anyhow::Result;
use proc_macro2::{Group, Ident, Span, TokenStream};
use quote::{quote, ToTokens, format_ident};
use syn::{ExprType, ItemStruct, LitStr, parse::{ParseStream}};

pub fn declare_id(id_tokens: TokenStream) -> TokenStream {
    let account_id_str = syn::parse2::<LitStr>(id_tokens)
        .expect("declare_id should have a string argument")
        .value();
    let first_char = account_id_str.as_bytes()[0];
    quote! {
        #[doc = "this is an id"]
        pub static ID: Pubkey = Pubkey { t: [#first_char], _padding: unsafe { std::mem::zeroed() } };
        #[doc = "this is a function which returns an id"]
        pub fn id() -> Pubkey {
            ID
        }
    }
}

pub fn pubkey(id_tokens: TokenStream) -> TokenStream {
    let account_id_str = syn::parse2::<LitStr>(id_tokens)
        .expect("pubkey should have a string argument")
        .value();
    let first_char = account_id_str.as_bytes()[0];
    
    // return pubkey with first char
    quote! {
        Pubkey { t: [#first_char], _padding: [0; 31] }
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
                pub fn __check_constraints(&self, #(#arg_names: #arg_types),*) -> solana_program::Result<bool>  {
                    Ok(true)
                }
            }
        }
    } else {
        quote! {
            #[allow(unused_variables)]
            impl #generics #ident #generics {
                pub fn __check_constraints(&self, #(#arg_names: #arg_types),*) -> solana_program::Result<bool> {
                    #(let #fields = &self.#fields;)*
                    Ok(#(#checks)&&*)
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
    let ident = val.ident.clone();
    let generics = val.generics.clone();

    let bumps_fields = val
        .fields
        .iter()
        .map(|field| {
            let (boxed, ident) = match field {
                AccountField::Field(field) => (is_field_boxed(field), &field.ident),
                AccountField::CompositeField(c_field) => (false, &c_field.ident),
            };
            quote! {
                #ident: u8
            }
        })
        .collect::<Vec<TokenStream>>();

    let bumps_struct_ident = format_ident!("{}Bumps", ident);

    let bumps_impl = quote! {
        #[derive(Default, Debug)]
        struct #bumps_struct_ident {
            #(#bumps_fields),*
        }

        impl anchor_lang::Bumps for #ident<'_> {
            type Bumps = #bumps_struct_ident;
        }
    };
    println!("{}", bumps_impl);

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
        #bumps_impl
        #arbitrary_impl
        #pre_invariant_impl
        #post_invariant_impl
        #constraint_checks
    };

    Ok(res)
}

pub fn account(args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let item = syn::parse2::<ItemStruct>(input.clone())?;
    let ident = item.ident;

    let args_parsed = syn::parse::Parser::parse2(
        syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
        args
    )?;

    let is_zero_copy = args_parsed.iter().any(|x| x.is_ident("zero_copy"));

    let ser_derives = if is_zero_copy {
        quote! {
        }
    } else {
        quote! {
            #[derive(AnchorSerialize, AnchorDeserialize)]
        }
    };

    let ser_impls = if is_zero_copy {
        quote! {
            impl anchor_lang::ZeroCopy for #ident {}
        }
    } else {
        quote! {
            impl AccountSerialize for #ident {
                fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> solana_program::Result<()> {
                    self.serialize(writer)
                        .map_err(|_| anchor_lang::Error::AccountDidNotSerialize)
                }
            }

            impl AccountDeserialize for #ident {
                fn try_deserialize_unchecked(buf: &mut &[u8]) -> solana_program::Result<Self> {
                    Self::deserialize(buf).map_err(|_| anchor_lang::Error::AccountDidNotDeserialize)
                }
            }
        }
    };

    let res = quote! {        
        #ser_derives
        #[derive(Arbitrary)]
        #input

        #ser_impls

        impl anchor_lang::Discriminator for #ident {
            const DISCRIMINATOR: [u8; 8] = *b"00000000";
        }

        impl anchor_lang::Owner for #ident {
            fn owner() -> anchor_lang::prelude::Pubkey {
                anchor_lang::prelude::Pubkey::new_from_array([10; 1])
            }
        }
    };
    Ok(res)
}


pub fn zero_copy(args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let s = syn::parse2::<ItemStruct>(input.clone())?;
    let attr = s
        .attrs
        .iter()
        .find(|attr| anchor_parser::tts_to_string(&attr.path) == "repr");

    let repr = match attr {
        Some(_) => quote! {},
        None => quote! { #[repr(C)] },
    };

    Ok(quote! {
        #[derive(Copy, Clone)]
        #repr
        #[derive(bytemuck::Pod)]
        #[derive(bytemuck::Zeroable)]
        #input
    })
}
