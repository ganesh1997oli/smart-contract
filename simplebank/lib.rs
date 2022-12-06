#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod simplebank {
    use ink_storage::{Mapping, traits::{SpreadAllocate}};


    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Simplebank {
        client_count: u8,
        balance: Mapping<AccountId, u128>,
        owner: AccountId,
    }

    #[ink(event)]
    pub struct DepositMade {
        account_address: AccountId,
        balance: u128,
    }

    impl Simplebank {
        #[ink(constructor)]
        pub fn new() -> Self {
            // assert!(Self::env().balance() == 30);
            let caller = Self::env().caller();

            ink_lang::utils::initialize_contract(|_instance: &mut Simplebank|{
                _instance.client_count = Default::default();
                _instance.balance = Default::default();
                _instance.owner = caller;
            })
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self) {
            if self.client_count < 3 {
                self.client_count += 1;
                self.balance.insert(self.env().caller(), &10);
            } else {
                self.balance.insert(self.env().caller(), &self.env().balance());
            }
        }

        #[ink(message, payable)]
        pub fn withdraw(&mut self, withdraw_amount: u128) {
            let caller = self.env().caller();
            let mut balance = self.balance.get(caller).unwrap_or_default();
            if withdraw_amount <=  balance {
                balance -= withdraw_amount;
                self.env().transfer(caller, withdraw_amount).unwrap_or_default();
            }
        }

        #[ink(message)]
        pub fn balance(&self) -> Option<u128>{
            let caller = self.env().caller();
            self.balance.get(caller)
        }

        #[ink(message)]
        pub fn deposits_balance(&self) -> u128 {
            self.env().balance()
        }


    }

}
