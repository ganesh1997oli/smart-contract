#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod votingI {

    use ink_prelude::{
        vec::Vec,
        string::String
    };

    use ink_storage::{
        traits::{
            SpreadAllocate,
            PackedLayout,
            SpreadLayout,
        },
        Mapping
    };


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
        Default,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Choice {
        id: i32,
        name: String,
        votes: i32,
    }

    #[derive(
        PartialEq,
        Debug,
        Eq,
        Clone,
        scale::Encode, 
        scale::Decode, 
        SpreadLayout, 
        PackedLayout, 
        Default,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Ballot {
        id: i32,
        name: String,
        choices: Vec<Choice>,
        end: u64
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct VotingI {
        voters: Mapping<AccountId, bool>,
        ballots: Mapping<i32, Ballot>,
        next_ballot_id: i32,
        admin: AccountId,
        votes: Mapping<AccountId, (i32, bool)>
    }

    impl VotingI {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();

            ink_lang::utils::initialize_contract(|_instance: &mut VotingI|{
                _instance.admin = caller;
            })
        }

        #[ink(message)]
        pub fn add_voters(&mut self, voters: Vec<AccountId>){
            for i in voters {
                self.voters.insert(i, &true);
            }
        }

        #[ink(message)]
        pub fn create_ballot(&mut self, name: String, choices: Vec<String>, offset: u64){
            let mut ballot = self.ballots.get(self.next_ballot_id).unwrap();
            ballot.id = self.next_ballot_id;
            ballot.name = name;
            ballot.end = self.env().block_timestamp() + offset;

            for i in 0..choices.len() {
                ballot.choices.push(Choice {id: i as i32, name: choices[i].clone(), votes: 0})
            }

            self.next_ballot_id += 1;
        }

        #[ink(message)]
        pub fn vote(&mut self, ballot_id: i32, choice_id: i32) {
            let caller = self.env().caller();
            let voters = self.voters.get(caller).unwrap_or_default();
            let votes = self.votes.get(caller).unwrap_or_default();
            assert!(voters == true, "only voters can vote");
            assert!(votes == (ballot_id, false), "voter can only vote once for a ballot");

            let now = self.env().block_timestamp();
            let ballots = self.ballots.get(ballot_id).unwrap();
            assert!(now < ballots.end, "can only vote until ballot end date");

            self.votes.insert(caller, &(ballot_id, true));

            let mut ballots = self.ballots.get(ballot_id).unwrap_or_default();
            ballots.choices[choice_id as usize].votes += 1;
        }

        #[ink(message)]
        pub fn results(&mut self, ballot_id: i32) -> Vec<Choice> {
            let ballot = self.ballots.get(ballot_id).unwrap_or_default();
            ballot.choices
        }
        
    }
}
