#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod house_token {
    use ink_storage::{Mapping, traits::{SpreadAllocate, PackedLayout, SpreadLayout}};
    use ink_prelude::vec::Vec;
    use openbrush::contracts::traits::psp34::extensions::mintable::PSP34MintableRef;

    pub type HouseId = i32;

    #[derive(PackedLayout, SpreadLayout, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct House {
        owner: AccountId,
        royalty_collector: AccountId,
        house_address: Vec<u8>,
        sq_feet: u32,
        bed_rooms: i32,
        bathroom: i32,
        price: u32,
        royalty: u32,
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct HouseToken {
        value: bool,
        house: Mapping<HouseId, House>,
        house_exists: Mapping<HouseId, bool>,
        house_next_id: i32,
    }

    impl HouseToken {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            ink_lang::utils::initialize_contract(|_instance: &mut Self|{});
            Self { 
                value: init_value, 
                house: Default::default(), 
                house_exists: Default::default(), 
                house_next_id: Default::default() 
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|_instance: &mut Self|{});
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn mint_house(
            &mut self, 
            house_address: Vec<u8>, 
            sq_feet: u32, 
            bed_rooms: i32, 
            bathroom: i32, 
            price: u32, 
            royalty: u32 
        ) {
            let owner = self.env().caller();
            let house = House {
                owner,
                royalty_collector: Default::default(),
                house_address,
                sq_feet,
                bed_rooms,
                bathroom,
                price,
                royalty,
            };
            let house_id = self.next_house_id();

            self.house.insert(house_id, &house);
            PSP34MintableRef::mint(&owner, house_id);
            
        }

        #[ink(message)]
        pub fn get_house(&self) -> bool {
            self.value
        }

        pub fn next_house_id(&self) -> HouseId {
            let id = self.house_next_id;
            self.house_next_id += 1;
            id
        }
    }
}
