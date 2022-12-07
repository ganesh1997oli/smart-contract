#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod simplestorage {

    use ink_prelude::{
        vec::Vec,
        string::String
    };

    #[ink(storage)]
    pub struct Simplestorage {
        value: bool,
        data: String,
        users: Vec<String>,

    }

    impl Simplestorage {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value, data: Default::default(), users: Default::default() }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn add_user(&mut self, users: String) {
            self.users.push(users)
        }

        #[ink(message)]
        pub fn get_users(&self) -> Vec<String> {
            self.users.clone()
        }

        #[ink(message)]
        pub fn set_data(&mut self, data: String) {
            self.data = data
        }

        // #[ink(message)]
        // pub fn user_length(&self) -> usize {
        //     self.users.len()
        // }

        #[ink(message)]
        pub fn get_data(&self) -> String {
            self.data.clone()
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}
