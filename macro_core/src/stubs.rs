use proc_macro2::{Ident, TokenStream};
use quote::{quote, format_ident};

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

            fn str_from_utf8(v: &[u8]) -> Result<&str, core::str::Utf8Error> {
                kani::assume(v.iter().all(|c| c.is_ascii()));
                // SAFETY: We check above that the string is ASCII
                unsafe {
                    Ok(core::str::from_utf8_unchecked(v))
                }
            }

            fn memchr(x: u8, text: &[u8]) -> Option<usize> {
                let mut i = 0;

                kani::assume(text.len() < 32);
                while i < text.len() {
                    if text[i] == x {
                        return Some(i);
                    }
                    i += 1;
                }

                None
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
        #[kani::stub(core::str::from_utf8, #mod_name::str_from_utf8)]
        #[kani::stub(std::str::from_utf8, #mod_name::str_from_utf8)]
        #[kani::stub(core::slice::memchr::memchr, #mod_name::memchr)]
    }
}
