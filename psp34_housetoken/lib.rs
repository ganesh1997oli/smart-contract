#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34_mintable {
    use ink_storage::{traits::{
        SpreadAllocate,
        SpreadLayout,
    }, Mapping};
    use openbrush::{
        contracts::psp34::extensions::mintable::*,
        traits::Storage,
    };

    #[derive(SpreadLayout, ink_storage::traits::PackedLayout, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo) )]
    pub struct  House {
        owner: AccountId,
        bed_rooms: i32,
    }

    pub type HouseId = u8;

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
        house: Mapping<HouseId, House>
    }

    impl PSP34 for Contract {}

    impl PSP34Mintable for Contract {}

    impl Contract {
        /// The constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {});
            Self {
                psp34: Default::default(),
                house: Default::default(),
            }
        }

        #[ink(message)]
        pub fn mint_house(
            &mut self,
            bed_rooms: i32
        ) {
            let owner = self.env().caller();
            let house = House {
                owner,
                bed_rooms,
            };
            let house_id = 1;

            self.house.insert(house_id, &house);

            let account_id = self.env().account_id();
            self.mint(account_id, Id::U8(house_id)).unwrap();
        }

        #[ink(message)]
        pub fn get_house(&self, house_id: u8) -> Option<House> {
            self.house.get(&house_id)
        }
    }
}