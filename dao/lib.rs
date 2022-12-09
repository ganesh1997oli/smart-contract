#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod dao {
    use ink_storage::{Mapping, traits::SpreadAllocate};

    /// DAO contract:
    /// Collets investors money
    /// Keep track of investor contributions with shares
    /// Allow investors to transfer shares
    /// allow investment proposals to be created and voted
    /// execute successful investment proposal (i.e send money)
   
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Dao {
        investors: Mapping<AccountId, bool>,
        shares: Mapping<AccountId, Balance>,
        total_shares: Balance,
        available_funds: Balance,
        contribution_end: u64,
    }

    impl Dao {
        #[ink(constructor)]
        pub fn new(contribution_time: u64) -> Self {
            ink_lang::utils::initialize_contract(|_instance: &mut Dao|{
                _instance.contribution_end = _instance.env().block_timestamp() + contribution_time;
            })
        }

        #[ink(message, payable)]
        pub fn contribute(&mut self){
            assert!(self.env().block_timestamp() < self.contribution_end, "cannot contribute after contribution end");

            let caller = self.env().caller();
            self.investors.insert(caller, &true);
            self.shares.insert(caller, &(self.shares.get(caller).unwrap_or_default() + self.env().balance()));
            self.total_shares += self.env().balance();
            self.available_funds += self.env().balance();
        }
    }
}
