#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod crud {

    use ink_prelude::{
        vec::Vec,
        string::String
    };
    use ink_primitives::Key;
    use ink_storage::traits::{PackedLayout, SpreadLayout, SpreadAllocate,PackedAllocate};
    
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
    pub struct User {
        id: i32,
        name: String,
    }

    impl ink_storage::traits::PackedAllocate for User {
        fn allocate_packed(&mut self, at: &Key){
            PackedAllocate::allocate_packed(&mut *self, at)
        }
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Crud {
        users: Vec<User>,
        next_id: i32,
    }

    impl Crud {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { users: Default::default(), next_id: Default::default() }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        #[ink(message)]
        pub fn create(&mut self, name: String) {
            let new_user = User {
                id: self.next_id + 1,
                name
            };

            self.users.push(new_user);
        }

        #[ink(message)]
        pub fn read(&self) -> Vec<User> {
            self.users.clone()
        }

        #[ink(message)]
        pub fn read_one(&self, id: i32) -> User {
            self.users[id as usize].clone()
        }

        #[ink(message)]
        pub fn update(&mut self, id: i32, name: String) {
            let mut usr = self.users[id as usize].name.clone();
            usr = name;
        }

        #[ink(message)]
         pub fn destroy(&mut self, id: i32) { }

    }
    
}
