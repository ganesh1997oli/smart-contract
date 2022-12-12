#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod eventcontract {
    use ink_storage::{Mapping, traits::{
        SpreadAllocate,
        PackedLayout,
        SpreadLayout,
        StorageLayout
    }};

    use ink_prelude::string::String;


    #[derive(Default, Eq, PartialEq, scale::Decode, scale::Encode, PackedLayout, SpreadLayout, StorageLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Event {
       admin: AccountId,
       name: String,
       date: u64,
       price: u128,
       ticket_count: u128,
       ticket_remaining: u128,
    }

    pub type EventId = i32;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Eventcontract {
        events: Mapping<EventId, Event>,
        tickets: Mapping<(AccountId, EventId), u128>,
        event_next_id: i32,
    }

    impl Eventcontract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|_instance: &mut Eventcontract|{})
        }

        #[ink(message)]
        pub fn create_event(&mut self, name: String, date: u64, price: u128, ticket_count: u128){
            let now = self.env().block_timestamp();
            let admin = self.env().caller();

            assert!(date > now, "can only organize event at a future date");
            assert!(ticket_count > 0, "can only organize event with at least 1 ticket");

            let event = Event {
                admin,
                name,
                date,
                price,
                ticket_count,
                ticket_remaining: ticket_count
            };

            let event_id = self.next_id();
            self.events.insert(event_id, &event);
        }

        #[ink(message)]
        pub fn buy_ticket(&mut self, id: i32, quantity: u128) {
            let mut event = self.events.get(id).unwrap_or_default();
            let now = self.env().block_timestamp();
            let balance = self.env().transferred_value();
            let caller = self.env().caller();

            assert!(event.date != 0, "this event doesn't exist");
            assert!(now < event.date, "this event is not active anymore");
            assert!(balance == (event.price * quantity), "value must be total ticket cost");
            assert!(event.ticket_remaining >= quantity, "not enough ticket left");

            event.ticket_remaining -= quantity;

            let ticket = self.tickets.get((caller, id)).unwrap_or_default();
            self.tickets.insert((caller, id), &(ticket + quantity));
        }

        #[ink(message)]
        pub fn transfer_ticket(&mut self, event_id: i32, quantity: u128, to: AccountId) {
            let event = self.events.get(event_id).unwrap_or_default();
            let now = self.env().block_timestamp();
            let caller = self.env().caller();

            assert!(event.date != 0, "this event doesn't exists");
            assert!(now < event.date, "event must be active");

            let ticket = self.tickets.get((caller, event_id)).unwrap_or_default();
            assert!(ticket > quantity, "not enough ticket");

            self.tickets.insert((to, event_id), &(ticket + quantity));
        }

        pub fn next_id(&mut self) -> EventId {
            let id = self.event_next_id;
            self.event_next_id += 1;
            id
        }
    }
}
