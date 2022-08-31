extern crate core;

use std::collections::HashMap;
use std::fmt::Debug;
use near_sdk::{PromiseOrValue, Promise, near_bindgen, PanicOnDefault, BorshStorageKey, AccountId, borsh::{self, BorshDeserialize, BorshSerialize}, serde::{Deserialize, Serialize}, env, CryptoHash};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{U128, Base64VecU8};

// use crate::zoo::*;
use crate::nft::*;

mod nft;
// mod zoo;
// mod test;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // token: NonFungibleToken,
    // metadata: LazyOption<NFTContractMetadata>,
    // pub zoos: UnorderedMap<AccountId, Zoo>,

    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,

}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
}

#[near_bindgen]
impl Contract {
    /*
      initialization function (can only be called once).
      this initializes the contract with default metadata so the
      user doesn't have to manually type metadata.
  */
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        Self::new(owner_id, NFTContractMetadata {
            spec: "u_zoo_1.0.0".to_string(),
            name: "u_zoo".to_string(),
            symbol: "UZOO".to_string(),
            icon: None,
            base_uri: None,
            reference: None,
            reference_hash: None,
        })
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id.
    */
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        let this = Self {
            owner_id,
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(StorageKey::TokenMetadataById.try_to_vec().unwrap()),
            metadata: LazyOption::new(StorageKey::NFTContractMetadata.try_to_vec().unwrap(), Some(&metadata)),
        };

        this
    }

    // #[init]
    // pub fn new(owner_id: AccountId) -> Self {
    //     Self {
    //         token: NonFungibleToken::new(
    //             StorageKey::NonFungibleToken,
    //             owner_id,
    //             Some(StorageKey::TokenMetadata),
    //             Some(StorageKey::Enumeration),
    //             Some(StorageKey::Approval),
    //         ),
    //         metadata: LazyOption::new(
    //             StorageKey::Metadata,
    //             Some(&NFTContractMetadata {
    //                 spec: NFT_METADATA_SPEC.to_string(),
    //                 name: "u_zoo".to_string(),
    //                 symbol: "Example".to_string(),
    //                 icon: Some("ANY_SVG".to_string()),
    //                 base_uri: None,
    //                 reference: None,
    //                 reference_hash: None,
    //             }),
    //         ),
    //         zoos: UnorderedMap::new(b"d".to_vec()),
    //     }
    // }

    // #[payable]
    // pub fn buy_nft(&mut self, zoo_id: AccountId) {
    //     let zoo_id = zoo_id.clone();
    //     let deposit: u128 = near_sdk::env::attached_deposit();
    //     let mut zoo: Zoo = self.zoos.get(&zoo_id).expect("Zoo doesn't exist");
    //
    //
    //     assert_eq!(deposit, zoo.nft_price, "Incorrect attached deposit");
    //
    //
    //     zoo.nft_sold += 1;
    //     zoo.total_collected += deposit;
    //     self.zoos.remove(&zoo_id);
    //     self.zoos.insert(&zoo_id, &zoo);
    //
    //
    //     let receiver_id: AccountId = env::predecessor_account_id();
    //
    //     let seed = near_sdk::env::random_seed();
    //     self.nft_mint(
    //         receiver_id.to_string() + &'-'.to_string() + &seed[0].to_string() + &seed[1].to_string() + &seed[2].to_string(),
    //         receiver_id,
    //         TokenMetadata {
    //             title: Option::from(zoo.title),
    //             description: Option::from(zoo.description),
    //             media: Option::from(zoo.nft_media),
    //             media_hash: None,
    //             copies: None,
    //             issued_at: None,
    //             expires_at: None,
    //             starts_at: None,
    //             updated_at: None,
    //             extra: None,
    //             reference: None,
    //             reference_hash: None,
    //         });
    //
    //     Promise::new(zoo.owner_id).transfer(deposit);
    // }
}


#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts};
    use near_sdk::{testing_env};
    use test::get_context;


    const MINT_STORAGE_COST: u128 = 5870000000000000000000;

    fn sample_token_metadata() -> TokenMetadata {
        TokenMetadata {
            title: Some("Olympus Mons".into()),
            description: Some("The tallest mountain in the charted solar system".into()),
            media: None,
            media_hash: None,
            copies: Some(1u64),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        }
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new(accounts(1).into());
        testing_env!(context.is_view(true).build());
        let metadata = contract.metadata.get().unwrap();
        assert_eq!(contract.token.owner_id, accounts(1).into());
        assert_eq!(metadata.name, String::from("u_zoo"));
        assert_eq!(metadata.symbol, String::from("Example"));
    }


    #[test]
    fn test_mint() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());

        let token_id = "0".to_string();
        let token = contract.nft_mint(token_id.clone(), accounts(1), sample_token_metadata());
        assert_eq!(token.token_id, token_id);
        assert_eq!(token.owner_id.to_string(), accounts(1).to_string());
        assert_eq!(token.metadata.unwrap(), sample_token_metadata());
        assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
    }


    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());
        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_transfer(accounts(1), token_id.clone(), None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        if let Some(token) = contract.nft_token(token_id.clone()) {
            assert_eq!(token.token_id, token_id);
            assert_eq!(token.owner_id.to_string(), accounts(1).to_string());
            assert_eq!(token.metadata.unwrap(), sample_token_metadata());
            assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
        } else {
            panic!("token not correctly created, or not found by nft_token");
        }
    }
}