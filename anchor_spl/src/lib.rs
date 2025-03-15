pub mod token_2022;
pub mod token_interface;
pub mod token;
pub mod spl_token_2022;
pub mod spl_token;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
