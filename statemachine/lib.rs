#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod statemachine {
    use ink_storage::traits::{SpreadLayout, StorageLayout};


    #[derive(SpreadLayout, StorageLayout, Debug)]
    pub enum State {
        PENDING,
        ACTIVE,
        CLOSED, 
    }

    #[ink(storage)]
    pub struct Statemachine {
       state: State,
       amount: u128,
       interest: u128,
       end: i32,
       borrower: AccountId,
       lender: AccountId,
    }

    impl Statemachine {
        #[ink(constructor)]
        pub fn new(amount: u128, interest: u128, end: i32, borrower: AccountId, lender: AccountId) -> Self {
            let state = State::PENDING;
            Self { 
                state,
                amount,
                interest,
                end,
                borrower,
                lender
            }
        }

        #[ink(message, payable)]
        pub fn fund(&mut self){
            let caller = self.env().caller();
            let balance = self.env().balance();

            assert!(caller == self.lender, "only lender can lend");
            assert!(balance == self.amount, "can only lend the exact amount");
            self.env().transfer(self.borrower, self.amount).unwrap_or_default();
        }

        #[ink(message, payable)]
        pub fn reimburse(&mut self) {
            let caller = self.env().caller();
            let balance = self.env().balance();

            assert!(caller == self.borrower, "only borrower can reimburse");
            assert!(balance == self.amount + self.interest, "can only lend the exact amount");

            self.env().transfer(self.lender, self.amount + self.interest).unwrap_or_default();

        }
        
    }
}
