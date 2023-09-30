use crate::error::Error;

pub type Slot = u64;

pub static mut UNIX_TIMESTAMP: i64 = 0i64;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Clock {
    pub unix_timestamp: i64,
}

impl Clock {
    pub fn get() -> Result<Self, Error> {
        // let secs = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        unsafe {
            let secs = UNIX_TIMESTAMP;
            UNIX_TIMESTAMP += 1;
            Ok(Self {
                unix_timestamp: secs,
            })
        }
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
