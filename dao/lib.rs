#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod dao {

    use ink_storage::{
        Mapping, 
        traits::{
            SpreadAllocate,
            SpreadLayout,
            PackedLayout
        }
    };
    use ink_prelude::vec::Vec;

    /// DAO contract:
    /// Collets investors money
    /// Keep track of investor contributions with shares
    /// Allow investors to transfer shares
    /// allow investment proposals to be created and voted
    /// execute successful investment proposal (i.e send money)
    
    #[derive(SpreadLayout, PackedLayout, scale::Decode, scale::Encode, Default)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Proposal {
        name: Vec<u8>,
        amount: Balance,
        recipient: AccountId,
        votes: u128,
        end: u64,
        execuated: bool,
    }

    pub type ProposalId = i32;
   
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Dao {
        investors: Mapping<AccountId, bool>,
        shares: Mapping<AccountId, Balance>,
        total_shares: Balance,
        available_funds: Balance,
        contribution_end: u64,
        proposals: Mapping<ProposalId, Proposal>,
        votes: Mapping<AccountId, (ProposalId, bool)>,
        next_proposal_id: i32,
        vote_time: Timestamp,
        quorum: u128,
        admin: AccountId,
    }

    impl Dao {
        #[ink(constructor)]
        pub fn new(contribution_time: u64, vote_time: Timestamp, quorum: u128) -> Self {
            let now = Self::env().block_timestamp();
            let caller = Self::env().caller();

            ink_lang::utils::initialize_contract(|_instance: &mut Dao|{
                _instance.contribution_end = now + contribution_time;
                _instance.vote_time = vote_time;
                _instance.quorum = quorum;
                _instance.admin = caller;
            })
        }

        #[ink(message, payable)]
        pub fn contribute(&mut self){
            assert!(self.env().block_timestamp() < self.contribution_end, "cannot contribute after contribution end");

            let caller = self.env().caller();
            self.investors.insert(caller, &true);

            let balance = self.env().balance();
            let share = self.shares.get(caller).unwrap_or_default();
            self.shares.insert(caller, &(share + balance));

            self.total_shares += balance;
            self.available_funds += balance;
        }

        #[ink(message)]
        pub fn redeem_share(&mut self, amount: Balance){
            let caller = self.env().caller();
            let now = self.env().block_timestamp();

            assert!(now < self.contribution_end, "cannot contribute after contribution end");
            assert!(self.available_funds >= amount, "not enough available funds");

            let share = self.shares.get(caller).unwrap_or_default();

            self.shares.insert(caller, &(share - amount));
            self.available_funds -= amount;
            
            self.env().transfer(caller, amount).unwrap_or_default();

        }

        #[ink(message)]
        pub fn transfer_share(&mut self, amount: Balance, to: AccountId) {
            let caller = self.env().caller();
            let share = self.shares.get(caller).unwrap_or_default();
            assert!(share >= amount, "not enough share");

            self.shares.insert(caller, &(share - amount));
            self.shares.insert(to, &(share + amount));
            self.investors.insert(to, &true);
        }

        #[ink(message)]
        pub fn create_proposal(&mut self, name: Vec<u8>, amount: Balance, recipient: AccountId) {
            let caller = self.env().caller();

            let investor = self.investors.get(caller).unwrap_or_default();
            assert!(investor == true, "only investor can create proposal");

            assert!(self.available_funds >= amount, "amount too big");

            let proposal_id = self.next_id();
            let now = self.env().block_timestamp();

            let proposal = Proposal{
                name,
                amount,
                recipient,
                votes: 0,
                end: now + self.vote_time,
                execuated: false
            };

            self.proposals.insert(proposal_id, &proposal);

            self.available_funds -= amount;

        }

        #[ink(message)]
        pub fn vote(&mut self, proposal_id: i32) {
            let mut proposal = self.proposals.get(proposal_id).unwrap_or_default();

            let caller = self.env().caller();
            let share = self.shares.get(caller).unwrap_or_default();

            let now = self.env().block_timestamp();
            let mut vote = self.votes.get(caller).unwrap_or_default();

            let proposal_id = self.next_id();
            assert!(vote == (proposal_id, false), "only investor can vote once for the proposal");
            assert!(now <= proposal.end, "can only vote until proposal end date");

            vote = (proposal_id, true);

            proposal.votes += share;
        }

        #[ink(message)]
        pub fn execute_proposal(&mut self, proposal_id: i32) {
            let caller = self.env().caller();
            assert!(caller == self.admin, "only admin can execute proposal");

            let now = self.env().block_timestamp();
            let proposal = self.proposals.get(proposal_id).unwrap_or_default();
            assert!(now >= proposal.end, "cannot execute proposal before end date");
            assert!(proposal.execuated == false, "cannot execute proposal already execuated");
            assert!((proposal.votes / self.total_shares) * 100 >= self.quorum, "cannot execute proposal with votes below quorum");

            assert!(proposal.amount <= self.available_funds, "not enough available funds");
            self.available_funds -= proposal.amount;
            self.env().transfer(proposal.recipient, proposal.amount).unwrap_or_default();

        }

        pub fn next_id(&mut self) -> ProposalId {
            let id = self.next_proposal_id;
            self.next_proposal_id += 1;
            id
        }

    }
}
