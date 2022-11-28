#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod blindauction {
    use ink_storage::{traits::SpreadAllocate, Mapping};


    #[derive(scale::Decode, scale::Encode, Default)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo) )]
    pub struct Bid {
        blinded_bid: i32,
        deposit: u32,
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Blindauction {
        beneficiary: AccountId,
        bidding_end: u32,
        reveal_end: u32,
        ended: bool,
        bid: Mapping<AccountId, Bid>,
        highest_bidder: AccountId,
        highest_bid: u32,
        pending_reaturns: u32,
    }

    impl Blindauction {
        #[ink(constructor)]
        pub fn new(bidding_end: u32, reveal_end: u32, beneficiary: AccountId) -> Self {
            ink_lang::utils::initialize_contract(|_contract: &mut Self|{
                _contract.bidding_end = bidding_end;
                _contract.reveal_end = reveal_end;
                _contract.beneficiary = beneficiary;
                _contract.ended = Default::default();
                _contract.bid = Mapping::default();
                _contract.highest_bidder = Default::default();
                _contract.highest_bid = Default::default();
                _contract.pending_reaturns = Default::default();
            })
        }

        #[ink(message)]
        pub fn bid(&self){}

    }

}
