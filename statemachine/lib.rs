#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod statemachine {
    use ink_storage::traits::{SpreadLayout, StorageLayout};


    #[derive(SpreadLayout, StorageLayout, Debug, scale::Decode, scale::Encode, PartialEq, Eq)]
    #[cfg_attr(feature="std", derive(scale_info::TypeInfo))]
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
       end: u64,
       borrower: AccountId,
       lender: AccountId,
    }

    impl Statemachine {
        #[ink(constructor)]
        pub fn new(amount: u128, interest: u128, end: u64, borrower: AccountId, lender: AccountId) -> Self {
            Self { 
                state: State::PENDING,
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

        #[ink(message)]
        pub fn transaction_to(&mut self, state: State, duration: u64) {
           assert!(state != State::PENDING, "cannot go back to pending");
           assert!(state != self.state, "cannot transaction to same state");

           if state == State::ACTIVE {
            assert!(self.state == State::PENDING,"cannot only go to active from pending");
            self.state = State::ACTIVE;

            let now = self.env().block_timestamp();
            self.end = now + duration;
           }

           if state == State::CLOSED {
            assert!(self.state == State::ACTIVE, "cannot only go to closed from active");

            let now = self.env().block_timestamp();
            assert!(now >= self.end, "loan hasn't matured yet");
            self.state = State::CLOSED;
           }
        }
        
    }
}
