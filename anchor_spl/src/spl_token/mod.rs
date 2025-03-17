pub mod state;
pub mod instruction;

use onchor::solana_program::pubkey::Pubkey;
use onchor::solana_program::pubkey;

pub fn id() -> Pubkey {
    pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
}
