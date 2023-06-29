#![cfg_attr(not(feature = "std"), no_std)]

pub mod errors;
pub mod types;

pub use nfts_extension_types::{CollectionConfigExt, Origin};

use crate::errors::NftsError;
use crate::types::{DefaultCollectionDetailsExt, DefaultCreateInput};
use ink::env::{DefaultEnvironment, Environment};
use scale::{Decode, Encode};

pub type AccountId = <DefaultEnvironment as Environment>::AccountId;
pub type Balance = <DefaultEnvironment as Environment>::Balance;
pub type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
pub type CollectionId = u32;

pub struct NftsExtension;

impl NftsExtension {
    // Getters
    /// Query the collection details of a specified ID
    /// TODO: make the input id as an Option and if None, query all the collections ?
    pub fn get_collection(id: CollectionId) -> Option<DefaultCollectionDetailsExt> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0002u32)
            .input::<CollectionId>()
            .output::<Option<DefaultCollectionDetailsExt>, false>()
            .ignore_error_code()
            .call(&id)
    }

    /// Calls create() in the pallet-assets
    pub fn create(input: DefaultCreateInput) -> Result<(), NftsError> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0001u32)
            .input::<DefaultCreateInput>()
            .output::<Result<(), NftsError>, true>()
            .handle_error_code::<NftsError>()
            .call(&input)
    }
}
