#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// use ink_lang as ink;

#[openbrush::contract]
mod erc20_crowd_sale {

    use ink_env::debug_print;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp22::{
            Transfer,
            extensions::metadata::*,
        },
        traits::{Storage, String},
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Erc20CrowdSale {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
        owner: AccountId,
        buy_rate: u128,
        hated_account: AccountId,
    }

    #[ink(event)]
    pub struct TokenBuy {
        caller: AccountId,
        value: u128,
        tokens: u128,
    }

    impl Transfer for Erc20CrowdSale {
        // let's override method to reject transactions to bad account
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            to: Option<&AccountId>,
            _amount: &Balance,
        ) -> Result<(), PSP22Error> {
            if to == Some(&self.hated_account) {
                return Err(PSP22Error::Custom(String::from("I hate this account")));
            }
            Ok(())
        }
    }

    impl PSP22 for Erc20CrowdSale {}
    impl PSP22Metadata for Erc20CrowdSale {}

    impl Erc20CrowdSale {
        #[ink(constructor)]
        pub fn new(
            total_supply: u128, 
            buy_rate: u128, 
            name: Option<String>, 
            symbol: Option<String>, 
            decimal: u8
        ) -> Self {
            let caller = Self::env().caller();
            ink_lang::utils::initialize_contract(|instance: &mut Erc20CrowdSale| {
                instance.metadata.name = name;
                instance.metadata.symbol = symbol;
                instance.metadata.decimals = decimal;
                instance.owner = caller;
                instance
                    ._mint_to(caller, total_supply)
                    .expect("Should mint");
                instance.buy_rate = buy_rate;
            })
        }

        #[ink(message, payable)]
        pub fn buy_token(&self) {
            let caller = self.env().caller();
            let value = self.env().transferred_value();
            let balance = self.env().balance();
            debug_print!("my value: {}\n",value);
            debug_print!("contract balance: {}\n",balance);
            // assert!(value > 0, "Transferred value must not be 0");
            let tokens = value * self.buy_rate;
            self.env().transfer(caller, tokens).unwrap();
            self.env().transfer(self.owner, value).unwrap();
            self.env().emit_event(TokenBuy{caller, value, tokens});
        }

        #[ink(message)]
        pub fn set_hated_account(&mut self, hated: AccountId) {
            self.hated_account = hated;
        }

        #[ink(message)]
        pub fn get_hated_account(&self) -> AccountId {
            self.hated_account.clone()
        }
    }
}










