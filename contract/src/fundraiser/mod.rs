use crate::*;


pub type FundraiserId = u32;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum FundraiserStatus {
    ACTIVE,
    DRAFT,
    COMPLETED,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Fundraiser {
    owner_id: AccountId,
    title: String,
    description: String,
    status: FundraiserStatus,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonFundraiser {
    pub fundraiser: Fundraiser,
    pub fundraiser_id: FundraiserId,
    pub token_id: TokenId,
    pub token: Token,
    pub token_metadata: TokenMetadata,
}


#[near_bindgen]
impl Contract {
    pub fn get_all_fundraisers(&self, page: Option<u32>) -> Vec<JsonFundraiser> {
        let (from_index, take) = pagination(page);
        self.fundraisers_by_id.keys()
            .skip(from_index as usize)
            .take(take as usize)
            .map(|fundraiser_id: FundraiserId| self.get_fundraiser_by_id(fundraiser_id.clone()).unwrap())
            .collect()
    }

    pub fn get_fundraiser_by_id(&self, id: FundraiserId) -> Option<JsonFundraiser> {
        if let Some(fundraiser) = self.fundraisers_by_id.get(&id) {
            let token_id: TokenId = id.to_string();
            if let Some(jsonToken) = self.nft_token(token_id.clone()) {
                Some(JsonFundraiser {
                    fundraiser,
                    fundraiser_id: id,
                    token: self.tokens_by_id.get(&token_id).unwrap(),
                    token_id,
                    token_metadata: jsonToken.metadata,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    #[payable]
    pub fn add_new_fundraiser(
        &mut self,
        title: String,
        description: String,
        status: FundraiserStatus,
        token_metadata: TokenMetadata,
    ) {
        assert!(title != "", "Abort. Title is empty");
        assert!(title.len() <= 1000, "Abort. Title is longer then 1000 characters");
        assert!(description.len() <= 2000, "Abort. Description is longer then 2000 characters");

        let owner_id = env::predecessor_account_id();
        let fundraiser = Fundraiser {
            owner_id: owner_id.clone(),
            title,
            description,
            status,
        };

        let mut f_owner_set = self.fundraiser_per_owner.get(&owner_id).unwrap_or_else(|| {
            //if the account doesn't have any fundraisers, we create a new unordered set

            // Constructing a unique prefix for a nested UnorderedSet from a concatenation
            // of a prefix and a hash of the account id.
            let prefix: Vec<u8> = [
                b"s".as_slice(),
                &near_sdk::env::sha256_array(owner_id.as_bytes()),
            ]
                .concat();
            UnorderedSet::new(prefix)
        });

        self.fundraiser_counter += 1;

        let fundraiser_id: FundraiserId = self.fundraiser_counter.clone();
        let token_id: TokenId = self.fundraiser_counter.clone().to_string();

        f_owner_set.insert(&fundraiser_id);
        self.fundraiser_per_owner.insert(&owner_id, &f_owner_set);

        self.nft_mint(token_id, owner_id, token_metadata);

        self.fundraisers_by_id.insert(&fundraiser_id.clone(), &fundraiser);
    }
    //
    // pub fn update_zoo(
    //     &mut self,
    //     zoo_id: AccountId,
    //     title: String,
    //     description: String,
    //     address: String,
    //     banner_image: String,
    //     nft_price: String,
    // ) -> Zoo {
    //     let nft_price_u128: u128 = nft_price.parse::<u128>().unwrap();
    //     assert!(env::predecessor_account_id() == zoo_id, "You cannot update this zoo!");
    //     assert!(title != "", "Abort. Title is empty");
    //     assert!(title.len() <= 1000, "Abort. Title is longer then 1000 characters");
    //     assert!(description.len() <= 2000, "Abort. Description is longer then 2000 characters");
    //     assert!(address != "", "Abort. Address is empty");
    //     assert!(address.len() <= 1000, "Abort. Address is longer then 1000 characters");
    //     assert!(banner_image != "", "Abort. Banner image is empty");
    //
    //     let mut zoo: Zoo = self.zoos.get(&zoo_id).expect("Zoo doesn't exist");
    //
    //     zoo.title = title;
    //     zoo.description = description;
    //     zoo.address = address;
    //     zoo.banner_image = banner_image;
    //     zoo.nft_price = nft_price_u128;
    //
    //     self.zoos.remove(&zoo_id);
    //     self.zoos.insert(&zoo_id, &zoo);
    //
    //     zoo
    // }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts};
    use std::iter::repeat;
    use test::init;

    #[test]
    fn get_fundraiser_by_id_not_found_test() {
        let contract = init(accounts(1));
        assert_eq!(contract.get_fundraiser_by_id(1), None);
    }

    #[test]
    fn get_fundraiser_by_id_success_test() {
        let mut contract = init(accounts(1));
        let owner_id = accounts(2);
        let fundraiser = Fundraiser {
            owner_id: owner_id.clone(),
            title: "".to_string(),
            description: "".to_string(),
            status: FundraiserStatus::ACTIVE,
        };
        let mut fundraiser_set = contract.fundraiser_per_owner.get(&owner_id).unwrap_or_else(|| {
            UnorderedSet::new(vec![])
        });
        let fundraiser_id: FundraiserId = contract.fundraiser_counter + 1;

        fundraiser_set.insert(&fundraiser_id);
        contract.fundraisers_by_id.insert(&fundraiser_id, &fundraiser);
        let token: Token = Token {
            owner_id
        };
        let token_metadata = TokenMetadata {
            title: None,
            description: None,
            media: None,
            media_hash: None,
            copies: None,
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        };
        let token_id: TokenId = fundraiser_id.to_string();
        contract.tokens_by_id.insert(&token_id, &token);

        contract.token_metadata_by_id.insert(&token_id, &token_metadata);

        let json_fundraiser: JsonFundraiser = JsonFundraiser {
            fundraiser,
            fundraiser_id,
            token_id,
            token_metadata,
        };
        assert_eq!(contract.get_fundraiser_by_id(1), Some(json_fundraiser));
    }


    #[test]
    fn get_all_fundraisers_empty_test() {
        let contract = init(accounts(1));

        let empty: Vec<JsonFundraiser> = Vec::new();
        assert_eq!(contract.get_all_fundraisers(), empty);
    }


    //
    // #[test]
    // fn get_all_zoos_empty_test() {
    //     let contract = init(accounts(1));
    //
    //     let empty: HashMap<AccountId, Zoo> = HashMap::new();
    //     assert_eq!(contract.get_all_zoos(), empty);
    // }
    //
    // #[test]
    // fn get_all_zoos_test() {
    //     let mut contract = init(accounts(1));
    //
    //     contract.zoos.insert(
    //         &accounts(2),
    //         &Zoo {
    //             owner_id: accounts(2),
    //             title: String::from("test"),
    //             description: String::from("test"),
    //             address: String::from("test"),
    //             banner_image: String::from("test"),
    //             nft_media: String::from("test"),
    //             nft_price: 1_000_000_000_000_000_000_000_000,
    //             total_collected: 0,
    //             nft_sold: 0,
    //         },
    //     );
    //     let result: HashMap<AccountId, Zoo> = contract.zoos.iter().collect();
    //     let result_from_method: HashMap<AccountId, Zoo> = contract.get_all_zoos();
    //     assert_eq!(result_from_method, result);
    //     assert_eq!(result_from_method.get(&accounts(2)).unwrap().owner_id, accounts(2));
    //     assert_eq!(result_from_method.get(&accounts(2)).unwrap().title, String::from("test"));
    // }
    //
    // #[test]
    // #[should_panic(expected = "Abort. Title is empty")]
    // fn add_new_zoo_title_empty_validation_error_test() {
    //     let mut contract = init(accounts(1));
    //     contract.add_new_zoo(
    //         String::from(""),
    //         String::from(""),
    //         String::from(""),
    //         String::from(""),
    //         String::from(""),
    //         String::from("10000000000000000000"),
    //     );
    // }
    //
    // #[test]
    // #[should_panic(expected = "Abort. Title is longer then 1000 characters")]
    // fn add_new_zoo_title_length_validation_error_test() {
    //     let mut contract = init(accounts(1));
    //     contract.add_new_zoo(
    //         repeat("X").take(1001).collect::<String>(),
    //         String::from(""),
    //         String::from(""),
    //         String::from(""),
    //         String::from(""),
    //         String::from("10000000000000000000"),
    //     );
    // }
    //
    // #[test]
    // #[should_panic(expected = "Abort. Description is longer then 2000 characters")]
    // fn add_new_zoo_description_length_validation_error_test() {
    //     let mut contract = init(accounts(1));
    //     contract.add_new_zoo(
    //         repeat("X").take(999).collect::<String>(),
    //         repeat("X").take(2001).collect::<String>(),
    //         String::from(""),
    //         String::from(""),
    //         String::from(""),
    //         String::from("10000000000000000000"),
    //     );
    // }
    //
    // #[test]
    // #[should_panic(expected = "Abort. Address is empty")]
    // fn add_new_zoo_address_empty_validation_error_test() {
    //     let mut contract = init(accounts(1));
    //     contract.add_new_zoo(
    //         repeat("X").take(999).collect::<String>(),
    //         repeat("X").take(1999).collect::<String>(),
    //         String::from(""),
    //         String::from(""),
    //         String::from(""),
    //         String::from("10000000000000000000"),
    //     );
    // }
    //
    // #[test]
    // #[should_panic(expected = "Abort. Address is longer then 1000 characters")]
    // fn add_new_zoo_address_length_validation_error_test() {
    //     let mut contract = init(accounts(1));
    //     contract.add_new_zoo(
    //         repeat("X").take(999).collect::<String>(),
    //         repeat("X").take(1999).collect::<String>(),
    //         repeat("X").take(1001).collect::<String>(),
    //         String::from(""),
    //         String::from(""),
    //         String::from("10000000000000000000"),
    //     );
    // }
    //
    // #[test]
    // #[should_panic(expected = "Abort. Banner image is empty")]
    // fn add_new_zoo_banner_image_empty_validation_error_test() {
    //     let mut contract = init(accounts(1));
    //     contract.add_new_zoo(
    //         repeat("X").take(999).collect::<String>(),
    //         repeat("X").take(1999).collect::<String>(),
    //         repeat("X").take(999).collect::<String>(),
    //         String::from(""),
    //         String::from(""),
    //         String::from("10000000000000000000"),
    //     );
    // }
    //
    // #[test]
    // #[should_panic(expected = "Abort. NFT media is empty")]
    // fn add_new_zoo_nft_media_empty_validation_error_test() {
    //     let mut contract = init(accounts(1));
    //     contract.add_new_zoo(
    //         repeat("X").take(999).collect::<String>(),
    //         repeat("X").take(1999).collect::<String>(),
    //         repeat("X").take(999).collect::<String>(),
    //         String::from("banner_image"),
    //         String::from(""),
    //         String::from("10000000000000000000"),
    //     );
    // }
    //
    // #[test]
    // fn add_new_zoo_success_test() {
    //     let mut contract = init(accounts(1));
    //     contract.add_new_zoo(
    //         String::from("SomeTitle"),
    //         repeat("X").take(1999).collect::<String>(),
    //         repeat("X").take(999).collect::<String>(),
    //         String::from("banner_image"),
    //         String::from("nft_media"),
    //         String::from("10000000000000000000"),
    //     );
    //     assert_eq!(contract.zoos.len(), 1);
    //     assert_eq!(contract.zoos.keys().find(|x| x == &accounts(1)), Some(accounts(1)));
    //     assert_eq!(contract.zoos.get(&accounts(1)).unwrap().title, String::from("SomeTitle"));
    // }


    //
    // #[test]
    // fn update_zoo_by_id_success_test() {
    //     let mut contract = init(accounts(1));
    //     let zoo: Zoo = Zoo {
    //         owner_id: accounts(1),
    //         title: String::from("test"),
    //         description: String::from("test"),
    //         address: String::from("test"),
    //         banner_image: String::from("test"),
    //         nft_media: String::from("test"),
    //         nft_price: 1_000_000_000_000_000,
    //         total_collected: 0,
    //         nft_sold: 0,
    //     };
    //     contract.zoos.insert(
    //         &accounts(1),
    //         &zoo,
    //     );
    //
    //     assert_eq!(contract.zoos.get(&accounts(1)), Some(zoo.clone()));
    //     assert_ne!(contract.update_zoo(accounts(1),
    //                                    "title".to_string(),
    //                                    "description".to_string(),
    //                                    "address".to_string(),
    //                                    "image".to_string(),
    //                                    "111111111".to_string()), zoo.clone());
    //     assert_ne!(contract.zoos.get(&accounts(1)), Some(zoo.clone()));
    // }
}