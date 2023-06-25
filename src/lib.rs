#![cfg_attr(not(feature = "std"), no_std)]

pub mod errors;
pub mod types;

use crate::errors::NftsError;
use crate::types::CreateInput;
use ink::env::{DefaultEnvironment, Environment};
use scale::{Decode, Encode};

pub type AccountId = <DefaultEnvironment as Environment>::AccountId;
pub type Balance = <DefaultEnvironment as Environment>::Balance;
pub type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
pub type CollectionId = u64;

pub struct NftsExtension;

impl NftsExtension {
    /// Calls create() in the pallet-assets
    pub fn create(input: CreateInput) -> Result<(), NftsError> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0001u32)
            .input::<CreateInput>()
            .output::<(), false>()
            .handle_error_code::<NftsError>()
            .call(&input)
    }
}
