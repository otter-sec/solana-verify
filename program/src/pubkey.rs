use std::{fmt::{self, Display, Formatter}, hash::Hash};

use borsh::{BorshDeserialize, BorshSerialize, BorshSchema};
use bytemuck;

pub const PUBKEY_BYTES: usize = 1;
pub const PUBKEY_PAD_BYTES: usize = 32 - PUBKEY_BYTES;

#[derive(
    Copy,
    PartialOrd,
    Ord,
    Default,
    Debug,
    BorshSerialize,
    BorshDeserialize,
    Hash,
    BorshSchema,
    bytemuck::Zeroable,
    bytemuck::Pod,
)]
#[repr(C)]
pub struct Pubkey {
    pub t: [u8; PUBKEY_BYTES],
    pub _padding: [u8; PUBKEY_PAD_BYTES],
}

impl Eq for Pubkey {}
impl PartialEq for Pubkey {
    fn eq(&self, other: &Self) -> bool {
        let mut res = true;
        let mut i = 0;
        while i < PUBKEY_BYTES {
            res &= self.t[i] == other.t[i];
            i += 1;
        }
        res
    }
}

impl Clone for Pubkey {
    fn clone(&self) -> Self { Self { t: self.t.clone(), _padding: [0; PUBKEY_PAD_BYTES] } }
}


impl Display for Pubkey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.t)
    }
}

impl Pubkey {
    pub fn new(k: &[u8]) -> Pubkey {
        Pubkey {
            t: <[u8; PUBKEY_BYTES]>::try_from(k)
                .expect("Slice must be the same length as a Pubkey"),
            _padding: Default::default()
        }
    }

    pub const fn new_from_array(pubkey_array: [u8; 32]) -> Self {
        Self{
            t: [pubkey_array[0]],
            _padding: [0; PUBKEY_PAD_BYTES]
        }
    }

    pub const fn new_from_array2(pubkey_array: [u8; 1]) -> Self {
        Self{
            t: [pubkey_array[0]],
            _padding: [0; PUBKEY_PAD_BYTES]
        }
    }

    pub fn to_bytes(&self) -> [u8; PUBKEY_BYTES] {
        self.t
    }

    pub fn key(&self) -> Self {
        *self
    }

    #[cfg(any(kani, feature = "kani"))]
    pub fn create_program_address(_seeds: &[&[u8]], _program_id: &Pubkey) -> Result<Pubkey, Box<dyn std::error::Error>> {
        Ok(kani::any())
    }
}

impl From<[u8; 32]> for Pubkey {
    fn from(x: [u8; 32]) -> Self {
        Self::new_from_array(x)
    }
}

#[cfg(not(any(kani, feature = "kani")))]
impl Pubkey {
    pub fn create_program_address(_seeds: &[&[u8]], _program_id: &Pubkey) -> Result<Pubkey, Box<dyn std::error::Error>> {
        Ok(Pubkey::default())
    }
}

impl Default for &Pubkey {
    fn default() -> Self {
        &Pubkey {
            t: [0; PUBKEY_BYTES],
            _padding: [0; PUBKEY_PAD_BYTES]
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
        Self { t: [kani::any()], _padding: unsafe { std::mem::zeroed() }}
    }
}

const MAX_KEYS: usize = 100;
pub static mut KEYS: [Pubkey; MAX_KEYS] = [Pubkey { t: [0], _padding: unsafe { std::mem::zeroed() } }; MAX_KEYS];
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
