#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod votingcontract {

    use ink_prelude::vec::Vec;
    use ink_storage::{traits::{
        SpreadLayout, 
        PackedLayout, 
        SpreadAllocate,
        StorageLayout,
    }, 
    Mapping,
};
    use scale::{Encode, Decode};

    #[derive(Encode, Decode)]
    #[cfg_attr(
        feature = "std", 
        derive(Debug, PartialEq, Eq, scale_info::TypeInfo)
    )] 
       pub enum VoteType {
        Aya,
        Nya,
    }

    #[derive( Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum VotingcontractError {
        AlreadyVoted,
    }

    #[derive(
        Encode, 
        Decode, 
        SpreadLayout, 
        PackedLayout, 
        SpreadAllocate, 
        Default
    )]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, scale_info::TypeInfo, StorageLayout)
    )]
    pub struct Proposal {
        name: Vec<u8>,
    }

    #[derive(
        Encode, 
        Decode, 
        SpreadLayout, 
        PackedLayout, 
        SpreadAllocate,
        Default,
    )]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, scale_info::TypeInfo, StorageLayout)
    )]
    pub struct ProposalVote {
        against_votes: u8,
        for_votes: u8,
    }

    pub type ProposalId = u32;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Votingcontract {
        proposal_votes: Mapping<ProposalId, ProposalVote>,
        proposals: Mapping<ProposalId, Proposal>,
        votes: Mapping<(ProposalId, AccountId), ()>,
        next_proposal_id: u32,
    }

    impl Votingcontract {
        #[ink(constructor)]
        pub fn new(name: Vec<u8>) -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Votingcontract|{
                let proposal = Proposal { name};
                let proposal_id = contract.next_proposal_id();
                contract.proposals.insert(proposal_id, &proposal);
            })
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|_contract: &mut Votingcontract|{});
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn vote(&mut self, proposal_id: ProposalId, vote: VoteType) -> Result<(),VotingcontractError >{
            let caller = self.env().caller();

            if self.votes.get(&(proposal_id, caller)).is_some() {
                return Err(VotingcontractError::AlreadyVoted)
            }

            let mut proposal_vote = self.proposal_votes.get(&proposal_id).unwrap_or_default();

            match vote {
                VoteType::Aya => {
                    proposal_vote.for_votes += 1;
                }

                VoteType::Nya => {
                    proposal_vote.against_votes += 1;
                }
            }

            self.proposal_votes.insert(proposal_id, &proposal_vote);
            Ok(())
        }

        #[ink(message)]
        pub fn get_proposal_vote(&mut self, proposal_id: ProposalId) -> Option<ProposalVote> {
            self.proposal_votes.get(&proposal_id)
        }

        #[ink(message)]
        pub fn get_proposal(&mut self, proposal_id: ProposalId) -> Option<Proposal> {
            self.proposals.get(&proposal_id)
        }

        fn next_proposal_id(&mut self) -> ProposalId {
            let id = self.next_proposal_id;
            self.next_proposal_id += 1;
            id
        }
        
    }
}
