use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct PratsContract {
    // name: String, 
}

#[near_bindgen]
impl PratsContract {
    //fn to greet user
    pub fn greet_user(&mut self,user_input: String) -> String {
        format!("Hello {}!", user_input)
    }
}
