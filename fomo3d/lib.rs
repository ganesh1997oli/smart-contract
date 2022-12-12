#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod fomo3d {
    use ink_storage::{traits::{
        SpreadAllocate, 
        SpreadLayout,
    }, Mapping};
    use ink_prelude::vec::Vec;


    #[derive( SpreadLayout)]
    #[cfg_attr(feature = "std", derive(Debug, Clone,scale::Decode, scale::Encode, Copy,scale_info::TypeInfo, ink_storage::traits::StorageLayout))]
    pub enum State {
        INACTIVE,
        ACTIVE,
    }

    impl SpreadAllocate for State {
        fn allocate_spread(_: &mut ink_primitives::KeyPtr) -> Self { State::INACTIVE } 
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Fomo3d {
        state: State,
        admin: AccountId,
        start: u64,
        end: u64,
        hard_end: u64,
        pot: i32,
        house_fee: i32,
        initial_key_price: i32,
        total_keys: i32,
        key_holders: Vec<AccountId>,
        keys: Mapping<AccountId, i32>,
    }

    impl Fomo3d {
        #[ink(constructor)]
        pub fn new() -> Self {
            let state = State::INACTIVE;

            ink_lang::utils::initialize_contract(|_instance: &mut Fomo3d|{
                _instance.state = state;
            })
        }

        #[ink(message)]
        pub fn create_round(&mut self){}
    }
}
