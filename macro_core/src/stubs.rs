use proc_macro2::{Group, Ident, Span, TokenStream};
use quote::{quote, ToTokens, format_ident};

fn stubs_mod_name(name: &Ident) -> Ident {
    format_ident!("{}_stubs", name)
}

static KLEND_NAME: &'static str = "kamino_lending";

pub fn stubs_def(name: &Ident) -> TokenStream {
    if name != KLEND_NAME {
        return quote! {};
    }

    let mod_name = stubs_mod_name(name);
    quote! {
        mod #mod_name {
            fn div_mod(x1: crate::utils::fraction::U256, x2: crate::utils::fraction::U256) -> (crate::utils::fraction::U256, crate::utils::fraction::U256) {
                // XXX: Does this hold for overflowing mul/div?

                let quo: crate::utils::fraction::U256 = kani::any();
                let rem: crate::utils::fraction::U256 = kani::any();

                // kani::assume(rem < x2);
                // kani::assume(quo.overflowing_mul(x2).0.overflowing_add(rem).0 == x1);

                (quo, rem)
            }

            fn cpi_set_stake_delegated(
                accounts_ctx: &crate::handlers::RefreshObligationFarmsForReserveBase,
                reserve: &crate::state::Reserve,
                mode: crate::state::ReserveFarmKind,
                amount: u64
            ) -> Result<(), anchor_lang::Error> {
                Ok(())
            }
        }
    }
}
pub fn stubs_attr(name: &Ident) -> TokenStream {
    if name != KLEND_NAME {
        return quote! {};
    }

    let mod_name = stubs_mod_name(name);
    quote! {
        #[kani::stub(crate::utils::fraction::U256::div_mod, #mod_name::div_mod)]
        #[kani::stub(crate::lending_market::farms_ixs::cpi_set_stake_delegated, #mod_name::cpi_set_stake_delegated)]
    }
}
