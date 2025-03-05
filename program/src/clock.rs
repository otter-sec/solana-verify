use crate::error::Error;

pub type Slot = u64;

pub type UnixTimestamp = i64;

/// The expected duration of a slot (400 milliseconds).
pub const DEFAULT_MS_PER_SLOT: u64 = 400;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Clock {
    pub slot: Slot,
    pub unix_timestamp: i64,
}

impl Clock {
    pub fn get() -> Result<Self, Error> {
        let secs = 1697536229i64;
        #[cfg(feature = "kani")]
        {
            let slot = kani::any();
            Ok(Self {
                slot,
                unix_timestamp: secs,
            })
        }
        #[cfg(not(feature = "kani"))]
        Ok(Self {
            slot: 0,
            unix_timestamp: secs,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_time() {
        let x = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        println!("{:?}", x);
    }
}
