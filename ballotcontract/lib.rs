#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod ballotcontract {
    use ink_storage::{Mapping, traits::{SpreadAllocate, SpreadLayout, PackedLayout}};

    #[derive(
        PartialEq,
        Debug,
        Eq,
        Clone,
        scale::Encode, 
        scale::Decode, 
        SpreadLayout, 
        PackedLayout, 
        SpreadAllocate,
        Copy,
    )]
    #[cfg_attr(
        feature = "std",
        derive(
            scale_info::TypeInfo,
            ink_storage::traits::StorageLayout
        )
    )]
    pub struct Voter {
        weight: i32,
        voted: bool,
        delegate: AccountId,
        vote: i32,
    }

    #[derive(
        PartialEq,
        Debug,
        Eq,
        Clone,
        scale::Encode,
        scale::Decode,
        SpreadLayout,
        PackedLayout,
        SpreadAllocate,
    )]
    #[cfg_attr(
        feature = "std", 
        derive(
            scale_info::TypeInfo, 
            ink_storage::traits::StorageLayout,
        )
    )]
    pub struct Proposal {
        name: Vec<u8>,
        vote_count: i32,
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Ballotcontract {
        creator: AccountId,
        voter: Mapping<AccountId, Voter>,
        proposal: Vec<Proposal>
    }

    impl Ballotcontract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Ballotcontract|{});
            let creator = Self::env().caller();

            Self { 
                creator,
                voter: Default::default(),
                // proposal: Default::default(),
             }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Ballotcontract|{});
            Self::new()
        }

        #[ink(message)]
        pub fn add_proposal(&mut self, proposal: Vec<u8>) {
            // for i in &proposal {
            //     self.proposal.push(Proposal {
            //         name: vec![i.clone()],
            //         vote_count: 0,
            //     });
            // }
        }
    }
}
