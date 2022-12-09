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
            let now = Self::env().block_timestamp();
            ink_lang::utils::initialize_contract(|_instance: &mut Dao|{
                _instance.contribution_end = now + contribution_time;
            })
        }

        #[ink(message, payable)]
        pub fn contribute(&mut self){
            assert!(self.env().block_timestamp() < self.contribution_end, "cannot contribute after contribution end");

            let caller = self.env().caller();
            self.investors.insert(caller, &true);

            let balance = self.env().balance();
            let share = self.shares.get(caller).unwrap_or_default();
            self.shares.insert(caller, &(share + balance));

            self.total_shares += balance;
            self.available_funds += balance;
        }

        #[ink(message)]
        pub fn redeem_share(&mut self, amount: Balance){
            let caller = self.env().caller();
            let now = self.env().block_timestamp();

            assert!(now < self.contribution_end, "cannot contribute after contribution end");
            assert!(self.available_funds >= amount, "not enough available funds");

            let share = self.shares.get(caller).unwrap_or_default();

            self.shares.insert(caller, &(share - amount));
            self.available_funds -= amount;
            
            self.env().transfer(caller, amount).unwrap_or_default();

        }

        #[ink(message)]
        pub fn transfer_share(&mut self, amount: Balance, to: AccountId) {
            let caller = self.env().caller();
            let share = self.shares.get(caller).unwrap_or_default();
            assert!(share >= amount, "not enough share");

            self.shares.insert(caller, &(share - amount));
            self.shares.insert(to, &(share + amount));
            self.investors.insert(to, &true);
        }

    }
}
