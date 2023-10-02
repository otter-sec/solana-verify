use anyhow::Result;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, Expr, ExprArray, ExprAssign, ExprAssignOp,
    ExprAsync, ExprAwait, ExprBinary, ExprBlock, ExprBox, ExprBreak, ExprCall, ExprCast,
    ExprClosure, ExprContinue, ExprField, ExprForLoop, ExprGroup, ExprIf, ExprIndex, ExprLet,
    ExprLit, ExprLoop, ExprMacro, ExprMatch, ExprMethodCall, ExprParen, ExprPath, ExprRange,
    ExprReference, ExprRepeat, ExprReturn, ExprStruct, ExprTry, ExprTryBlock, ExprTuple, ExprType,
    ExprUnary, ExprUnsafe, ExprWhile, ExprYield, FnArg, GenericArgument, Generics, Item, ItemFn,
    ItemMod, Pat, PatType, PathArguments, Stmt, Type,
};

const KANI_UNWIND_AMOUNT: usize = 100;

fn get_ctx_type(ctx_param: &PatType) -> syn::Result<Punctuated<GenericArgument, Comma>> {
    let Type::Path(pa) = ctx_param.ty.as_ref() else {
        return Err(syn::Error::new_spanned(
            ctx_param.ty.as_ref(),
            "Invalid type for ctx",
        ));
    };

    for s in pa.path.segments.iter() {
        if s.ident == "Context" {
            if let PathArguments::AngleBracketed(an) = &s.arguments {
                return Ok(an.args.clone());
            }
        }
    }

    Err(syn::Error::new(Span::call_site(), "invalid context type"))
}

fn create_constraint_check(has_constraint: bool, parameters: &[&PatType]) -> TokenStream {
    if !has_constraint {
        quote! {
            let constraints = true;
        }
    } else {
        let constraint_params = parameters
            .iter()
            .map(|p| Ident::new(&p.pat.to_token_stream().to_string(), Span::call_site()));
        quote! {
            let constraints = ctx.accounts.__check_constraints(#(#constraint_params.clone()),*);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn create_succeeds_if(
    mod_name: &Ident,
    function_name: &Ident,
    generics: &Generics,
    ctx_type: &Punctuated<GenericArgument, Comma>,
    attr: Attribute,
    parameters: &[&PatType],
    parameter_names: &[Ident],
    constraint_check: TokenStream,
) -> syn::Result<TokenStream> {
    let precondition = match attr.parse_args::<Expr>() {
        Ok(p) => p.to_token_stream(),
        Err(_) => quote! { true },
    };
    let proof_name = format_ident!("succeeds_if_{}", function_name, span = function_name.span());

    Ok(quote! {
        #[kani::proof]
        #[kani::unwind(#KANI_UNWIND_AMOUNT)]
        pub fn #proof_name #generics () {
            #(
                let #parameters = kani::any();
            );*

            let conc: anchor_lang::context::ConcreteContext<#ctx_type> = kani::any();
            let ctx = conc.to_ctx();
            kani::assume(conc.to_ctx().accounts.__pre_invariants());
            let precondition = #precondition;
            kani::assume(precondition);
            #constraint_check
            let result = if constraints {
                #mod_name::#function_name(#(#parameter_names),*)
            } else {
                err!("constraint check failed")
            };
            kani::assert(
                result.is_ok(),
                "function failed to succeed given a precondition"
            );
        }
    })
}

#[allow(clippy::too_many_arguments)]
fn create_errors_if(
    mod_name: &Ident,
    function_name: &Ident,
    generics: &Generics,
    ctx_type: &Punctuated<GenericArgument, Comma>,
    attr: Attribute,
    parameters: &[&PatType],
    parameter_names: &[Ident],
    constraint_check: TokenStream,
) -> syn::Result<TokenStream> {
    let error_conds = match attr.parse_args::<Expr>() {
        Ok(p) => p.to_token_stream(),
        Err(_) => quote! { true },
    };
    let proof_name = format_ident!("errors_if_{}", function_name, span = function_name.span());

    Ok(quote! {
        #[kani::proof]
        #[kani::unwind(#KANI_UNWIND_AMOUNT)]
        pub fn #proof_name #generics () {
            #(
                let #parameters = kani::any();
            );*

            let conc: anchor_lang::context::ConcreteContext<#ctx_type> = kani::any();
            let ctx = conc.to_ctx();
            kani::assume(conc.to_ctx().accounts.__pre_invariants());
            let error_conds = #error_conds;
            kani::assume(error_conds);
            #constraint_check
            let result = if constraints {
                #mod_name::#function_name(#(#parameter_names),*)
            } else {
                err!("constraint check failed")
            };
            kani::assert(
                result.is_err(),
                "Function succeeded when it should have errored"
            );
        }
    })
}

fn create_verify(
    mod_name: &Ident,
    function_name: &Ident,
    generics: &Generics,
    ctx_type: &Punctuated<GenericArgument, Comma>,
    parameters: &[&PatType],
    parameter_names: &[Ident],
) -> syn::Result<TokenStream> {
    let proof_name = format_ident!("verify_{}", function_name, span = function_name.span());
    let res = quote! {
        #[kani::proof]
        #[kani::unwind(#KANI_UNWIND_AMOUNT)]
        pub fn #proof_name #generics () {
            #(
                let #parameters = kani::any();
            );*

            let conc: anchor_lang::context::ConcreteContext<#ctx_type> = kani::any();
            let ctx = conc.to_ctx();
            kani::assume(conc.to_ctx().accounts.__pre_invariants());
            let result = #mod_name::#function_name(#(#parameter_names),*);
            kani::assert(
                result.is_err() || conc.to_ctx().accounts.__post_invariants(),
                "Function failed",
            );
        }
    };
    Ok(res)
}

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

pub fn remove_verify_ignore_statements(item: &mut ItemFn) {
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

fn verification_harness_of(mod_name: &Ident, item: &mut ItemFn) -> syn::Result<TokenStream> {
    remove_verify_ignore_statements(item);

    let mut parameters = vec![];
    let mut parameter_names = vec![];
    let mut ctx_type = None;

    for p in item.sig.inputs.iter() {
        let FnArg::Typed(a) = p else { unreachable!() };

        let Pat::Ident(pi) = a.pat.as_ref() else {
            return Err(syn::Error::new_spanned(
                a.pat.as_ref(),
                "Expected identifier",
            ));
        };

        parameter_names.push(pi.ident.clone());

        if pi.ident != "ctx" {
            parameters.push(a);
        } else {
            ctx_type = Some(get_ctx_type(a)?);
        }
    }

    let Some(ctx_type) = ctx_type else {
        return Err(syn::Error::new(
            Span::call_site(),
            "missing context parameter",
        ));
    };

    let function_name = &item.sig.ident;
    let generics = &item.sig.generics;
    let mut succeeds_if_harness: Option<TokenStream> = None;
    let mut errors_if_harness: Option<TokenStream> = None;
    let mut create_succeeds_attr: Option<Attribute> = None;
    let mut create_errors_attr: Option<Attribute> = None;
    let mut has_constraint = false;

    for attr in std::mem::take(&mut item.attrs).into_iter() {
        if attr.path.is_ident("succeeds_if") {
            create_succeeds_attr = Some(attr);
        } else if attr.path.is_ident("errors_if") {
            create_errors_attr = Some(attr);
        } else if attr.path.is_ident("has_constraint") {
            has_constraint = true;
        } else {
            item.attrs.push(attr);
        }
    }

    if let Some(attr) = create_succeeds_attr {
        succeeds_if_harness = Some(create_succeeds_if(
            mod_name,
            function_name,
            generics,
            &ctx_type,
            attr,
            &parameters,
            &parameter_names,
            create_constraint_check(has_constraint, &parameters),
        )?);
    }

    if let Some(attr) = create_errors_attr {
        errors_if_harness = Some(create_errors_if(
            mod_name,
            function_name,
            generics,
            &ctx_type,
            attr,
            &parameters,
            &parameter_names,
            create_constraint_check(has_constraint, &parameters),
        )?);
    }

    let verify = create_verify(
        mod_name,
        function_name,
        generics,
        &ctx_type,
        &parameters,
        &parameter_names,
    )?;

    let res = match (succeeds_if_harness, errors_if_harness) {
        (Some(succeeds_if_harness), Some(errors_if_harness)) => quote! {
            #verify
            #succeeds_if_harness
            #errors_if_harness
        },
        (Some(succeeds_if_harness), None) => quote! {
            #verify
            #succeeds_if_harness
        },
        (None, Some(errors_if_harness)) => quote! {
            #verify
            #errors_if_harness
        },
        (None, None) => verify,
    };
    Ok(res)
}

pub fn program(_args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let mut item = syn::parse2::<ItemMod>(input)?;
    let name = &item.ident;
    let items = &mut item.content;
    if items.is_none() {
        panic!("#[program] must be placed on full modules:  {name}")
    }

    let mut harnesses = Vec::new();
    for item in &mut items.as_mut().unwrap().1 {
        if let Item::Fn(item) = item {
            if let Ok(harness) = verification_harness_of(name, item) {
                harnesses.push(harness);
            } else {
                println!("ignored harness for: {:?}", item.sig.ident);
            }
        }
    }

    let res = quote! {
        #item
        #(#harnesses)*
    };
    Ok(res)
}
