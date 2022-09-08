// use near_sdk::test_utils::{accounts, VMContextBuilder};
// use near_sdk::testing_env;
// use crate::*;
//
//
// pub fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
//     let mut builder = VMContextBuilder::new();
//     builder
//         .current_account_id(accounts(0))
//         .signer_account_id(predecessor_account_id.clone())
//         .predecessor_account_id(predecessor_account_id);
//     builder
// }
//
// pub fn init(account_id: AccountId) -> Contract {
//     let context = get_context(account_id.clone());
//     testing_env!(context.build());
//     let contract = Contract::new_default_meta(account_id);
//     contract
// }