use codec::{Decode, Encode};
use sp_runtime::{DispatchError, ModuleError};

#[derive(PartialEq, Eq, Copy, Clone, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum NftsError {
    /// Success
    Success = 0,
    /// The signing account has no permission to do the operation.
    NoPermission = 2,
    /// The given item ID is unknown.
    UnknownCollection = 3,
    /// The item ID has already been used for an item.
    AlreadyExists = 4,
    /// The approval had a deadline that expired, so the approval isn't valid anymore.
    ApprovalExpired = 5,
    /// The owner turned out to be different to what was expected.
    WrongOwner = 6,
    /// The witness data given does not match the current state of the chain.
    BadWitness = 7,
    /// Collection ID is already taken.
    CollectionIdInUse = 8,
    /// Items within that collection are non-transferable.
    ItemsNonTransferable = 9,
    /// The provided account is not a delegate.
    NotDelegate = 10,
    /// The delegate turned out to be different to what was expected.
    WrongDelegate = 11,
    /// No approval exists that would allow the transfer.
    Unapproved = 12,
    /// The named owner has not signed ownership acceptance of the collection.
    Unaccepted = 13,
    /// The item is locked (non-transferable).
    ItemLocked = 14,
    /// Item's attributes are locked.
    LockedItemAttributes = 15,
    /// Collection's attributes are locked.
    LockedCollectionAttributes = 16,
    /// Item's metadata is locked.
    LockedItemMetadata = 17,
    /// Collection's metadata is locked.
    LockedCollectionMetadata = 18,
    /// All items have been minted.
    MaxSupplyReached = 19,
    /// The max supply is locked and can't be changed.
    MaxSupplyLocked = 20,
    /// The provided max supply is less than the number of items a collection already has.
    MaxSupplyTooSmall = 21,
    /// The given item ID is unknown.
    UnknownItem = 22,
    /// Swap doesn't exist.
    UnknownSwap = 23,
    /// The given item has no metadata set.
    MetadataNotFound = 24,
    /// The provided attribute can't be found.
    AttributeNotFound = 25,
    /// Item is not for sale.
    NotForSale = 26,
    /// The provided bid is too low.
    BidTooLow = 27,
    /// The item has reached its approval limit.
    ReachedApprovalLimit = 28,
    /// The deadline has already expired.
    DeadlineExpired = 29,
    /// The duration provided should be less than or equal to `MaxDeadlineDuration`.
    WrongDuration = 30,
    /// The method is disabled by system settings.
    MethodDisabled = 31,
    /// The provided setting can't be set.
    WrongSetting = 32,
    /// Item's config already exists and should be equal to the provided one.
    InconsistentItemConfig = 33,
    /// Config for a collection or an item can't be found.
    NoConfig = 34,
    /// Some roles were not cleared.
    RolesNotCleared = 35,
    /// Mint has not started yet.
    MintNotStarted = 36,
    /// Mint has already ended.
    MintEnded = 37,
    /// The provided Item was already used for claiming.
    AlreadyClaimed = 38,
    /// The provided data is incorrect.
    IncorrectData = 39,
    /// The extrinsic was sent by the wrong origin.
    WrongOrigin = 40,
    /// The provided signature is incorrect.
    WrongSignature = 41,
    /// The provided metadata might be too long.
    IncorrectMetadata = 42,
    /// Can't set more attributes per one call.
    MaxAttributesLimitReached = 43,
    /// The provided namespace isn't supported in this call.
    WrongNamespace = 44,
    /// Can't delete non-empty collections.
    CollectionNotEmpty = 45,
    /// Unknown error
    UnknownError = 99,
}

// TODO: macro to make the implement not verbose that much ?
impl TryFrom<DispatchError> for NftsError {
    type Error = DispatchError;

    fn try_from(value: DispatchError) -> Result<Self, Self::Error> {
        let error_text = match value {
            DispatchError::Module(ModuleError { message, .. }) => message,
            _ => Some("No module error Info"),
        };
        return match error_text {
            Some("NoPermission") => Ok(NftsError::NoPermission),
            Some("UnknownCollection") => Ok(NftsError::UnknownCollection),
            Some("AlreadyExists") => Ok(NftsError::AlreadyExists),
            Some("ApprovalExpired") => Ok(NftsError::ApprovalExpired),
            Some("WrongOwner") => Ok(NftsError::WrongOwner),
            Some("BadWitness") => Ok(NftsError::BadWitness),
            Some("CollectionIdInUse") => Ok(NftsError::CollectionIdInUse),
            Some("ItemsNonTransferable") => Ok(NftsError::ItemsNonTransferable),
            Some("NotDelegate") => Ok(NftsError::NotDelegate),
            Some("WrongDelegate") => Ok(NftsError::WrongDelegate),
            Some("Unapproved") => Ok(NftsError::Unapproved),
            Some("Unaccepted") => Ok(NftsError::Unaccepted),
            Some("ItemLocked") => Ok(NftsError::ItemLocked),
            Some("LockedItemAttributes") => Ok(NftsError::LockedItemAttributes),
            Some("LockedCollectionAttributes") => Ok(NftsError::LockedCollectionAttributes),
            Some("LockedItemMetadata") => Ok(NftsError::LockedItemMetadata),
            Some("LockedCollectionMetadata") => Ok(NftsError::LockedCollectionMetadata),
            Some("MaxSupplyReached") => Ok(NftsError::MaxSupplyReached),
            Some("MaxSupplyLocked") => Ok(NftsError::MaxSupplyLocked),
            Some("MaxSupplyTooSmall") => Ok(NftsError::MaxSupplyTooSmall),
            Some("UnknownItem") => Ok(NftsError::UnknownItem),
            Some("UnknownSwap") => Ok(NftsError::UnknownSwap),
            Some("MetadataNotFound") => Ok(NftsError::MetadataNotFound),
            Some("AttributeNotFound") => Ok(NftsError::AttributeNotFound),
            Some("NotForSale") => Ok(NftsError::NotForSale),
            Some("BidTooLow") => Ok(NftsError::BidTooLow),
            Some("ReachedApprovalLimit") => Ok(NftsError::ReachedApprovalLimit),
            Some("DeadlineExpired") => Ok(NftsError::DeadlineExpired),
            Some("WrongDuration") => Ok(NftsError::WrongDuration),
            Some("MethodDisabled") => Ok(NftsError::MethodDisabled),
            Some("WrongSetting") => Ok(NftsError::WrongSetting),
            Some("InconsistentItemConfig") => Ok(NftsError::InconsistentItemConfig),
            Some("NoConfig") => Ok(NftsError::NoConfig),
            Some("RolesNotCleared") => Ok(NftsError::RolesNotCleared),
            Some("MintNotStarted") => Ok(NftsError::MintNotStarted),
            Some("MintEnded") => Ok(NftsError::MintEnded),
            Some("AlreadyClaimed") => Ok(NftsError::AlreadyClaimed),
            Some("IncorrectData") => Ok(NftsError::IncorrectData),
            Some("WrongOrigin") => Ok(NftsError::WrongOrigin),
            Some("WrongSignature") => Ok(NftsError::WrongSignature),
            Some("IncorrectMetadata") => Ok(NftsError::IncorrectMetadata),
            Some("MaxAttributesLimitReached") => Ok(NftsError::MaxAttributesLimitReached),
            Some("WrongNamespace") => Ok(NftsError::WrongNamespace),
            Some("CollectionNotEmpty") => Ok(NftsError::CollectionNotEmpty),
            _ => Ok(NftsError::UnknownError),
        };
    }
}
