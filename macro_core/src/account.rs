use anchor_syn::{
    codegen::accounts::{
        __client_accounts, __cpi_client_accounts, bumps, to_account_infos, to_account_metas,
    },
    codegen::accounts::{generics, ParsedGenerics},
    parser as anchor_parser, AccountField, AccountsStruct, CompositeField, ConstraintGroup, Field,
    Ty,
};
use anyhow::Result;
use proc_macro2::{Group, Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse::ParseStream, Expr, ExprType, ItemStruct, Lit, LitStr};

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

fn get_primitive_constraints(field: &Field) -> Option<ConstraintGroup> {
    match field.ty {
        Ty::Account(_) | Ty::AccountLoader(_) => Some(field.constraints.clone()),
        Ty::AccountInfo => Some(Default::default()),
        _ => None,
    }
}

fn get_composite_constraints(field: &CompositeField) -> Option<ConstraintGroup> {
    Some(field.constraints.clone())
}

fn get_valid_ident_constraints(field: &AccountField) -> Option<(Ident, ConstraintGroup)> {
    Some(match field {
        AccountField::Field(field) => (field.ident.clone(), get_primitive_constraints(field)?),
        AccountField::CompositeField(field) => {
            (field.ident.clone(), get_composite_constraints(field)?)
        }
    })
}

fn create_constraints_checks(
    val: &AccountsStruct,
    arg_names: &[Ident],
    arg_types: &[syn::Type],
) -> TokenStream {
    let mut checks = vec![];
    let mut fields = vec![];

    for field in val.fields.iter() {
        let AccountField::Field(f) = field else {
            continue;
        };

        fields.push(&f.ident);

        let Some(constraints) = get_primitive_constraints(f) else {
            continue;
        };

        for c in constraints.raw.iter() {
            checks.push(c.raw.clone());
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

fn create_pre_invariants(val: &AccountsStruct, has_assume_types: bool) -> TokenStream {
    let ident = &val.ident;
    let generics = &val.generics;
    let mut pre = vec![];
    let mut ensure_init = vec![];
    for field in val.fields.iter() {
        if let Some((ident, constraints)) = get_valid_ident_constraints(field) {
            if constraints.init.is_some() {
                continue;
            } else {
                let (invariant, init, field_ty) = match field {
                    AccountField::Field(f) => {
                        if f.is_optional {
                            (
                                quote! {
                                    self.#ident.as_ref().map(|x| x.check_invariant())
                                },
                                quote! {
                                    self.#ident.as_ref().map(|x| x.ensure_init())
                                },
                                &f.ty,
                            )
                        } else {
                            (
                                quote! {
                                    slf.#ident.check_invariant()
                                },
                                quote! {
                                    slf.#ident.ensure_init()
                                },
                                &f.ty,
                            )
                        }
                    }
                    AccountField::CompositeField(_) => (
                        quote! {
                            slf.#ident.__pre_invariants(remaining_accounts);
                        },
                        quote! {},
                        // dummy
                        &Ty::Signer,
                    ),
                };

                pre.push(invariant);
                if matches!(field_ty, Ty::AccountLoader(_) | Ty::Account(_)) {
                    ensure_init.push(init);
                }
            }
        }
    }

    let assume_types = if has_assume_types {
        quote!{ slf._assume_types(remaining_accounts) }
    } else {
        quote!{ }
    };

    quote! {
        impl #generics #ident #generics {
            pub fn __pre_invariants(&self, remaining_accounts: &[AccountInfo])
            where Self: 'static {
                use shared::Invariant;
                let slf
                : &'static Self = unsafe { std::mem::transmute(self) };

                #assume_types;

                #(#ensure_init);*;
                #(#pre);*;
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
            let (invariant, transition_invariant) = match field {
                AccountField::Field(f) => {
                    let invariant = if f.is_optional {
                        quote! {
                            slf.#ident.as_ref().map(|x| x.check_invariant())
                        }
                    } else {
                        quote! {
                            slf.#ident.check_invariant();
                        }
                    };

                    let transition_invariant = match (constraints.init.as_ref(), f.is_optional) {
                        (None, true) => {
                            quote! { slf.#ident.as_ref().map(|x| x.check_transition_invariant(&old.accounts.#ident.as_ref().unwrap().to_account_info(), &remaining_accounts)) }
                        }
                        (None, false) => {
                            quote! { slf.#ident.check_transition_invariant(&old.accounts.#ident.to_account_info(), &remaining_accounts) }
                        }
                        _ => quote! {},
                    };

                    (invariant, transition_invariant)
                }
                AccountField::CompositeField(_) => (
                    quote! {
                        slf.#ident.__post_invariants(&anchor_lang::context::DummyContext::new(old.accounts.#ident.clone(), old.remaining_accounts.to_vec()))
                    },
                    quote! { () },
                ),
            };

            post.push(invariant);
            post.push(transition_invariant);
        }
    }

    let assert_initialized = quote! {
        kani::assert(
            crate::solana_program::verify_helpers::slice_all(&old.remaining_accounts, |acc| acc.is_initialized()),
            "all accounts must be initialized"
        );
    };

    if post.is_empty() {
        quote! {
            impl #generics #ident #generics
            where Self: 'static  {
                pub fn __post_invariants(&self, old: &anchor_lang::context::DummyContext<Self>) {
                    #assert_initialized
                }
            }
        }
    } else {
        quote! {
            impl #generics #ident #generics
            where Self: 'static {
                pub fn __post_invariants(&self, old: &anchor_lang::context::DummyContext<Self>) {
                    #assert_initialized;
                    use shared::Invariant;
                    // SAFETY: Nah
                    let slf: &'static Self = unsafe {
                        std::mem::transmute(self)
                    };
                    let old: &'static anchor_lang::context::DummyContext<Self> = unsafe {
                        std::mem::transmute(old)
                    };
                    let remaining_accounts: &'static [AccountInfo<'static>] = unsafe {
                        std::mem::transmute(old.remaining_accounts.as_slice())
                    };
                    #(#post);*
                    ;
                }
            }
        }
    }
}

pub fn derive_accounts(item: TokenStream) -> Result<TokenStream> {
    let arg_item = syn::parse2::<ItemStruct>(item.clone())?;
    let mut arg_names: Vec<Ident> = vec![];
    let mut arg_types: Vec<syn::Type> = vec![];
    let mut has_clone = false;
    let mut assume_types_impl = None;

    for t in arg_item.attrs {
        if t.path.is_ident("instruction") {
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
        } else if t.path.is_ident("invariant") {
            unimplemented!("invariants are only supported on #[account] structs")
        } else if t.path.is_ident("derive") && t.tokens.to_string().contains("Clone") {
            has_clone = true;
        } else if t.path.is_ident("assume_types") {
            let expr = syn::parse2::<Expr>(t.tokens)?;
            let ident = arg_item.ident.clone();
            let generics =arg_item.generics.clone();
            assume_types_impl = Some(quote!{
                impl #generics #ident #generics {
                    #[deny(dead_code)]
                    fn _assume_types(&self, remaining_accounts: &[AccountInfo]) {
                        #expr
                    }
                }
            });
        }
    }

    let item = item.to_token_stream();
    let val = syn::parse2::<AccountsStruct>(item)?;
    let ident = val.ident.clone();

    let ParsedGenerics {
        combined_generics,
        trait_generics: _,
        struct_generics,
        where_clause,
    } = generics(&val);

    let bumps_impl = bumps::generate(&val);

    // println!("{}", bumps_impl);

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
        impl<#combined_generics> kani::Arbitrary for #ident<#struct_generics> #where_clause {
            fn any() -> Self {
                Self {
                    #(#fields),*
                }
            }
        }
    };

    let clone_impl = if has_clone {
        quote! {}
    } else {
        let clone_fields = val
            .fields
            .iter()
            .map(|x| {
                let (ident, is_acc_info, is_optional) = match x {
                    AccountField::Field(field) => (
                        &field.ident,
                        matches!(field.ty, Ty::AccountInfo),
                        field.is_optional,
                    ),
                    AccountField::CompositeField(field) => (&field.ident, false, false),
                };
                if is_acc_info {
                    if is_optional {
                        quote! {
                            #ident: {
                                let mut tmp = self.#ident.clone();
                                if let Some(tmp) = tmp.as_mut() {
                                    tmp.clone_data();
                                }
                                tmp
                            }
                        }
                    } else {
                        quote! {
                            #ident: {
                                let mut tmp = self.#ident.clone();
                                tmp.clone_data();
                                tmp
                            }
                        }
                    }
                } else {
                    quote! { #ident: self.#ident.clone() }
                }
            })
            .collect::<Vec<_>>();
        quote! {
            impl<#combined_generics> Clone for #ident<#struct_generics> #where_clause {
                fn clone(&self) -> Self {
                    Self {
                        #(#clone_fields),*
                    }
                }
            }
        }
    };

    let pre_invariant_impl = create_pre_invariants(&val, assume_types_impl.is_some());
    let post_invariant_impl = create_post_invariants(&val);
    // println!("post invariants for {} {}", ident, post_invariant_impl);
    let constraint_checks = create_constraints_checks(&val, &arg_names, &arg_types);

    let client_accounts = __client_accounts::generate(&val);
    let cpi_client_accounts = __cpi_client_accounts::generate(&val);
    let to_account_metas = to_account_metas::generate(&val);
    let to_account_infos = to_account_infos::generate(&val);
    let assume_types_impl = assume_types_impl.unwrap_or_default();

    let res = quote! {
        #bumps_impl
        #arbitrary_impl
        #clone_impl
        #pre_invariant_impl
        #post_invariant_impl
        #constraint_checks
        #client_accounts
        #cpi_client_accounts
        #to_account_metas
        #to_account_infos
        #assume_types_impl
    };

    // println!("{}", res);

    Ok(res)
}

pub fn account(args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let item = syn::parse2::<ItemStruct>(input.clone())?;
    let ident = item.ident;

    let args_parsed = syn::parse::Parser::parse2(
        syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated,
        args,
    )?;

    if args_parsed.iter().any(|x| {
        if let syn::Expr::Lit(l) = x {
            if let Lit::Str(s) = &l.lit {
                return s.value() == String::from("internal");
            }
        }
        false
    }) {
        return Ok(input);
    }

    let is_zero_copy = args_parsed.iter().any(|x| {
        if let syn::Expr::Path(p) = x {
            return p.path.is_ident("zero_copy");
        } else {
            false
        }
    });

    let ser_derives = if is_zero_copy {
        quote! {
            #[derive(Clone)]
        }
    } else {
        quote! {
            #[derive(AnchorSerialize, AnchorDeserialize, Clone)]
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
                    Ok(kani::any())
                    // Self::deserialize(buf).map_err(|_| anchor_lang::Error::AccountDidNotDeserialize)
                }
            }
        }
    };

    let invariant = match (
        item.attrs.iter().any(|x| x.path.is_ident("invariant")),
        item.attrs
            .iter()
            .any(|x| x.path.is_ident("transition_invariant")),
    ) {
        (false, false) => quote! {
            #[anchor_lang::prelude::invariant(true)]
            #[anchor_lang::prelude::transition_invariant(true)]
        },
        (true, false) => quote! { #[anchor_lang::prelude::transition_invariant(true)] },
        (false, true) => quote! { #[anchor_lang::prelude::invariant(true)] },
        _ => quote! {},
    };

    let res = quote! {
        #ser_derives
        #[derive(Arbitrary)]
        #invariant
        #input

        #ser_impls

        impl anchor_lang::Discriminator for #ident {
            const DISCRIMINATOR: [u8; 8] = *b"00000000";
        }

        impl anchor_lang::Owner for #ident {
            fn owner() -> anchor_lang::prelude::Pubkey {
                anchor_lang::prelude::Pubkey::new_from_array2([10; 1])
            }
        }
    };
    Ok(res)
}

pub fn zero_copy(_args: TokenStream, input: TokenStream) -> Result<TokenStream> {
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
