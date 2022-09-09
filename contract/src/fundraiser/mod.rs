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
    #[payable]
    pub fn donate_to_fundraiser(&mut self, fundraiser_id: FundraiserId)
    {
        let fundraiser: Fundraiser = self.fundraisers_by_id.get(&fundraiser_id).unwrap();
        let donation = env::attached_deposit();
        let donor_id = env::predecessor_account_id();

        let mut fundraiser_donations_list = self.fundraisers_donations.get(&fundraiser_id).unwrap_or_else(|| {
            // if there is no donations yet -> initialize lookup for the donor
            let prefix: Vec<u8> = [
                b"f_donations".as_slice(),
                &near_sdk::env::sha256_array(donor_id.as_bytes()),
            ]
                .concat();
            UnorderedMap::new(prefix)
        });
        let mut donations_of_donor = fundraiser_donations_list.get(&donor_id).unwrap_or_else(|| {
            // if there is no donations for donor -> init it
            let prefix: Vec<u8> = [
                b"donor".as_slice(),
                &near_sdk::env::sha256_array(donor_id.as_bytes()),
            ].concat();
            UnorderedSet::new(prefix)
        });
        donations_of_donor.insert(&u128::from(donation)); // todo make to store a lot of equal values
        fundraiser_donations_list.insert(&donor_id, &donations_of_donor);
        self.fundraisers_donations.insert(&fundraiser_id, &fundraiser_donations_list);
        log!("{:?}",donation);
        log!("{:?}", self.fundraisers_donations.get(&fundraiser_id).unwrap().get(&donor_id).unwrap().to_vec())
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
    use std::iter::repeat;
    use test::test_helpers::init;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use crate::test_helpers::get_context;

    const MINT_STORAGE_COST: u128 = 5870000000000000000000;

    fn attach_dep_for_adding_fundraiser() -> Contract {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());

        contract
    }

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
            token,
            token_metadata,
        };
        assert_eq!(contract.get_fundraiser_by_id(1), Some(json_fundraiser));
    }


    #[test]
    fn get_all_fundraisers_empty_test() {
        let contract = init(accounts(1));
        let empty: Vec<JsonFundraiser> = Vec::new();
        assert_eq!(contract.get_all_fundraisers(Some(1)), empty);
    }

    #[test]
    fn get_all_fundraisers_test() {
        let mut contract = attach_dep_for_adding_fundraiser();

        contract.add_new_fundraiser("test".to_string(), "".to_string(), FundraiserStatus::ACTIVE, TokenMetadata {
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
        });
        assert_eq!(contract.get_all_fundraisers(Some(1)).len(), 1);
        assert_eq!(contract.get_all_fundraisers(None).len(), 1);
    }

    // #[should_panic(expected = "Abort. Address is longer then 1000 characters")]

    #[test]
    #[should_panic] // we are not attaching any deposit so it would panic
    fn add_new_fundraiser_panic_test() {
        let mut contract = init(accounts(1));
        contract.add_new_fundraiser("test".to_string(), "".to_string(), FundraiserStatus::ACTIVE, TokenMetadata {
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
        });
    }

    #[test]
    #[should_panic]
    fn add_new_fundraiser_validation_title_test() {
        let mut contract = attach_dep_for_adding_fundraiser();
        contract.add_new_fundraiser("".to_string(), "".to_string(), FundraiserStatus::ACTIVE, TokenMetadata {
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
        });
    }


    #[test]
    #[should_panic]
    fn add_new_fundraiser_validation_description_test() {
        let mut contract = attach_dep_for_adding_fundraiser();

        contract.add_new_fundraiser("test".to_string(), repeat("X").take(2001).collect::<String>(), FundraiserStatus::ACTIVE, TokenMetadata {
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
        });
    }
}