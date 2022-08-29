// Core logic that allows you to transfer NFTs between users.

use crate::*;
use near_sdk::{ext_contract, Gas, log, PromiseResult};

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_ON_TRANSFER: Gas = Gas(25_000_000_000_000);

pub trait NonFungibleTokenCore {
    //transfers an NFT to a receiver ID
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        memo: Option<String>,
    );

    //get information about the NFT token passed in
    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken>;
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {

    //implementation of the nft_transfer method. This transfers the NFT from the current owner to the receiver.
    #[payable]
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        memo: Option<String>,
    ) {
        /*
            FILL THIS IN
        */
    }

    //get the information for a specific token ID
    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken> {
       if let Some(token) = self.tokens_by_id.get(&token_id) {
           let metadata = self.token_metadata_by_id.get(&token_id).unwrap();
           Some(JsonToken{
               token_id,
               owner_id: token.owner_id,
               metadata
           })
       }else{
           None
       }
    }
}
