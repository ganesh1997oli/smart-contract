#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod spliitpayment {

    use openbrush::{
        modifiers,
        traits::Storage, contracts::ownable::{only_owner, self}
    };

    #[ink(storage)]
    #[derive(Storage)]
    pub struct Spliitpayment {
        owner: AccountId,
        ownable: ownable::Data,
    }

    impl Spliitpayment {
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            ink_lang::utils::initialize_contract(|_instance: &mut Spliitpayment|{
                let caller = _instance.env().caller();
                _instance._init_with_owner(caller);
                _instance.owner = owner;
            })
        }

        

        #[ink(message, payable)]
        #[modifiers(only_owner)]
        pub fn send(&mut self, to: Vec<u8>, amount: Vec<u8>) {
            assert!(to.len() == amount.len());
            for i in to {
                self.env().transfer(to[i], amount[i]);
            }
        }
    }

    
}
