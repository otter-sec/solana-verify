use crate::error::Error;

pub type Slot = u64;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Clock {
    pub unix_timestamp: i64,
}

impl Clock {
    pub fn get() -> Result<Self, Error> {
        let secs = 1697536229i64;
        Ok(Self {
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
