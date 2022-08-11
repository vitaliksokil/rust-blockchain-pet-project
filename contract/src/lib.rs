use std::collections::HashMap;
use std::fmt::Debug;
use near_sdk::{PromiseOrValue, Promise, near_bindgen, PanicOnDefault, BorshStorageKey, AccountId, borsh::{self, BorshDeserialize, BorshSerialize}, serde::{Deserialize, Serialize}, env};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{metadata::NFTContractMetadata, Token, TokenId};
use near_contract_standards::non_fungible_token::metadata::{NFT_METADATA_SPEC, NonFungibleTokenMetadataProvider, TokenMetadata};
use near_sdk::collections::{LazyOption, UnorderedMap};

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Zoo {
    owner_id: AccountId,
    title: String,
    description: String,
    address: String,
    banner_image: String,
    nft_media: String,
    // in yoctoNear 5_000_000_000_000_000_000_000_000 = 5 Ⓝ
    nft_price: u128,
    total_collected: u128,
    nft_sold: u16,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    pub zoos: UnorderedMap<AccountId, Zoo>,
}

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            token: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(
                StorageKey::Metadata,
                Some(&NFTContractMetadata {
                    spec: NFT_METADATA_SPEC.to_string(),
                    name: "u_zoo".to_string(),
                    symbol: "Example".to_string(),
                    icon: Some("ANY_SVG".to_string()),
                    base_uri: None,
                    reference: None,
                    reference_hash: None,
                }),
            ),
            zoos: UnorderedMap::new(b"d".to_vec()),
        }
    }


    pub fn get_all_zoos(&self) -> HashMap<AccountId, Zoo> {
        self.zoos.iter().collect()
    }

    pub fn get_zoo_by_id(&self, id: AccountId) -> Zoo {
        self.zoos.get(&id).expect("Zoo doesn't exist")
    }

    pub fn add_new_zoo(
        &mut self,
        title: String,
        description: String,
        address: String,
        banner_image: String,
        nft_media: String,
        nft_price: String,
    ) {
        let nft_price_u128: u128 = nft_price.parse::<u128>().unwrap();
        assert!(self.zoos.get(&env::predecessor_account_id()).is_none(), "Zoo already created for this user.");

        assert!(title != "", "Abort. Title is empty");
        assert!(title.len() <= 1000, "Abort. Title is longer then 1000 characters");
        assert!(description.len() <= 2000, "Abort. Description is longer then 2000 characters");
        assert!(address != "", "Abort. Address is empty");
        assert!(address.len() <= 1000, "Abort. Address is longer then 1000 characters");
        assert!(banner_image != "", "Abort. Banner image is empty");
        assert!(nft_media != "", "Abort. NFT media is empty");


        self.zoos.insert(
            &env::predecessor_account_id(),
            &Zoo {
                owner_id: env::predecessor_account_id(),
                title,
                description,
                address,
                banner_image,
                nft_media,
                nft_price: nft_price_u128,
                total_collected: 0,
                nft_sold: 0,
            },
        );
    }

    pub fn update_zoo(
        &mut self,
        zoo_id: AccountId,
        title: String,
        description: String,
        address: String,
        banner_image: String,
        nft_price: String,
    ) -> Zoo {
        let nft_price_u128: u128 = nft_price.parse::<u128>().unwrap();
        assert!(env::predecessor_account_id() == zoo_id, "You cannot update this zoo!");
        assert!(title != "", "Abort. Title is empty");
        assert!(title.len() <= 1000, "Abort. Title is longer then 1000 characters");
        assert!(description.len() <= 2000, "Abort. Description is longer then 2000 characters");
        assert!(address != "", "Abort. Address is empty");
        assert!(address.len() <= 1000, "Abort. Address is longer then 1000 characters");
        assert!(banner_image != "", "Abort. Banner image is empty");

        let mut zoo: Zoo = self.zoos.get(&zoo_id).expect("Zoo doesn't exist");

        zoo.title = title;
        zoo.description = description;
        zoo.address = address;
        zoo.banner_image = banner_image;
        zoo.nft_price = nft_price_u128;

        self.zoos.remove(&zoo_id);
        self.zoos.insert(&zoo_id, &zoo);

        zoo
    }

    #[payable]
    pub fn buy_nft(&mut self, zoo_id: AccountId) {
        let zoo_id = zoo_id.clone();
        let deposit: u128 = near_sdk::env::attached_deposit();
        let mut zoo: Zoo = self.zoos.get(&zoo_id).expect("Zoo doesn't exist");


        assert_eq!(deposit, zoo.nft_price, "Incorrect attached deposit");


        zoo.nft_sold += 1;
        zoo.total_collected += deposit;
        self.zoos.remove(&zoo_id);
        self.zoos.insert(&zoo_id, &zoo);


        let receiver_id: AccountId = env::predecessor_account_id();

        let seed = near_sdk::env::random_seed();
        self.nft_mint(
            receiver_id.to_string() + &'-'.to_string() + &seed[0].to_string() + &seed[1].to_string() + &seed[2].to_string(),
            receiver_id,
            TokenMetadata {
                title: Option::from(zoo.title),
                description: Option::from(zoo.description),
                media: Option::from(zoo.nft_media),
                media_hash: None,
                copies: None,
                issued_at: None,
                expires_at: None,
                starts_at: None,
                updated_at: None,
                extra: None,
                reference: None,
                reference_hash: None,
            });

        Promise::new(zoo.owner_id).transfer(deposit);
    }

    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        token_metadata: TokenMetadata,
    ) -> Token {
        self.token.internal_mint(token_id, receiver_id, Some(token_metadata))
    }
}

near_contract_standards::impl_non_fungible_token_core!(Contract, token);
near_contract_standards::impl_non_fungible_token_approval!(Contract, token);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, token);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}


#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, VMContext};
    use std::iter::repeat;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn init(account_id: AccountId) -> Contract {
        let context = get_context(account_id.clone());
        testing_env!(context.build());
        let contract = Contract::new(account_id);
        contract
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
    fn get_all_zoos_empty_test() {
        let contract = init(accounts(1));

        let empty: HashMap<AccountId, Zoo> = HashMap::new();
        assert_eq!(contract.get_all_zoos(), empty);
    }

    #[test]
    fn get_all_zoos_test() {
        let mut contract = init(accounts(1));

        contract.zoos.insert(
            &accounts(2),
            &Zoo {
                owner_id: accounts(2),
                title: String::from("test"),
                description: String::from("test"),
                address: String::from("test"),
                banner_image: String::from("test"),
                nft_media: String::from("test"),
                nft_price: 1_000_000_000_000_000_000_000_000,
                total_collected: 0,
                nft_sold: 0,
            },
        );
        let result: HashMap<AccountId, Zoo> = contract.zoos.iter().collect();
        let result_from_method: HashMap<AccountId, Zoo> = contract.get_all_zoos();
        assert_eq!(result_from_method, result);
        assert_eq!(result_from_method.get(&accounts(2)).unwrap().owner_id, accounts(2));
        assert_eq!(result_from_method.get(&accounts(2)).unwrap().title, String::from("test"));
    }

    #[test]
    #[should_panic(expected = "Abort. Title is empty")]
    fn add_new_zoo_title_empty_validation_error_test() {
        let mut contract = init(accounts(1));
        contract.add_new_zoo(
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from("10000000000000000000"),
        );
    }

    #[test]
    #[should_panic(expected = "Abort. Title is longer then 1000 characters")]
    fn add_new_zoo_title_length_validation_error_test() {
        let mut contract = init(accounts(1));
        contract.add_new_zoo(
            repeat("X").take(1001).collect::<String>(),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from("10000000000000000000"),
        );
    }

    #[test]
    #[should_panic(expected = "Abort. Description is longer then 2000 characters")]
    fn add_new_zoo_description_length_validation_error_test() {
        let mut contract = init(accounts(1));
        contract.add_new_zoo(
            repeat("X").take(999).collect::<String>(),
            repeat("X").take(2001).collect::<String>(),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from("10000000000000000000"),
        );
    }

    #[test]
    #[should_panic(expected = "Abort. Address is empty")]
    fn add_new_zoo_address_empty_validation_error_test() {
        let mut contract = init(accounts(1));
        contract.add_new_zoo(
            repeat("X").take(999).collect::<String>(),
            repeat("X").take(1999).collect::<String>(),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from("10000000000000000000"),
        );
    }

    #[test]
    #[should_panic(expected = "Abort. Address is longer then 1000 characters")]
    fn add_new_zoo_address_length_validation_error_test() {
        let mut contract = init(accounts(1));
        contract.add_new_zoo(
            repeat("X").take(999).collect::<String>(),
            repeat("X").take(1999).collect::<String>(),
            repeat("X").take(1001).collect::<String>(),
            String::from(""),
            String::from(""),
            String::from("10000000000000000000"),
        );
    }

    #[test]
    #[should_panic(expected = "Abort. Banner image is empty")]
    fn add_new_zoo_banner_image_empty_validation_error_test() {
        let mut contract = init(accounts(1));
        contract.add_new_zoo(
            repeat("X").take(999).collect::<String>(),
            repeat("X").take(1999).collect::<String>(),
            repeat("X").take(999).collect::<String>(),
            String::from(""),
            String::from(""),
            String::from("10000000000000000000"),
        );
    }

    #[test]
    #[should_panic(expected = "Abort. NFT media is empty")]
    fn add_new_zoo_nft_media_empty_validation_error_test() {
        let mut contract = init(accounts(1));
        contract.add_new_zoo(
            repeat("X").take(999).collect::<String>(),
            repeat("X").take(1999).collect::<String>(),
            repeat("X").take(999).collect::<String>(),
            String::from("banner_image"),
            String::from(""),
            String::from("10000000000000000000"),
        );
    }

    #[test]
    fn add_new_zoo_success_test() {
        let mut contract = init(accounts(1));
        contract.add_new_zoo(
            String::from("SomeTitle"),
            repeat("X").take(1999).collect::<String>(),
            repeat("X").take(999).collect::<String>(),
            String::from("banner_image"),
            String::from("nft_media"),
            String::from("10000000000000000000"),
        );
        assert_eq!(contract.zoos.len(), 1);
        assert_eq!(contract.zoos.keys().find(|x| x == &accounts(1)), Some(accounts(1)));
        assert_eq!(contract.zoos.get(&accounts(1)).unwrap().title, String::from("SomeTitle"));
    }
}