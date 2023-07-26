pub const ACCOUNT_STORAGE_OVERHEAD: u64 = 128;

pub struct Rent {
    /// Rental rate
    pub lamports_per_byte_year: u64,

    /// exemption threshold, in years
    pub exemption_threshold: f64,

    // What portion of collected rent are to be destroyed, percentage-wise
    pub burn_percent: u8,
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
