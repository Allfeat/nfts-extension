#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct CreateInput<AccountId, Price, BlockNumber, CollectionId> {
    pub admin: AccountId,
    pub config: CollectionConfigExt<Price, BlockNumber, CollectionId>, //pub config: CollectionConfigExt<Price, BlockNumber, CollectionId>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
/// Mint type. Can the NFT be create by anyone, or only the creator of the collection,
/// or only by wallets that already hold an NFT from a certain collection?
/// The ownership of a privately minted NFT is still publicly visible.
pub enum MintTypeExt<CollectionId> {
    /// Only an `Issuer` could mint items.
    Issuer,
    /// Anyone could mint items.
    Public,
    // Only holders of items in specified collection could mint new items.
    HolderOf(CollectionId),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Origin {
    Caller,
    Address,
}

impl Default for Origin {
    fn default() -> Self {
        Self::Address
    }
}

#[macro_export]
macro_rules! select_origin {
    ($origin:expr, $account:expr) => {
        match $origin {
            Origin::Caller => return Ok(RetVal::Converging(NftsError::OriginCannotBeCaller as u32)),
            Origin::Address => RawOrigin::Signed($account),
        }
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct MintSettingsExt<Price, BlockNumber, CollectionId> {
    /// Whether anyone can mint or if minters are restricted to some subset.
    pub mint_type: MintTypeExt<CollectionId>,
    /// An optional price per mint.
    pub price: Option<Price>,
    /// When the mint starts.
    pub start_block: Option<BlockNumber>,
    /// When the mint ends.
    pub end_block: Option<BlockNumber>,
    /// Default settings each item will get during the mint.
    pub default_item_settings: ItemSettingsExt,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct ItemSettingsExt {
    /// This item is transferable.
    pub transferable: bool,
    /// The metadata of this item can be modified.
    pub unlocked_metadata: bool,
    /// Attributes of this item can be modified.
    pub unlocked_attributes: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct CollectionSettingsExt {
    /// Items in this collection are transferable.
    pub transferable_items: bool,
    /// The metadata of this collection can be modified.
    pub unlocked_metadata: bool,
    /// Attributes of this collection can be modified.
    pub unlocked_attributes: bool,
    /// The supply of this collection can be modified.
    pub unlocked_max_supply: bool,
    /// When this isn't set then the deposit is required to hold the items of this collection.
    pub deposit_required: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct CollectionConfigExt<Price, BlockNumber, CollectionId> {
    /// Collection's settings.
    pub setting: CollectionSettingsExt,
    /// Collection's max supply.
    pub max_supply: Option<u32>,
    /// Default settings each item will get during the mint.
    pub mint_settings: MintSettingsExt<Price, BlockNumber, CollectionId>,
}
