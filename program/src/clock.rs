use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::Error;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Clock {
    pub unix_timestamp: i64,
}

impl Clock {
    pub fn get() -> Result<Self, Error> {
        let secs = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        Ok(Self {
            unix_timestamp: secs as i64,
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
