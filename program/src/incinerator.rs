// Docs: Lamports credited to this address will be
// removed from the total supply (burned) at the end
// of the current block.

use otter_solana_macro::declare_id;

use crate::pubkey::Pubkey;

declare_id!("1nc1nerator11111111111111111111111111111111");

pub fn check_id(pubkey: &Pubkey) -> bool {
    pubkey == &ID
}
