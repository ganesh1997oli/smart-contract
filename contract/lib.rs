#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod contract {
    use ink_storage::{Mapping, traits::{SpreadAllocate, PackedLayout, SpreadLayout}};


    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotExist,
        NotFalse,
    }

    #[derive(Default, scale::Encode, scale::Decode, Debug, PartialEq, Eq, PackedLayout, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout))]
    struct CustomBool {
        boolean: bool,

    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Contract {
        /// Stores a single `bool` value on the storage.
        value: u128,
        mappings: Mapping<u128, CustomBool>,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(init_value: u128) -> Self {
            ink_lang::utils::initialize_contract(|_instance: &mut Contract|{
                _instance.value = init_value;
                _instance.mappings = Mapping::default();
            })
        }

        #[ink(message)]
        pub fn insert_value(&mut self, boolean: bool) -> Result<()> {
            let custom_bool = CustomBool { boolean };
            self.mappings.insert(self.value, &custom_bool);
            self.value += 1;
            Ok(())
        }

        #[ink(message)]
        pub fn change_value(&mut self, id: u128) -> Result<()> {
            let custom_bool = self.mappings.get(id);
            match custom_bool {
                None => return Err(Error::NotExist),
                Some(v) => {
                    if !v.boolean {
                        let custom = CustomBool { boolean: true };
                        self.mappings.insert(id, &custom);
                    }
                }
            }
            Ok(())
        }

        #[ink(message)]
        pub fn get(&self, id: u128) -> bool {
            self.mappings.get(id).unwrap().boolean
        }
    }

}
