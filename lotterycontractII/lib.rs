#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod lotterycontract_ii {
    use ink_storage::traits::{
        SpreadLayout, 
    };

    use ink_prelude::vec::Vec;


    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Default,SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout))]
    pub enum State {
        #[default] 
        IDLE,
        BETTING,
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct LotterycontractIi {
        players: Vec<AccountId>,
        state: State,
        bet_count: i32,
        bet_size: u128,
        house_fee: u128,
        admin: AccountId,
    }

    impl LotterycontractIi {
        #[ink(constructor)]
        pub fn new(fee: u128) -> Self {
            let admin = Self::env().caller();
            let state = State::IDLE;
            assert!((fee > 1 && fee < 99),"fee should be between 1 and 99");
            Self {
                admin,
                house_fee: fee,
                state,
                ..Self::default()
            }
            
        }

        #[ink(message)]
        pub fn create_bet(&mut self, bet_count: i32, bet_size: u128){
            let caller = self.env().caller();

            assert!(self.state == State::IDLE, "current state doesn't allow this");
            assert!(caller == self.admin, "only admin");
            self.bet_count = bet_count;
            self.bet_size = bet_size;
            self.state = State::BETTING;
        }

        #[ink(message)]
        pub fn bet(&mut self){
            let caller = self.env().caller();
            assert!(self.state == State::BETTING, "current state doesn't allow this");
            
            let balance = self.env().transferred_value();
            assert!(balance == self.bet_size, "can only bet the exactly bet size");

            self.players.push(caller);

            if self.players.len() == self.bet_count as usize {
                let additional_randomness = b"seed";
                let winner = self.env().random(additional_randomness);

                // TODO:: INCOMPLETE function
            }
        }

        #[ink(message)]
        pub fn cancel(&mut self){
            assert!(self.state == State::BETTING, "current state doesn't allow this");

            let caller = self.env().caller();
            assert!(caller == self.admin);

            self.state = State::IDLE;

            // TODO:: INCOMPLETE function
        }
    }
}
