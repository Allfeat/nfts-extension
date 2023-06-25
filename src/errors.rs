use super::{Decode, Encode};
use ink::env::chain_extension::FromStatusCode;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum NftsError {
    NoPermission,
    UnknownCollection,
    AlreadyExists,
    ApprovalExpired,
    WrongOwner,
    BadWitness,
    CollectionIdInUse,
    ItemsNonTransferable,
    NotDelegate,
    WrongDelegate,
    Unapproved,
    Unaccepted,
    ItemLocked,
    LockedItemAttributes,
    LockedCollectionAttributes,
    LockedItemMetadata,
    LockedCollectionMetadata,
    MaxSupplyReached,
    MaxSupplyLocked,
    MaxSupplyTooSmall,
    UnknownItem,
    UnknownSwap,
    MetadataNotFound,
    AttributeNotFound,
    NotForSale,
    BidTooLow,
    ReachedApprovalLimit,
    DeadlineExpired,
    WrongDuration,
    MethodDisabled,
    WrongSetting,
    InconsistentItemConfig,
    NoConfig,
    RolesNotCleared,
    MintNotStarted,
    MintEnded,
    AlreadyClaimed,
    IncorrectData,
    WrongOrigin,
    WrongSignature,
    IncorrectMetadata,
    MaxAttributesLimitReached,
    WrongNamespace,
    CollectionNotEmpty,
    OriginCannotBeCaller,
}

impl FromStatusCode for NftsError {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            _ => panic!("encountered unknown status code"),
        }
    }
}

impl From<scale::Error> for NftsError {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}
