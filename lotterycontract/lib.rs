#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod lotterycontract {
    use ink_env::AccountId;
    use ink_storage::{Mapping, traits::{SpreadAllocate, SpreadLayout, PackedLayout, StorageLayout, PackedAllocate}};
    use scale::{Encode, Decode};
    use ink_prelude::vec::Vec;
    // use ink_primitives::Key;
    
    #[derive(
        Encode, 
        Decode, 
        SpreadLayout, 
        PackedLayout, 
        SpreadAllocate,
        Default,
    )]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, scale_info::TypeInfo, StorageLayout)
    )]
    pub struct Lottery {
        lottery_name: Vec<u8>,
        lottery_manager: AccountId,
    }

    // impl ink_storage::traits::PackedAllocate for Lottery {
    //     fn allocate_packed(&mut self, at: &Key){
    //         PackedAllocate::allocate_packed(&mut *self, at)
    //     }
    // }

    pub type LotteryId = u32;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Lotterycontract {
        my_lottery: Mapping<LotteryId, Lottery>,
        next_lottery_id: u32,
        is_lottery_live: bool,
        lottery_bag: Vec<AccountId>
    }

    impl Lotterycontract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|_instance: &mut Self|{
                _instance.my_lottery = Mapping::default();
                _instance.is_lottery_live = false;
                _instance.lottery_bag = Default::default();
            })
        }

        #[ink(message)]
        pub fn create_lottery(&mut self, lottery_name: Vec<u8>) {
            let lottery_manager = self.env().caller();
            let lottery = Lottery {
                lottery_name,
                lottery_manager
            };

            let lottery_id = self.next_lottery_id();
            self.my_lottery.insert(lottery_id, &lottery);
            self.is_lottery_live = true;
        }

        #[ink(message)]
        pub fn participate(&mut self, player: AccountId) {
            self.lottery_bag.push(player)
        }

        #[ink(message)]
        pub fn declare_winner(&mut self) -> AccountId {
            let winner = self.lottery_bag.get(self.lottery_bag.len() % 2).unwrap();
            self.is_lottery_live = false;
            winner
        }

        fn next_lottery_id(&mut self) -> LotteryId {
            let id = self.next_lottery_id;
            self.next_lottery_id += 1;
            id
        }
    }
}
