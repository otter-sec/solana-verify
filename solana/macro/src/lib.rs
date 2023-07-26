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

#[proc_macro_attribute]
pub fn account(args: TokenStream, input: TokenStream) -> TokenStream {
    core::account::account(args.into(), input.into())
        .unwrap()
        .into()
}

#[proc_macro_attribute]
pub fn error_code(args: TokenStream, input: TokenStream) -> TokenStream {
    core::error::error_code(args.into(), input.into())
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
pub fn program(args: TokenStream, input: TokenStream) -> TokenStream {
    core::program::program(args.into(), input.into())
        .unwrap()
        .into()
}

#[proc_macro_derive(Arbitrary, attributes(osec))]
pub fn derive_arbitrary(item: TokenStream) -> TokenStream {
    core::arbitrary::arbitrary(item.into()).unwrap().into()
}

#[proc_macro]
pub fn verify_unpackable(types: TokenStream) -> TokenStream {
    core::unpackable::unpackable(types.into()).unwrap().into()
}

#[cfg(feature = "verify")]
#[proc_macro_attribute]
pub fn verify(args: TokenStream, input: TokenStream) -> TokenStream {
    core::verify::verify(args.into(), input.into())
        .expect("verify used on non-function?")
        .into()
}
