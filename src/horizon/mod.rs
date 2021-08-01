use sp_std::vec::Vec;
use ureq::{Agent, AgentBuilder};
use std::time::Duration;

pub mod json_response_types;

pub mod api_response_types;
pub mod fetch;
pub mod submit_transaction;

pub struct Horizon {
    base_url: Vec<u8>,
    agent: Agent,
}

pub use fetch::FetchError;

impl Horizon {
    pub fn new(base_url: &str) -> Horizon {
        let agent = AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .user_agent(&format!("{}/{}", HTTP_HEADER_CLIENT_NAME, HTTP_HEADER_CLIENT_VERSION))
            .build();
        Horizon {
            base_url: base_url.as_bytes().to_vec(),
            agent,
        }
    }
}

const HTTP_HEADER_CLIENT_NAME: &str = "substrate-stellar-sdk";
const HTTP_HEADER_CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");
