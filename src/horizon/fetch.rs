use core::num::{ParseFloatError, ParseIntError};
use sp_std::{str, vec, vec::Vec};
use thiserror::Error; 
use ureq::{Agent, AgentBuilder};
use std::time::Duration;

use core::convert::TryInto;

use crate::{AccountId, IntoAccountId, StellarSdkError};

use super::{
    api_response_types::FeeStats, json_response_types, Horizon, HTTP_HEADER_CLIENT_NAME,
    HTTP_HEADER_CLIENT_VERSION,
};

impl From<ParseIntError> for FetchError {
    fn from(error: ParseIntError) -> Self {
        FetchError::ParseIntError(error)
    }
}

impl From<ParseFloatError> for FetchError {
    fn from(error: ParseFloatError) -> Self {
        FetchError::ParseFloatError(error)
    }
}

#[derive(Debug, Error)]
pub enum FetchError {
    #[error("Timeout reached")]
    DeadlineReached,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Network error: {0}")]
    NetworkError(#[from] ureq::Error),
    #[error("Invalid request")]
    Invalid,
    #[error("Unknown error")]
    Unknown,
    #[error("Unexpected response status {status}")]
    UnexpectedResponseStatus { status: u16, body: Vec<u8> },
    #[error("Failed to parse response JSON")]
    JsonParseError,
    #[error("Invalid sequence number")]
    InvalidSequenceNumber,
    #[error("Failed to parse integer: {0}")]
    ParseIntError(ParseIntError),
    #[error("Failed to parse float: {0}")]
    ParseFloatError(ParseFloatError),
    #[error("Account required memo {0:?}")]
    AccountRequiredMemo(AccountId),
}

impl From<FetchError> for StellarSdkError {
    fn from(error: FetchError) -> Self {
        StellarSdkError::FetchError(error)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(_error: serde_json::Error) -> Self {
        FetchError::JsonParseError
    }
}

pub enum Method {
    Get, 
    Post
}

fn read_body(response: ureq::Response) -> Result<Vec<u8>, std::io::Error> {
    use std::io::Read;
    let mut buf: Vec<u8> = vec![];
    response.into_reader().read_to_end(&mut buf)?;
    Ok(buf)
}

impl Horizon {
    pub fn request(
        &self,
        path: Vec<&[u8]>,
        method: Method,
        timeout_milliseconds: u64,
    ) -> Result<Vec<u8>, FetchError> {
        let mut url = self.base_url.clone();
        for path_segment in path {
            url.extend_from_slice(path_segment);
        }
        let final_url = str::from_utf8(&url).unwrap();
        let requester = match method {
            Method::Get => self.agent.get(final_url),
            Method::Post => self.agent.post(final_url),
        };
        let response = match requester.call() {
            Ok(response) => response, 
            Err(ureq::Error::Status(code, response)) => {
                return Err(FetchError::UnexpectedResponseStatus {
                    status: code,
                    body: read_body(response)?,
                });
            }
            Err(e) => Err(e)?,
        };

        Ok(read_body(response)?)
    }

    /// Fetch the sequence number of an account
    ///
    /// The sequence number is defined to be of type [i64](https://github.com/stellar/stellar-core/blob/master/src/xdr/Stellar-ledger-entries.x)
    pub fn fetch_fee_stats(&self, timeout_milliseconds: u64) -> Result<FeeStats, FetchError> {
        let json = self.request(vec![b"/fee_stats"], Method::Get, timeout_milliseconds)?;

        let response: json_response_types::FeeStats = serde_json::from_slice(&json)?;

        response.try_into()
    }

    /// Fetch the sequence number of an account
    ///
    /// The sequence number is defined to be of type [i64](https://github.com/stellar/stellar-core/blob/master/src/xdr/Stellar-ledger-entries.x)
    pub fn fetch_account<T: IntoAccountId>(
        &self,
        account_id: T,
        timeout_milliseconds: u64,
    ) -> Result<json_response_types::AccountResponse, FetchError> {
        let json = self.request(
            vec![b"/accounts/", account_id.into_encoding().as_slice()],
            Method::Get,
            timeout_milliseconds,
        )?;

        let account_response: json_response_types::AccountResponse = serde_json::from_slice(&json)?;

        Ok(account_response)
    }

    /// Fetch the sequence number of an account
    ///
    /// The sequence number is defined to be of type [i64](https://github.com/stellar/stellar-core/blob/master/src/xdr/Stellar-ledger-entries.x)
    pub fn fetch_next_sequence_number<T: IntoAccountId>(
        &self,
        account_id: T,
        timeout_milliseconds: u64,
    ) -> Result<i64, FetchError> {
        let account_response = self.fetch_account(account_id, timeout_milliseconds)?;

        let sequence_number: i64 = match account_response.sequence.parse() {
            Ok(n) => n,
            Err(_) => return Err(FetchError::InvalidSequenceNumber),
        };
        let next_sequence_number = sequence_number + 1;
        Ok(next_sequence_number)
    }
}
