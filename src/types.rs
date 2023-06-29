use super::*;
use nfts_extension_types::CreateInput;

pub type DefaultCollectionConfigExt = CollectionConfigExt<Balance, BlockNumber, CollectionId>;
pub type DefaultCreateInput = CreateInput<AccountId, Balance, BlockNumber, CollectionId>;
