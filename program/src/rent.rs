use crate::error::Error;

pub const ACCOUNT_STORAGE_OVERHEAD: u64 = 128;
pub const DEFAULT_LAMPORTS_PER_BYTE_YEAR: u64 = 1_000_000_000 / 100 * 365 / (1024 * 1024);
pub const DEFAULT_EXEMPTION_THRESHOLD: f64 = 2.0;
pub const DEFAULT_BURN_PERCENT: u8 = 50;

/// Account storage overhead for calculation of

#[derive(Clone, Copy, Debug)]
pub struct Rent {
    /// Rental rate
    pub lamports_per_byte_year: u64,

    /// exemption threshold, in years
    pub exemption_threshold: f64,

    // What portion of collected rent are to be destroyed, percentage-wise
    pub burn_percent: u8,
}

impl Default for Rent {
    fn default() -> Self {
        Self {
            lamports_per_byte_year: DEFAULT_LAMPORTS_PER_BYTE_YEAR,
            exemption_threshold: DEFAULT_EXEMPTION_THRESHOLD,
            burn_percent: DEFAULT_BURN_PERCENT,
        }
    }
}

impl Rent {
    pub fn minimum_balance(&self, data_len: usize) -> u64 {
        let bytes = data_len as u64;
        (((ACCOUNT_STORAGE_OVERHEAD + bytes) * self.lamports_per_byte_year) as f64
            * self.exemption_threshold) as u64
    }

    pub fn is_exempt(&self, balance: u64, data_len: usize) -> bool {
        balance >= self.minimum_balance(data_len)
    }

    pub fn get() -> Result<Self, Error> {
        Ok(Self::default())
    }
}

#[cfg(any(kani, feature = "kani"))]
impl kani::Arbitrary for Rent {
    fn any() -> Self {
        Self {
            lamports_per_byte_year: kani::any(),
            exemption_threshold: kani::any(),
            burn_percent: kani::any(),
        }
    }
}
