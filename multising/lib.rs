#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod multising {
    use ink_storage::{Mapping, traits::{SpreadAllocate, PackedLayout, SpreadLayout}};
    use ink_prelude::vec::Vec;

    #[derive(PackedLayout, SpreadLayout, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo) )]
    pub struct Transfer {
        id: i32,
        amount: i32,
        to: AccountId,
        approvals: i32,
        sent: bool
    }
    
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Multising {
        value: bool,
        transfer: Mapping<i32, Transfer>,
        approvers: Vec<u8>,
        quorum: i32,
        approvals: Mapping<(AccountId, i32), bool>,
        next_id: i32,
    }

    impl Multising {
        #[ink(constructor)]
        pub fn new(approvers: Vec<u8> ,quorum: i32) -> Self {
            ink_lang::utils::initialize_contract(|_instance: &mut Multising|{
                _instance.approvers = approvers;
                _instance.quorum = quorum;
            })
        }

        #[ink(message)]
        pub fn create_transfer(&mut self, amount: i32, to: AccountId){
            let transfer = Transfer {
                id: self.next_id,
                amount,
                to,
                approvals: 0,
                sent: false
            };

            self.transfer.insert(self.next_id, &transfer);
            self.next_id += 1;
        }

        #[ink(message)]
        pub fn send_transfer(&mut self, id: i32) {
            let caller = self.env().caller(); 

            let mut transfer = self.transfer.get(id).unwrap();
            assert!(transfer.sent == false, "Transfer has already been sent" );

            // approvals: Mapping<(AccountId, i32), bool>, // I'm trying to read this mapping but got this issue
            let mut _approvals = self.approvals.get((caller, id)).unwrap();

            if _approvals == false {
                _approvals = true;
                transfer.approvals += 1;
            }
            if transfer.approvals >= self.quorum {
                transfer.sent = true;
                let to = transfer.to;
                let amount = transfer.amount;
                self.env().transfer(to, amount as u128).unwrap_or_default()

            }
        }
    }
}
