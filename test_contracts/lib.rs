#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod test_contracts {
    use nfts_extension::errors::NftsError;
    use nfts_extension::types::{
        DefaultCollectionConfigExt, DefaultCollectionDetailsExt, DefaultCreateInput,
    };
    use nfts_extension::*;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Mock;

    impl Mock {
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            Default::default()
        }

        // Constants query
        #[ink(message)]
        pub fn get_approval_limits(&mut self) -> u32 {
            NftsExtension::get_approvals_limit()
        }
        #[ink(message)]
        pub fn get_attribute_deposit_base(&mut self) -> Balance {
            NftsExtension::get_attribute_deposit_base()
        }
        #[ink(message)]
        pub fn get_collection_deposit(&mut self) -> Balance {
            NftsExtension::get_collection_deposit()
        }
        #[ink(message)]
        pub fn get_deposit_per_bytet(&mut self) -> Balance {
            NftsExtension::get_deposit_per_byte()
        }

        // Chain state query
        #[ink(message)]
        pub fn get_collection(&mut self, id: CollectionId) -> Option<DefaultCollectionDetailsExt> {
            NftsExtension::get_collection(id)
        }

        #[ink(message, payable)]
        pub fn create(
            &mut self,
            admin: AccountId,
            config: DefaultCollectionConfigExt, // config: DefaultCollectionConfigExt,
        ) -> Result<(), NftsError> {
            NftsExtension::create(DefaultCreateInput { admin, config })?;
            Ok(())
        }
    }

    /*#[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink::env::test::default_accounts;
        use ink_e2e::build_message;
        use nfts_extension::types::DefaultCollectionConfigExt;
        use nfts_extension::AccountId;
        use nfts_extension_types::{
            CollectionSettingsExt, ItemSettingsExt, MintSettingsExt, MintTypeExt,
        };
        use std::error::Error;

        type E2EResult<T> = std::result::Result<T, Box<dyn Error>>;

        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // given
            let constructor = MockRef::new();

            // when
            let contract_acc_id = client
                .instantiate("nft_mock", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let admin = default_accounts().bob;
            let config = DefaultCollectionConfigExt {
                setting: CollectionSettingsExt {
                    transferable_items: true,
                    unlocked_metadata: true,
                    unlocked_attributes: true,
                    unlocked_max_supply: true,
                    deposit_required: true,
                },
                max_supply: Some(10000000000),
                mint_settings: MintSettingsExt {
                    mint_type: MintTypeExt::Issuer,
                    price: None,
                    start_block: None,
                    end_block: None,
                    default_item_settings: ItemSettingsExt {
                        transferable: true,
                        unlocked_metadata: true,
                        unlocked_attributes: true,
                    },
                },
            };

            // then
            let create = build_message::<MockRef>(contract_acc_id)
                .call(|contract| contract.create(admin, config));
            Ok(())
        }*/

    //#[ink_e2e::test]
    //async fn create_test(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {}
}
