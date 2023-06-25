#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod test_contracts {
    use nfts_extension::errors::NftsError;
    use nfts_extension::types::{CreateInput, DefaultCollectionConfigExt, Origin};
    use nfts_extension::*;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Mock;

    impl Mock {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message, payable)]
        pub fn create(
            &mut self,
            admin: AccountId,
            config: DefaultCollectionConfigExt,
        ) -> Result<(), NftsError> {
            NftsExtension::create(CreateInput {
                origin: Origin::Caller,
                admin: admin,
                config,
            })?;
            Ok(())
        }
    }
}
