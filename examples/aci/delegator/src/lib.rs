use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::metadata;
use near_sdk::near_bindgen;

schemafy_near::schemafy!("../target/near/adder/metadata.json");

metadata! {
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Delegator {}

#[near_bindgen]
impl Delegator {
    pub fn delegate(&self, a: u32, b: u32) -> u32 {
        a + b
    }
}
}
