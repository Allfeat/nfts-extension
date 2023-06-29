#![cfg_attr(not(feature = "std"), no_std)]

mod errors;

use frame_support::traits::Currency;
use pallet_contracts::chain_extension::{
    ChainExtension, Environment, Ext, InitState, RetVal, SysConfig,
};
use pallet_nfts::{
    weights::WeightInfo, CollectionConfig, CollectionSetting, CollectionSettings, ItemSetting,
    MintSettings,
};
use sp_runtime::traits::StaticLookup;
use sp_runtime::{DispatchError, SaturatedConversion};

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::Get;

use crate::errors::NftsError;
use nfts_extension_types::CollectionConfigExt;
use sp_std::marker::PhantomData;

use pallet_contracts::RawOrigin;

enum NftsFunc {
    Create,
    GetCollection,
}

impl TryFrom<u16> for NftsFunc {
    type Error = DispatchError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(NftsFunc::Create),
            2 => Ok(NftsFunc::GetCollection),
            _ => Err(DispatchError::Other(
                "PalletNftsExtension: Unimplemented func_id",
            )),
        }
    }
}

/// Pallet Assets chain extension.
pub struct NftsExtension<T>(PhantomData<T>);

impl<T> Default for NftsExtension<T> {
    fn default() -> Self {
        NftsExtension(PhantomData)
    }
}

impl<T> ChainExtension<T> for NftsExtension<T>
where
    T: pallet_contracts::Config + pallet_nfts::Config,
    <<T as SysConfig>::Lookup as StaticLookup>::Source: From<<T as SysConfig>::AccountId>,
    <T as SysConfig>::AccountId: From<[u8; 32]>,
    <T as SysConfig>::RuntimeOrigin: From<RawOrigin<<T as SysConfig>::AccountId>>,
{
    fn call<E: Ext<T = T>>(
        &mut self,
        env: Environment<E, InitState>,
    ) -> Result<RetVal, DispatchError> {
        let func_id: NftsFunc = env.func_id().try_into()?;
        let mut env = env.buf_in_buf_out();

        match func_id {
            NftsFunc::Create => {
                let args: CreateInput<NftsBalanceOf<T>, T::BlockNumber, T::CollectionId> =
                    env.read_as()?;
                let admin: T::AccountId = args.admin.into();
                //let CollectionConfigWrapperFor::<T>(config) = args.config.into();

                let base_weight = <T as pallet_nfts::Config>::WeightInfo::create();
                env.charge_weight(base_weight)?;

                let caller = env.ext().address().clone();
                let call_result = pallet_nfts::Pallet::<T>::create(
                    RawOrigin::Contract(caller).into(),
                    admin.into(),
                    CollectionConfig {
                        settings: Default::default(),
                        max_supply: None,
                        mint_settings: Default::default(),
                    },
                );
                return match call_result {
                    Err(e) => {
                        let mapped_error = NftsError::try_from(e)?;
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(NftsError::Success as u32)),
                };
            }

            NftsFunc::GetCollection => {
                let id: T::CollectionId = env.read_as()?;

                let base_weight = <T as frame_system::Config>::DbWeight::get().reads(1);
                env.charge_weight(base_weight)?;

                let collection_details = pallet_nfts::Collection::<T>::get(id);
                env.write(&collection_details.encode(), false, None)?;
            }
        };

        Ok(RetVal::Converging(NftsError::Success as u32))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
struct CreateInput<Price, BlockNumber, CollectionId> {
    pub admin: [u8; 32],
    pub config: CollectionConfigExt<Price, BlockNumber, CollectionId>, //pub config: CollectionConfigExt<P, B, C>,
}

type NftsBalanceOf<T> = <<T as pallet_nfts::Config>::Currency as Currency<
    <T as frame_system::Config>::AccountId,
>>::Balance;

type CollectionConfigFor<T> = CollectionConfig<
    NftsBalanceOf<T>,
    <T as frame_system::Config>::BlockNumber,
    <T as pallet_nfts::Config>::CollectionId,
>;
type CollectionConfigExtFor<T> = CollectionConfigExt<
    NftsBalanceOf<T>,
    <T as frame_system::Config>::BlockNumber,
    <T as pallet_nfts::Config>::CollectionId,
>;

/// Wrapper to implement From trait and convert Extension types to original types of the nfts pallet
struct CollectionConfigWrapperFor<T: pallet_nfts::Config>(CollectionConfigFor<T>);
impl<T: pallet_nfts::Config> From<CollectionConfigExtFor<T>> for CollectionConfigWrapperFor<T> {
    fn from(value: CollectionConfigExtFor<T>) -> Self {
        let mut settings = CollectionSettings::default();
        if value.setting.transferable_items {
            settings.0.insert(CollectionSetting::TransferableItems)
        }
        if value.setting.unlocked_metadata {
            settings.0.insert(CollectionSetting::UnlockedMetadata)
        }
        if value.setting.unlocked_attributes {
            settings.0.insert(CollectionSetting::UnlockedAttributes)
        }
        if value.setting.unlocked_max_supply {
            settings.0.insert(CollectionSetting::UnlockedMaxSupply)
        }
        if value.setting.deposit_required {
            settings.0.insert(CollectionSetting::DepositRequired)
        }

        let mut mint_settings = MintSettings::<
            <<T as pallet_nfts::Config>::Currency as Currency<
                <T as frame_system::Config>::AccountId,
            >>::Balance,
            T::BlockNumber,
            T::CollectionId,
        >::default();
        // mint_settings.mint_type = match value.mint_settings.mint_type {
        //     MintTypeExt::Issuer => MintType::Issuer,
        //     MintTypeExt::Public => MintType::Public,
        //     MintTypeExt::HolderOf(_) => MintType::HolderOf(T::CollectionId::initial_value()),
        // };
        mint_settings.price = match value.mint_settings.price {
            Some(x) => Some(x.saturated_into()),
            None => None,
        };
        mint_settings.start_block = match value.mint_settings.start_block {
            Some(x) => Some(x.saturated_into()),
            None => None,
        };
        mint_settings.end_block = match value.mint_settings.end_block {
            Some(x) => Some(x.saturated_into()),
            None => None,
        };
        if value.mint_settings.default_item_settings.transferable {
            mint_settings
                .default_item_settings
                .0
                .insert(ItemSetting::Transferable)
        }
        if value.mint_settings.default_item_settings.unlocked_metadata {
            mint_settings
                .default_item_settings
                .0
                .insert(ItemSetting::UnlockedMetadata)
        }
        if value
            .mint_settings
            .default_item_settings
            .unlocked_attributes
        {
            mint_settings
                .default_item_settings
                .0
                .insert(ItemSetting::UnlockedAttributes)
        }

        let config = CollectionConfigFor::<T> {
            settings,
            max_supply: value.max_supply,
            mint_settings,
        };
        CollectionConfigWrapperFor(config)
    }
}
