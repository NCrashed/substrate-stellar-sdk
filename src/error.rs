use base64::DecodeError;
use hex::FromHexError;
use thiserror::Error;
use crate::types::SignatureHint;

#[cfg(feature = "offchain")]
use crate::horizon::FetchError;

#[derive(Debug, Error)]
pub enum StellarSdkError {
    #[error("Invalide base32 encoding at {at_position}")]
    InvalidBase32Character {
        at_position: usize,
    },

    /// The encoding can be decoded but is not the canonical encoding of the underlying binary key
    #[error("The encoding can be decoded but is not the canonical encoding of the underlying binary key")]
    InvalidStellarKeyEncoding,

    /// The encoding has an invalid length
    #[error("The encoding has an invalid length")]
    InvalidStellarKeyEncodingLength,

    /// The initial version byte is invalid for this `EncodableKey`
    #[error("The initial version byte is invalid for this `EncodableKey`")]
    InvalidStellarKeyEncodingVersion {
        expected_version: char,
        found_version: char,
    },

    /// The checksum in the encoding is invaliid
    #[error("The checksum {found} in the encoding is invalid, expected {expected}")]
    InvalidStellarKeyChecksum {
        expected: u16,
        found: u16,
    },

    /// The signature has an invalid length
    #[error("The signature has an invalid length {found_length}, expected {expected_length}")]
    InvalidSignatureLength {
        found_length: usize,
        expected_length: usize,
    },
    /// Verification for this public key failed
    #[error("Verification for this public key failed")]
    PublicKeyCantVerify,

    /// The base64 encoding of the signature is invalid
    #[error("The base64 encoding of the signature is invalid")]
    InvalidBase64Encoding(DecodeError),

    /// The transaction envelope already has the maximal number of signatures (20)
    #[error("The transaction envelope already has the maximal number of signatures (20)")]
    TooManySignatures,

    /// The public key is not known as signer of the transaction
    #[error("The public key {0} is not known as signer of the transaction")]
    UnknownSignerKey(String),

    #[error("Asset code too long")]
    AssetCodeTooLong,

    #[error("Invalid asset code character")]
    InvalidAssetCodeCharacter,

    #[error("Exceeds max length, allowed {allowed_length}, request {requested_length}")]
    ExceedsMaximumLength {
        requested_length: usize,
        allowed_length: i32,
    },

    #[error("Invalid hex encoding")]
    InvalidHexEncoding(FromHexError),

    #[error("Invalid hash length, found {found_length}, expected {expected_length}")]    
    InvalidHashLength {
        found_length: usize,
        expected_length: usize,
    },

    #[error("Not approximable as fraction")] 
    NotApproximableAsFraction,

    #[error("Invalid price")] 
    InvalidPrice,

    #[error("Invalid trust line limit")] 
    InvalidTrustLineLimit,

    #[error("Invalid authorize flag")] 
    InvalidAuthorizeFlag,

    #[error("Invalid amount string")] 
    InvalidAmountString,

    #[error("Amount overflow")] 
    AmountOverflow,

    #[error("Amount negative")] 
    AmountNegative,

    #[error("Amount non positive")] 
    AmountNonPositive,

    #[error("Invalid binary length, expected {expected_length}, {found_length}")] 
    InvalidBinaryLength {
        found_length: usize,
        expected_length: usize,
    },

    #[error("Invalid balance id")]
    InvalidBalanceId,

    #[error("Empty claimants")]
    EmptyClaimants,

    #[error("Invalid signer weight")]
    InvalidSignerWeight,

    #[error("Cant wrap fee bump transaction")]
    CantWrapFeeBumpTransaction,

    #[cfg(feature = "offchain")]
    #[error("Fetch error")]
    FetchError(FetchError)
}
