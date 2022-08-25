// Contains token minting logic.

use crate::*;

#[near_bindgen]
impl Contract {
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