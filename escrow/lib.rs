#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod escrow {


    #[ink(storage)]
    pub struct Escrow {
        payer: AccountId,
        payee: AccountId,
        lawyer: AccountId,
        amount: u128,
    }

    impl Escrow {
        #[ink(constructor)]
        pub fn new(payer: AccountId, payee: AccountId, amount: u128) -> Self {
            let caller = Self::env().caller();
            Self { 
                payer,
                payee,
                lawyer: caller,
                amount
             }
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self) {
            assert!(self.env().caller() == self.payer, "Sender must be the payer");
            assert!(self.env().balance() <= self.amount, "Can't send more than escrow amount");
        }

        #[ink(message)]
        pub fn release(&mut self){
            assert!(self.env().balance() == self.amount, "can't release fund before full amount is sent");
            assert!(self.env().caller() == self.lawyer, "Only lawyer can release fund");
            self.env().transfer(self.payee, self.amount).unwrap_err();
        }

        #[ink(message)]
        pub fn balance_of(&self) -> u128 {
            self.env().balance()
        }
    }
}
