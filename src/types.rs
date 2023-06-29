use super::*;
use nfts_extension_types::{CollectionDetailsExt, CreateInput};

pub type DefaultCollectionConfigExt = CollectionConfigExt<Balance, BlockNumber, CollectionId>;
pub type DefaultCreateInput = CreateInput<AccountId, Balance, BlockNumber, CollectionId>;
pub type DefaultCollectionDetailsExt = CollectionDetailsExt<AccountId, Balance>;
