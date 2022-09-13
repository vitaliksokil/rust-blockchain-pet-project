use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn seed(&mut self) {
        if config::ENV == config::ConfigEnv::Dev && env::predecessor_account_id() == self.owner_id {
            fundraiser_seed(self);
        }else{
            panic!("You cannot seed!");
        }
    }
}
fn fundraiser_seed(contract: &mut Contract) {
    let nft_media: String = String::from("https://bafybeiekkhg57mp7u22zwiwjqxiwe3r4birpm2xwlrgv7u6boxndwgpi3y.ipfs.nftstorage.link/");
    contract.add_new_fundraiser("Test".to_string(),
                                "test".to_string(),
                                FundraiserStatus::ACTIVE,
                                TokenMetadata {
                                    title: Some(String::from("NFT title")),
                                    description: Some(String::from("NFT description")),
                                    media: Some(nft_media),
                                    media_hash: None,
                                    copies: None,
                                    issued_at: None,
                                    expires_at: None,
                                    starts_at: None,
                                    updated_at: None,
                                    extra: None,
                                    reference: None,
                                    reference_hash: None
                                });
}