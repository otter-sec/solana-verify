use super::pubkey::Pubkey;
/// The static program ID.
pub static ID: Pubkey = Pubkey { t: [0x42] };

/// Returns `true` if given pubkey is the program ID.
pub fn check_id(id: &Pubkey) -> bool {
    id == &ID
}

/// Returns the program ID.
pub fn id() -> Pubkey {
    ID
}
