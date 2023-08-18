use borsh::{BorshDeserialize, BorshSerialize};
pub const PUBKEY_BYTES: usize = 1;

#[derive(
    PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Default, Debug, BorshSerialize, BorshDeserialize,
)]
pub struct Pubkey {
    pub t: [u8; PUBKEY_BYTES],
}

impl Pubkey {
    pub fn new(k: &[u8]) -> Pubkey {
        Pubkey {
            t: <[u8; PUBKEY_BYTES]>::try_from(k)
                .expect("Slice must be the same length as a Pubkey"),
        }
    }

    pub fn new_from_array(arr: [u8; PUBKEY_BYTES]) -> Pubkey {
        Pubkey { t: arr }
    }

    pub fn to_bytes(&self) -> [u8; PUBKEY_BYTES] {
        self.t
    }

    pub fn key(&self) -> Self {
        *self
    }
}

impl Default for &Pubkey {
    fn default() -> Self {
        &Pubkey {
            t: [0; PUBKEY_BYTES],
        }
    }
}

#[cfg(any(kani, feature = "kani"))]
impl Pubkey {
    pub fn find_program_address(_seeds: &[&[u8]], _program_id: &Pubkey) -> (Pubkey, u8) {
        (kani::any(), kani::any())
    }
}

impl AsRef<[u8]> for Pubkey {
    fn as_ref(&self) -> &[u8] {
        &self.t
    }
}

#[cfg(any(kani, feature = "kani"))]
impl kani::Arbitrary for Pubkey {
    fn any() -> Self {
        Self { t: [kani::any()] }
    }
}

const MAX_KEYS: usize = 100;
pub static mut KEYS: [Pubkey; MAX_KEYS] = [Pubkey { t: [0] }; MAX_KEYS];
pub static mut KEYS_IDX: usize = 0;

#[cfg(any(kani, feature = "kani"))]
pub fn kani_new_pubkey() -> &'static Pubkey {
    unsafe {
        kani::assert(
            KEYS_IDX < MAX_KEYS,
            "Ran out of keys during context creation.",
        );
        let p: Pubkey = kani::any();
        KEYS[KEYS_IDX] = p;
        KEYS_IDX += 1;
        KEYS.get(KEYS_IDX - 1).unwrap()
    }
}
