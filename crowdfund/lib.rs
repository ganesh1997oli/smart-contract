#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod crowdfund {
    use ink_prelude::vec::Vec;
    use ink_storage::{
        Mapping, 
        traits::{
            SpreadLayout, 
            PackedLayout,
            SpreadAllocate,
        }
    };

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
    pub struct Campagin {
        id: i64,
        creator: AccountId,
        goal: i64,
        pledge: i64,
        start_at: i64,
        end_at: i64,
        claimed: bool,
    }

    impl Default for Campagin {
        fn default() -> Campagin {
            Campagin {
                id: Default::default(),
                creator: Default::default(),
                goal: Default::default(),
                pledge: Default::default(),
                // pledge_amount: Default::default(),
                start_at: Default::default(),
                end_at: Default::default(),
                claimed: Default::default(),
            }
        }
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Crowdfund {
        campagin: Mapping<i64, Campagin>,
        pledge_amount: Mapping<AccountId, i64>,
        all_campagins: Vec<i64>,
    }

    #[ink(event)]
    pub struct CampaginCreated {
        id: i64,
    }

    impl Crowdfund {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|_instance: &mut Crowdfund|{});
            Self {
                campagin: Default::default(),
                pledge_amount: Default::default(),
                all_campagins: Default::default(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|_instance: &mut Crowdfund|{});
            Self::new()
        }

        #[ink(message)]
        pub fn create(&mut self, goal: i64, pledge: i64, start_at: i64, end_at: i64) {
            let caller = Self::env().caller();
            let id = (self.all_campagins.len() + 1) as i64;
            let campagin = Campagin{
                id,
                creator: caller,
                goal,
                pledge,
                start_at,
                end_at,
                claimed: false,
            };
            self.campagin.insert(id, &campagin);
            self.all_campagins.push(id);
            self.env().emit_event(CampaginCreated{
                id,
            })
        }

        #[ink(message)]
        pub fn get(&self, id: i64) -> Option<Campagin> {
            self.campagin.get(&id)
        }

        #[ink(message)]
        pub fn remove(&mut self, id: i64) {
            self.campagin.remove(&id);
            // let mut campagin = self.campagin.get(&id).unwrap();
            
        }

        #[ink(message)]
        pub fn pledge(&mut self, id: i64, amount: i64, recipent: AccountId) {
            let mut campagin = self.campagin.get(&id).unwrap();
            campagin.pledge += amount;
            let pamount = campagin.pledge;
            self.pledge_amount.insert(Self::env().caller(), &pamount);
            self.env().transfer(recipent, pamount as u128);
        }

        #[ink(message)]
        pub fn unpledge(&mut self, id: i64, amount: i64) {
            let mut campagin = self.campagin.get(&id).unwrap();
            campagin.pledge -= amount;
        }

        #[ink(message)]
        pub fn claim(&mut self, id: i64) {
            let mut campagin = self.campagin.get(&id).unwrap();
            campagin.claimed = true;
        }

        #[ink(message)]
        pub fn refund(&mut self) {
            let plead_amount = self.pledge_amount.get(self.env().caller()).unwrap();
            self.env().transfer(self.env().caller(), plead_amount as u128);
            self.pledge_amount.remove(self.env().caller());
        }
            
    }
}
