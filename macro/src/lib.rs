use otter_solana_macro_core as core;
use proc_macro::TokenStream;

#[proc_macro]
pub fn declare_id(s: TokenStream) -> TokenStream {
    core::account::declare_id(s.into()).into()
}

#[proc_macro_derive(Accounts, attributes(account, instruction))]
pub fn derive_accounts(item: TokenStream) -> TokenStream {
    core::account::derive_accounts(item.into()).unwrap().into()
}

#[proc_macro_derive(InitSpace, attributes(max_len))]
pub fn derive_init_space(item: TokenStream) -> TokenStream {
    core::space::derive_init_space(item.into()).unwrap().into()
}

#[proc_macro_attribute]
pub fn account(args: TokenStream, item: TokenStream) -> TokenStream {
    core::account::account(args.into(), item.into())
        .unwrap()
        .into()
}

#[proc_macro_attribute]
pub fn error_code(args: TokenStream, item: TokenStream) -> TokenStream {
    core::error::error_code(args.into(), item.into())
        .unwrap()
        .into()
}

#[proc_macro_attribute]
pub fn invariant(attr: TokenStream, item: TokenStream) -> TokenStream {
    core::invariant::invariant(attr.into(), item.into())
        .unwrap()
        .into()
}

#[cfg(feature = "anchor")]
#[proc_macro_attribute]
pub fn program(args: TokenStream, item: TokenStream) -> TokenStream {
    core::program::program(args.into(), item.into())
        .unwrap()
        .into()
}

#[proc_macro]
pub fn verify_unpackable(types: TokenStream) -> TokenStream {
    core::unpackable::unpackable(types.into()).unwrap().into()
}

#[proc_macro_attribute]
pub fn access_control(args: TokenStream, item: TokenStream) -> TokenStream {
    core::access_control::access_control(args.into(), item.into())
        .expect("access_control used on non-function?")
        .into()
}

#[proc_macro_attribute]
pub fn helper_fn(_args: TokenStream, item: TokenStream) -> TokenStream {
    core::helper_fn::helper_fn(item.into())
        .expect("helper_fn used on non-function?")
        .into()
}

#[cfg(feature = "verify")]
#[proc_macro_attribute]
pub fn verify(args: TokenStream, item: TokenStream) -> TokenStream {
    core::verify::verify(args.into(), item.into())
        .expect("verify used on non-function?")
        .into()
}
