#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod ebaycontract {
    use ink_storage::{Mapping, traits::{SpreadAllocate, PackedLayout, SpreadLayout}};
    use ink_prelude::{
        vec::Vec,
        string::String,
    };

    /// Allow seller to create auction
    /// Allow buyers make offer for an auctions
    /// Allow seller and buyers to trade at the end of the auction
    /// Create some getter functions for auctions and offer

    pub type AuctionId = i32;
    pub type OfferId = i32;

    #[derive(Clone, scale::Decode, scale::Encode, PackedLayout, SpreadLayout, Default)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Auction {
        id: AuctionId,
        seller: AccountId,
        name: String,
        description: String,
        min: u128,
        end: u64,
        best_offer_id: OfferId,
        offer_ids: Vec<OfferId>
    }

    #[derive(Clone, scale::Decode, scale::Encode, PackedLayout, SpreadLayout, Default)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Offer {
        id: OfferId,
        auction_id: AuctionId,
        buyer: AccountId,
        price: u128,
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Ebaycontract {
       auctions: Mapping<AuctionId, Auction>,
       offers: Mapping<OfferId, Offer>,
       user_auctions: Mapping<AccountId, Vec<Auction>>,
       user_offer: Mapping<AccountId, Vec<Offer>>,
       next_auction_id: i32,
       next_offer_id: i32,
    }

    impl Ebaycontract {
        #[ink(constructor)]
        pub fn new() -> Self { 
            ink_lang::utils::initialize_contract(|_instance: &mut Ebaycontract|{})
        }

        #[ink(message)]
        pub fn create_auction(&mut self, name: String, description: String, min: u128, end: u64){
            assert!(min > 0, "min must be > 0");
            assert!((end > 86400 && end < 864000), "duration must be between 1 to 10 days");

            let offer_ids: Vec<OfferId> = Vec::new();
            let auction_id = self.auction_next_id();
            let caller = self.env().caller();

            let auction = Auction {
                id: auction_id,
                seller: caller,
                name,
                description,
                min,
                end, 
                best_offer_id: 0,
                offer_ids,
            };

            self.auctions.insert(auction_id, &auction);
            self.user_auctions.insert(caller, &vec![auction]);
        }

        #[ink(message, payable)]
        pub fn create_offer(&mut self, auction_id: i32) {
            assert!((auction_id > 0 && auction_id < self.next_auction_id), "auction deosn't exists");
            let mut auction = self.auctions.get(auction_id).unwrap_or_default();
            let best_offer = self.offers.get(auction.best_offer_id).unwrap_or_default();
            let now = self.env().block_timestamp();
            let balance = self.env().transferred_value();
            
            assert!(now < auction.end, "auction has expired");
            assert!((balance >= auction.min && balance > best_offer.price), "balance must be superiod to min and bestoffer");

            let offer_id = self.offer_next_id();
            auction.best_offer_id = offer_id;
            auction.offer_ids.push(offer_id);

            let caller = self.env().caller();
            let offer = Offer {
                id: offer_id,
                auction_id,
                buyer: caller,
                price: balance,
            };

            self.offers.insert(offer_id, &offer);
            self.user_offer.insert(caller, &vec![offer]);

        }

        #[ink(message)]
        pub fn trade(&mut self, auction_id: i32) {
            assert!((auction_id > 0 && auction_id < self.next_auction_id), "auction deosn't exists");
            let auction = self.auctions.get(auction_id).unwrap_or_default();
            let best_offer = self.offers.get(auction.best_offer_id).unwrap_or_default();
            let now = self.env().block_timestamp();
            
            assert!(now > auction.end, "auction still active");

            let i = 0;
            for i in i..auction.offer_ids.len() {
                let offer_id = auction.offer_ids[i];

                if offer_id != auction.best_offer_id {
                    let offer = self.offers.get(offer_id).unwrap_or_default();
                    self.env().transfer(offer.buyer, offer.price).unwrap_or_default();
                }
            }

            self.env().transfer(auction.seller, best_offer.price).unwrap_or_default();
        }

        #[ink(message)]
        pub fn get_auction(&mut self, auction_id: i32) -> Option<Auction> {
            self.auctions.get(auction_id)
        }

        #[ink(message)]
        pub fn get_user_auction(&mut self, account_id: AccountId) -> Vec<Auction> {
            let user_auction = self.user_auctions.get(account_id).unwrap_or_default();
            user_auction
        }

        #[ink(message)]
        pub fn get_user_offer(&mut self, account_id: AccountId) -> Vec<Offer> {
            let user_offer = self.user_offer.get(account_id).unwrap_or_default();
            user_offer
        }

        pub fn auction_next_id(&mut self) -> AuctionId {
            let id = self.next_auction_id;
            self.next_auction_id += 1;
            id
        }

        pub fn offer_next_id(&mut self) -> OfferId {
            let id = self.next_offer_id;
            self.next_offer_id += 1;
            id
        }
    }
}
