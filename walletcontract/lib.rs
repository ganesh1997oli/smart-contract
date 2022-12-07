#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod walletcontract {


    #[ink(storage)]
    pub struct Walletcontract {
        owner: AccountId,
    }

    impl Walletcontract {
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            Self { owner }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self) {}

        #[ink(message, payable)]
        pub fn send(&mut self, to: AccountId, amount: u128){
            let caller = self.env().caller();
            if caller == self.owner {
                self.env().transfer(to, amount).unwrap_or_default()
            } else {
                panic!("sender is not allowed")
            }
        }

        #[ink(message)]
        pub fn balance_of(&self) -> u128 {
            self.env().balance()
        }
    }
}
