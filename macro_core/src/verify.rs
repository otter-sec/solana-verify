// This is for non-anchor verification

use anyhow::Result;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_quote, Attribute, Expr, ExprArray, ExprAssign, ExprAssignOp, ExprAsync, ExprAwait,
    ExprBinary, ExprBlock, ExprBox, ExprBreak, ExprCall, ExprCast, ExprClosure, ExprContinue,
    ExprField, ExprForLoop, ExprGroup, ExprIf, ExprIndex, ExprLet, ExprLit, ExprLoop, ExprMacro,
    ExprMatch, ExprMethodCall, ExprParen, ExprPath, ExprRange, ExprReference, ExprRepeat,
    ExprReturn, ExprStruct, ExprTry, ExprTryBlock, ExprTuple, ExprType, ExprUnary, ExprUnsafe,
    ExprWhile, ExprYield, FnArg, ItemFn, Pat, PatIdent, PatType, Signature, Stmt, Type,
};

const KANI_UNWIND_AMOUNT: usize = 16;
const MAX_UNPACK: usize = 10;

fn check_expr_is_ignored(expr: &Expr) -> bool {
    match expr {
        Expr::Array(ExprArray { attrs, .. })
        | Expr::Assign(ExprAssign { attrs, .. })
        | Expr::AssignOp(ExprAssignOp { attrs, .. })
        | Expr::Async(ExprAsync { attrs, .. })
        | Expr::Await(ExprAwait { attrs, .. })
        | Expr::Binary(ExprBinary { attrs, .. })
        | Expr::Block(ExprBlock { attrs, .. })
        | Expr::Box(ExprBox { attrs, .. })
        | Expr::Break(ExprBreak { attrs, .. })
        | Expr::Call(ExprCall { attrs, .. })
        | Expr::Cast(ExprCast { attrs, .. })
        | Expr::Closure(ExprClosure { attrs, .. })
        | Expr::Continue(ExprContinue { attrs, .. })
        | Expr::Field(ExprField { attrs, .. })
        | Expr::ForLoop(ExprForLoop { attrs, .. })
        | Expr::Group(ExprGroup { attrs, .. })
        | Expr::If(ExprIf { attrs, .. })
        | Expr::Index(ExprIndex { attrs, .. })
        | Expr::Let(ExprLet { attrs, .. })
        | Expr::Lit(ExprLit { attrs, .. })
        | Expr::Loop(ExprLoop { attrs, .. })
        | Expr::Macro(ExprMacro { attrs, .. })
        | Expr::Match(ExprMatch { attrs, .. })
        | Expr::MethodCall(ExprMethodCall { attrs, .. })
        | Expr::Paren(ExprParen { attrs, .. })
        | Expr::Path(ExprPath { attrs, .. })
        | Expr::Range(ExprRange { attrs, .. })
        | Expr::Reference(ExprReference { attrs, .. })
        | Expr::Repeat(ExprRepeat { attrs, .. })
        | Expr::Return(ExprReturn { attrs, .. })
        | Expr::Struct(ExprStruct { attrs, .. })
        | Expr::Try(ExprTry { attrs, .. })
        | Expr::TryBlock(ExprTryBlock { attrs, .. })
        | Expr::Tuple(ExprTuple { attrs, .. })
        | Expr::Type(ExprType { attrs, .. })
        | Expr::Unary(ExprUnary { attrs, .. })
        | Expr::Unsafe(ExprUnsafe { attrs, .. })
        | Expr::While(ExprWhile { attrs, .. })
        | Expr::Yield(ExprYield { attrs, .. }) => {
            attrs.iter().any(|a| a.path.is_ident("verify_ignore"))
        }
        _ => false,
    }
}

fn remove_verify_ignore_statements(item: &mut ItemFn) {
    for stmt in std::mem::take(&mut item.block.stmts) {
        let contains_ignore = match &stmt {
            Stmt::Local(local) => local.attrs.iter().any(|a| a.path.is_ident("verify_ignore")),
            Stmt::Item(_item) => todo!(),
            Stmt::Expr(expr) | Stmt::Semi(expr, _) => check_expr_is_ignored(expr),
        };

        if !contains_ignore {
            item.block.stmts.push(stmt);
        }
    }
}

fn create_succeeds_if(
    function_sig: &Signature,
    unpack_types: &[(syn::Type, String)],
    attr: Attribute,
    parameters: &[PatType],
    parameter_names: &[Expr],
) -> syn::Result<TokenStream> {
    // get precondition that was part of the macro invocation
    let precondition = match attr.parse_args::<Expr>() {
        Ok(p) => p.to_token_stream(),
        Err(_) => quote! { true },
    };

    let function_name = function_sig.ident.clone();
    let proof_name = format_ident!("succeeds_if_{}", function_name, span = function_name.span());

    // create all the accounts that can be constrained
    let mut unpack_decls = Vec::new();
    let mut expect_unpack = Vec::new();
    for (t, s) in unpack_types {
        let t_lower = syn::Ident::new(&s.to_lowercase(), Span::call_site());
        let t_lower_plural = syn::Ident::new(&format!("{}s", t_lower), Span::call_site());
        unpack_decls.push(quote! {
            let mut #t_lower_plural: Vec<#t> = Vec::new();
            for _ in 0..#MAX_UNPACK {
                let x = kani::any();
                #t_lower_plural.push(x);
                kani::assume(x._check_invariant());
            }
        });

        expect_unpack.push(quote! {
            for x in #t_lower_plural.iter() {
                #t::expect_unpack(x.clone());
            }
        });
    }

    let fn_call = if function_sig.receiver().is_some() {
        quote! {
            let result = #function_name(#(#parameter_names),*);
        }
    } else {
        quote! {
            let result = Self::#function_name(#(#parameter_names),*);
        }
    };

    Ok(quote! {
        #[kani::proof]
        #[kani::unwind(#KANI_UNWIND_AMOUNT)]
        pub fn #proof_name() {
            // Parameters includes the Vec of AccountInfos that the
            // transaction processor can choose to deserialize from.
            // (This represents a Vec of any size up to a global bound)

            #(
                let #parameters = kani::any();
            );*

            #(#unpack_decls)*

            // Apply preconditions
            let precondition = #precondition;
            kani::assume(precondition);

            // magic macro-defined function needed to initialize globals
            // that track each of the unpackable structures.
            init();

            #(#expect_unpack)*

            // Finally: Actually call the function we are trying to verify
            #fn_call

            // Assert that the transaction was successful and we
            // have the correct new balances
            assert!(result.is_ok());

            // TODO:
            // Assert that they used all of the accounts
            //
            // assert!(Account::num_used() == 2);

            // TODO:
            // Get the re-packed accounts and assert post-invariants
            //
            // let from_acct_post = Account::get_packed(0);
            // let to_acct_post = Account::get_packed(1);
            // assert!(from_acct_post.amount == from_old_amt - amount);
            // assert!(to_acct_post.amount == to_old_amt + amount);


        }
    })
}

fn create_verify(
    function_sig: &Signature,
    unpack_types: &[(syn::Type, String)],
    parameters: &[PatType],
    parameter_names: &[Expr],
    postcondition: Option<Attribute>,
) -> syn::Result<TokenStream> {
    // get postcondition that was part of the macro invocation
    let postcondition = match postcondition {
        Some(a) => a.parse_args::<Expr>().unwrap().to_token_stream(),
        None => quote! { true },
    };

    let function_name = function_sig.ident.clone();
    let proof_name = format_ident!("verify_{}", function_name, span = function_name.span());

    // create all the accounts that can be constrained
    let mut unpack_decls = Vec::new();
    let mut expect_unpack = Vec::new();
    for (t, s) in unpack_types {
        let t_lower = syn::Ident::new(&s.to_lowercase(), Span::call_site());
        let t_lower_plural = syn::Ident::new(&format!("{}s", t_lower), Span::call_site());
        unpack_decls.push(quote! {
            let mut #t_lower_plural: Vec<#t> = Vec::new();
            for _ in 0..#MAX_UNPACK {
                let x = kani::any();
                #t_lower_plural.push(x);
                kani::assume(x._check_invariant());
            }
        });

        expect_unpack.push(quote! {
            for x in #t_lower_plural.iter() {
                #t::expect_unpack(x.clone());
            }
        });

        if t_lower == "account" {
            let before_t_lower_plural =
                syn::Ident::new(&format!("before_{}s", t_lower), Span::call_site());
            unpack_decls.push(quote! {
                let mut #before_t_lower_plural: Vec<#t> = Vec::new();
                for x in #t_lower_plural.iter() {
                    #before_t_lower_plural.push(x.clone());
                }
            });
        }
    }

    let fn_call = if function_sig.receiver().is_some() {
        quote! {
            let result = #function_name(#(#parameter_names),*);
        }
    } else {
        quote! {
            let result = Self::#function_name(#(#parameter_names),*);
        }
    };

    Ok(quote! {
        #[kani::proof]
        #[kani::unwind(#KANI_UNWIND_AMOUNT)]
        pub fn #proof_name() {
            // Parameters includes the Vec of AccountInfos that the
            // transaction processor can choose to deserialize from.
            // (This represents a Vec of any size up to a global bound)

            #(
                let #parameters = kani::any();
            );*

            #(#unpack_decls)*

            // magic macro-defined function needed to initialize globals
            // that track each of the unpackable structures.
            init();

            #(#expect_unpack)*

            // Finally: Actually call the function we are trying to verify
            #fn_call

            // Assert that if the transaction was successful then
            // the postconditions apply.
            assert!(!result.is_ok() || (#postcondition));

        }
    })
}

fn verification_harness_of(
    item: &mut ItemFn,
    types: &[(syn::Type, String)],
) -> syn::Result<TokenStream> {
    remove_verify_ignore_statements(item);

    let mut parameters = vec![];
    let mut parameter_names = vec![];

    for p in item.sig.inputs.iter() {
        let FnArg::Typed(mut a) = p.clone() else {
            unreachable!()
        };

        let Pat::Ident(pi) = a.pat.as_ref().clone() else {
            return Err(syn::Error::new_spanned(
                a.pat.as_ref(),
                "Expected identifier",
            ));
        };

        // all parameters should be included

        // We need both the type and "how to pass it"... e.g. we may need to instantiate
        // a Vec<AccountInfo> and pass it as a reference
        let mut ty = a.ty.clone();
        let mut expr = Expr::Path(ExprPath {
            attrs: vec![],
            qself: None,
            path: pi.ident.clone().into(),
        });

        // replace reference with value types
        if let Type::Reference(tp) = ty.as_ref() {
            ty = tp.elem.clone();
            let ident = pi.ident.clone();
            expr = parse_quote! { &#ident };

            // We can't have an array without a size, so lets just use a Vec
            // NOTE: Currently, this only handles account_infos: &[AccountInfo]
            if let Type::Slice(tp) = ty.as_ref() {
                // Reference to a slice should be replaced with a Vec
                if let Type::Path(tp) = tp.elem.as_ref() {
                    let path = tp.path.clone();
                    if let Some(ident) = path.get_ident() {
                        if ident == "AccountInfo" {
                            // Replace with a anchor_lang::vec::fast::Vec
                            ty = parse_quote! {solana_program::vec::fast::Vec<#ident> };
                            let ident = format_ident!("account_infos", span = pi.ident.span());
                            expr = parse_quote! { &#ident };
                            a.pat = Box::new(Pat::Ident(PatIdent {
                                attrs: vec![],
                                by_ref: None,
                                mutability: None,
                                ident,
                                subpat: None,
                            }));
                        }
                    }
                }
            }
        }

        a.ty = ty;

        parameter_names.push(expr);
        parameters.push(a);
    }

    let function_sig = &item.sig;
    let mut precondition: Option<TokenStream> = None;
    let mut create_succeeds_attr: Option<Attribute> = None;
    let mut _has_constraint = false;
    let mut postcondition: Option<Attribute> = None;
    for attr in std::mem::take(&mut item.attrs).into_iter() {
        if attr.path.is_ident("succeeds_if") {
            create_succeeds_attr = Some(attr);
        } else if attr.path.is_ident("has_constraint") {
            _has_constraint = true;
        } else if attr.path.is_ident("post_condition") {
            postcondition = Some(attr);
        } else {
            item.attrs.push(attr);
        }
    }

    if let Some(attr) = create_succeeds_attr {
        precondition = Some(create_succeeds_if(
            function_sig,
            types,
            attr,
            &parameters,
            &parameter_names,
        )?);
    }

    let verify = create_verify(
        function_sig,
        types,
        &parameters,
        &parameter_names,
        postcondition,
    )?;

    let res = match precondition {
        Some(precondition) => quote! {
            #verify
            #precondition
        },
        None => verify,
    };
    Ok(res)
}

pub fn verify(args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let Ok(mut item) = syn::parse2::<ItemFn>(input) else {
        panic!("use #[verify] on a function")
    };

    // Parse the array of types in arg
    let types = syn::parse2::<syn::ExprArray>(args)?;
    let types = types
        .elems
        .into_iter()
        .map(|t| {
            let ty = syn::parse2::<syn::Type>(t.into_token_stream()).unwrap();
            let t = ty.to_token_stream().to_string();
            (ty, t)
        })
        .collect::<Vec<_>>();

    let mut harnesses = Vec::new();
    if let Ok(harness) = verification_harness_of(&mut item, &types) {
        println!("created harness for: {:?}", item.sig.ident);
        println!("harness: {}", harness);
        harnesses.push(harness);
    } else {
        println!("ignored harness for: {:?}", item.sig.ident);
    }

    let res = quote! {
        #item
        #(#harnesses)*
    };
    Ok(res)
}
