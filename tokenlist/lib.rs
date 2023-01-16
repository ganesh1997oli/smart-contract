#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod tokenlist {
    use ink_storage::{Mapping, traits::SpreadAllocate};
    use ink_prelude::{
        vec::Vec,
        string::String
    };

    pub type NID = i32;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Tokenlist {
        value: bool,
        name: Mapping<NID, String>,
        nid: NID,
    }

    impl Tokenlist {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Tokenlist|{
                instance.value = init_value;
                instance.name = Mapping::default();
                instance.nid = Default::default();
            })
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|_instance: &mut Tokenlist|{});
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn add_name(&mut self, name: String) {
            self.name.insert(self.nid, &name);
            self.nid += 1;
        }

        #[ink(message)]
        pub fn get_name(&self) -> Vec<String> {
            let mut result: Vec<String> = Vec::new();
            for i in 0..self.nid {
                match self.name.get(i) {
                    Some(value) => result.push(value),
                    None => ()
                }
            }
            result
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

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let tokenlist = Tokenlist::default();
            assert_eq!(tokenlist.get(), false);
        }

        #[ink::test]
        fn it_works() {
            let mut tokenlist = Tokenlist::new(false);
            assert_eq!(tokenlist.get(), false);
            tokenlist.flip();
            assert_eq!(tokenlist.get(), true);
        }
    }
}
