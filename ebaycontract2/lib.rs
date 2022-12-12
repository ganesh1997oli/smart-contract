#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod ebaycontract2 {

    use ink_prelude::vec::Vec;
    use ink_storage::{
        Mapping,
        traits::{
            SpreadAllocate, SpreadLayout, PackedLayout
        }
    };

    #[derive(SpreadLayout,PackedLayout, Clone, Eq, PartialEq, scale::Decode, scale::Encode, Default)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Status {
        #[default] 
        PENDING,
        DONE, 
    }

    #[derive(scale::Decode, scale::Encode, SpreadLayout, PackedLayout, Default)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Trade {
        trade_id: i32,
        offer_id: i32,
        buyer: AccountId,
        seller: AccountId,
        price: u128,
        status: Status,
    }

    #[derive(scale::Decode, scale::Encode, SpreadLayout, PackedLayout, Default)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Offer {
        offer_id: i32,
        seller: AccountId,
        description: Vec<u8>,
        price: Balance,
        status: Status,
    }

    pub type TradeId = i32;
    pub type OfferId = i32;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Ebaycontract2 {
       trades: Mapping<TradeId, Trade>,
       offers: Mapping<OfferId, Offer>,
       balance: Mapping<AccountId, Balance>,
       available_balance: Mapping<AccountId, Balance>,
       members: Mapping<AccountId, bool>,
       admin: AccountId,
       next_offer_id: i32,
       next_trade_id: i32,
    }

    impl Ebaycontract2 {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|_instance: &mut Ebaycontract2|{
                _instance.admin = _instance.env().caller();
            })
        }

        #[ink(message)]
        pub fn sell(&mut self, description: Vec<u8>, price: u128){
            let caller = self.env().caller();
            let member = self.members.get(caller).unwrap_or_default();

            assert!(member == true, "must be registered");
            assert!(price > 0, "cannot sell free items");

            let offer_id = self.offer_id_next();

            let offer = Offer {
                offer_id,
                seller: caller,
                description,
                price,
                status: Status::PENDING,
            };

            self.offers.insert(offer_id, &offer);

        }

        #[ink(message)]
        pub fn buy(&mut self, offer_id: i32) {
            let caller = self.env().caller();
            let member = self.members.get(caller).unwrap_or_default();

            assert!(member == true, "must be registered");

            let mut offer = self.offers.get(offer_id).unwrap_or_default();
            assert!(offer.offer_id > 0, "offer must exists");
            assert!(offer.status == Status::PENDING, "offer must be pending");
            let available_balance = self.available_balance.get(caller).unwrap_or_default();
            assert!(available_balance >= offer.price, "price must be greater or equal to available balance");
            
            self.available_balance.insert(caller, &(available_balance - offer.price));
            offer.status = Status::DONE;

            let trade_id = self.trade_id_next();
            let trade = Trade {
                trade_id,
                offer_id: offer.offer_id,
                buyer: caller,
                seller: offer.seller,
                price: offer.price,
                status: Status::PENDING,
            };

            self.trades.insert(trade_id, &trade);
        }

        #[ink(message, payable)]
        pub fn deposite(&mut self) {
            let caller = self.env().caller();
            let member = self.members.get(caller).unwrap_or_default();

            assert!(member == true, "must be registered");
            let balance = self.env().transferred_value();
            let acc_balance = self.balance.get(caller).unwrap_or_default();

            self.balance.insert(caller, &(acc_balance + balance));
            
        }


        // Admin functions
        #[ink(message)]
        pub fn settle(&mut self, trade_id: i32) {
            let caller = self.env().caller();
            assert!(caller == self.admin, "only admin");

            let mut trade = self.trades.get(trade_id).unwrap_or_default();
            assert!(trade.trade_id != 0, "trade must exists");
            assert!(trade.status == Status::PENDING, "trade must be in pending state");
            trade.status = Status::DONE;
            let available_balance = self.available_balance.get(caller).unwrap_or_default();
            self.available_balance.insert(caller, &(available_balance + trade.price));

            let balance = self.balance.get(trade.buyer).unwrap_or_default();
            assert!(balance >= trade.price, "cannot transfer more than current balance");
            self.balance.insert(trade.buyer, &(balance - trade.price));

            let available_buyer_balance = self.available_balance.get(trade.buyer).unwrap_or_default();
            self.available_balance.insert(trade.buyer, &(available_buyer_balance - trade.price));

            let sell_balance = self.balance.get(trade.seller).unwrap_or_default();
            self.balance.insert(trade.seller, &(sell_balance + trade.price));

            let available_seller_balance = self.available_balance.get(trade.seller).unwrap_or_default();
            self.available_balance.insert(trade.seller, &(available_seller_balance + trade.price));

        }


        #[ink(message)]
        pub fn register(&mut self, new_member: AccountId) {
            let caller = self.env().caller();
            let member = self.members.get(caller).unwrap_or_default();

            assert!(member == true, "must be registered");
            assert!(caller == self.admin, "only admin");

            self.members.insert(new_member, &true);

            let member_balance = self.balance.get(new_member).unwrap_or_default();
            self.balance.insert(new_member, &(member_balance + 500));

            let member_available_balance = self.available_balance.get(new_member).unwrap_or_default();
            self.available_balance.insert(new_member, &(member_available_balance + 500));

        }

        #[ink(message)]
        pub fn unregister(&mut self, remove_member: AccountId) {
            let caller = self.env().caller();
            let member = self.members.get(caller).unwrap_or_default();

            assert!(member == true, "must be registered");
            assert!(caller == self.admin, "only admin");

            // let member_balance = self.balance.get(remove_member).unwrap_or_default();
            self.members.insert(remove_member, &false);
        }
        

        pub fn trade_id_next(&mut self) -> TradeId {
            let trade_id = self.next_trade_id;
            self.next_trade_id += 1;
            trade_id
        }

        pub fn offer_id_next(&mut self) -> OfferId {
            let offer_id = self.next_offer_id;
            self.next_offer_id += 1;
            offer_id
        }
    }
}
