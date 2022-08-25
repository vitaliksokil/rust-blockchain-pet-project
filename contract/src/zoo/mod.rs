use crate::*;

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
impl Contract {
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
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts};
    use std::iter::repeat;
    use test::init;

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

    #[test]
    #[should_panic(expected = "Zoo doesn't exist")]
    fn get_zoo_by_id_not_found_test() {
        let contract = init(accounts(1));
        contract.get_zoo_by_id(accounts(2));
    }

    #[test]
    fn get_zoo_by_id_success_test() {
        let mut contract = init(accounts(1));
        let zoo: Zoo = Zoo {
            owner_id: accounts(1),
            title: String::from("test"),
            description: String::from("test"),
            address: String::from("test"),
            banner_image: String::from("test"),
            nft_media: String::from("test"),
            nft_price: 1_000_000_000_000_000,
            total_collected: 0,
            nft_sold: 0,
        };
        contract.zoos.insert(
            &accounts(1),
            &zoo,
        );
        assert_eq!(contract.get_zoo_by_id(accounts(1)), zoo);
    }

    #[test]
    fn update_zoo_by_id_success_test() {
        let mut contract = init(accounts(1));
        let zoo: Zoo = Zoo {
            owner_id: accounts(1),
            title: String::from("test"),
            description: String::from("test"),
            address: String::from("test"),
            banner_image: String::from("test"),
            nft_media: String::from("test"),
            nft_price: 1_000_000_000_000_000,
            total_collected: 0,
            nft_sold: 0,
        };
        contract.zoos.insert(
            &accounts(1),
            &zoo,
        );

        assert_eq!(contract.zoos.get(&accounts(1)), Some(zoo.clone()));
        assert_ne!(contract.update_zoo(accounts(1),
                                       "title".to_string(),
                                       "description".to_string(),
                                       "address".to_string(),
                                       "image".to_string(),
                                       "111111111".to_string()), zoo.clone());
        assert_ne!(contract.zoos.get(&accounts(1)), Some(zoo.clone()));
    }

}