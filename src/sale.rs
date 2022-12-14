use gstd::{prelude::*, exec, msg, ActorId};

use sale_io::{SaleEvent, SaleParameters, Participate};
use ft_io::{FTAction, FTEvent};
use staking_io::{StakingEvent, StakingAction};

use crate::require;

const ZERO_ID: ActorId = ActorId::new([0u8; 32]);
const ZERO_MAX_ALLOCATION_SIZE: u128 = 0;

#[derive(Debug, Default)]
pub struct RegistrationRound {
    pub start_datetime: u64,
    pub end_datetime: u64,
    pub users: BTreeMap<ActorId, u128>
}

#[derive(Debug, Default)]
pub struct SaleRound {
    pub start_datetime: u64,
    pub end_datetime: u64,
    pub participants: BTreeMap<ActorId, Participate>
}

#[derive(Debug, Default)]
pub struct Sale {
    pub admin: ActorId,
    pub owner: ActorId,
    pub token: ActorId,
    pub staking: ActorId,
    pub registration: RegistrationRound,
    pub sale: SaleRound,

    pub tokens_to_sell: u128,
    pub tokens_sold: u128,
    pub tokens_raised: u128,
    pub token_price_in_gear: u128,
    pub tokens_deposited: bool,

    pub registration_fee_gear: u128,
    pub registration_fees: u128,

    pub earnings_withdrawn: bool,
    pub leftover_withdrawn: bool,
    pub is_created: bool,

    pub gate_closed: bool
}

impl Sale {
    pub async fn register(&mut self) {
        let reply: StakingEvent = msg::send_for_reply_as::<_, StakingEvent>(
            self.staking,
            StakingAction::StakeOf(msg::source()),
            0,
        )
        .unwrap()
        .await
        .expect("Function call error");

        let balance = if let StakingEvent::Staked(balance) = reply { balance } else { panic!("Error while parsing event") };

        require!(balance > 0, "You need to stake PLPD to participate this sale");

        require!(self.registration_fee_gear == msg::value(), "Registration deposit doesn't match");
        require!(exec::block_timestamp() >= self.registration.start_datetime &&
                 exec::block_timestamp() <= self.registration.end_datetime,
            "Registration round is over"
        );
        require!(
            self.registration.users.get(&msg::source()).is_none(),
            "User already registered"
        );

        self.registration.users.insert(msg::source(), ZERO_MAX_ALLOCATION_SIZE);
        self.registration_fees = self.registration_fees.saturating_add(msg::value());

        msg::send_for_reply(
            msg::source(), 
            SaleEvent::UserRegistered(msg::source()), 0)
            .unwrap()
            .await
            .expect("Sending message error");
    }

    pub async fn participate(&mut self) {
        require!(exec::block_timestamp() >= self.sale.start_datetime &&
                 exec::block_timestamp() <= self.sale.end_datetime,
            "Sale round is over"
        );

        require!(
            self.registration.users.get(&msg::source()).is_some(),
            "User must be registered"
        );
        require!(
            self.sale.participants.get(&msg::source()).is_none(),
            "User already participated"
        );

        let tokens_to_buy_in_gear = msg::value();

        let reply: FTEvent = msg::send_for_reply_as::<_, FTEvent>(
            self.token,
            FTAction::Decimals,
            0,
        )
        .unwrap()
        .await
        .expect("Function call error");

        let decimals = if let FTEvent::Decimals(decimals) = reply { decimals } else { panic!("Error while parsing event") } as u32;

        let tokens_to_buy = tokens_to_buy_in_gear
            .saturating_mul(10_u128.pow(decimals))
            .saturating_div(self.token_price_in_gear);

        require!(
            tokens_to_buy <= *self.registration.users.get(&msg::source()).unwrap(),
            "You cannot realize allocation greater than your max allocation size"
        );

        require!(tokens_to_buy > 0, "It is impossible to buy zero amount of tokens");
        require!(
            tokens_to_buy <= self.tokens_to_sell.saturating_sub(self.tokens_sold),
            "Not enough tokens to sell"
        );

        self.tokens_sold = self.tokens_sold.saturating_add(tokens_to_buy);
        self.tokens_raised = self.tokens_raised.saturating_add(tokens_to_buy_in_gear);
        self.registration_fees = self.registration_fees.saturating_sub(self.registration_fee_gear);

        self.sale.participants.insert(msg::source(), Participate { 
            amount_bought: tokens_to_buy,
            amount_paid_gear: tokens_to_buy_in_gear,
            participated_datetime: exec::block_timestamp()
        });

        msg::send_for_reply(
            msg::source(),
            SaleEvent::RegistrationGEARRefunded(msg::source(), self.registration_fee_gear), 
            self.registration_fee_gear)
            .unwrap()
            .await
            .expect("Sending message error");
    }

    pub fn remove_registered(&mut self, who: ActorId) {
        require!(exec::block_timestamp() >= self.registration.start_datetime &&
                 exec::block_timestamp() <= self.registration.end_datetime,
            "Registration round is over"
        );
        require!(
            self.registration.users.get(&msg::source()).is_some(),
            "User must be registered"
        );

        self.registration.users.remove(&who);   
        
        msg::reply(SaleEvent::RegisteredRemoved(who), 0).unwrap();
    }

    pub async fn deposit_tokens(&mut self) {
        self.only_sale_owner();
        self.only_if_gate_open();
        
        require!(self.is_created, "Sale must be created");
        require!(!self.tokens_deposited, "Tokens already deposited");
        
        transfer_tokens(
            &self.token, 
            &msg::source(), 
            &exec::program_id(), 
            self.tokens_to_sell)
            .await;

        self.tokens_deposited = true;
    }

    pub async fn withdraw_allocation(&mut self) {
        require!(exec::block_timestamp() >= self.sale.end_datetime, "Sale is not over yet");

        let participant = self.sale.participants.get(&msg::source());
        require!(participant.is_some(), "User has to participates sale to be able withdraw funds");

        let participation = participant.unwrap();
        require!(participation.amount_bought > 0, "There are no funds to withdraw");
        
        transfer_tokens(
            &self.token, 
            &exec::program_id(), 
            &msg::source(), 
            participation.amount_bought)
            .await;

        msg::send_for_reply(
            msg::source(), 
            SaleEvent::AllocationWithdrawn(msg::source(), participation.amount_bought), 
            0)
            .unwrap()
            .await
            .expect("Sending message error");
    }

    pub fn withdraw_earnings(&mut self) {
        self.only_sale_owner();

        require!(exec::block_timestamp() >= self.sale.end_datetime, "Sale is not over yet");
        require!(!self.earnings_withdrawn, "Impossible to withdraw earnings twice");

        require!(self.tokens_raised > 0, "There are no tokens to withdraw");

        msg::reply(SaleEvent::EarningsWithdrawn(self.tokens_raised), self.tokens_raised).unwrap();

        self.earnings_withdrawn = true;
    }

    pub async fn withdraw_leftover(&mut self) {
        self.only_sale_owner();

        require!(exec::block_timestamp() >= self.sale.end_datetime, "Sale is not over yet");
        require!(!self.leftover_withdrawn, "Impossible to withdraw leftover twice");

        let leftover = self.tokens_to_sell.saturating_sub(self.tokens_sold);
        require!(leftover > 0, "There are no tokens to withdraw");

        transfer_tokens(
            &self.token, 
            &exec::program_id(), 
            &msg::source(), 
            leftover)
            .await;

        self.leftover_withdrawn = true;
    }

    pub fn withdraw_registration_fees(&mut self) {
        self.only_admin();

        require!(exec::block_timestamp() >= self.sale.end_datetime, "Sale is not over yet");
        require!(self.registration_fees > 0, "There are no tokens to withdraw");

        msg::reply(SaleEvent::RegistrationFeeWithdrawn(self.registration_fees), self.registration_fees).unwrap();

        self.registration_fees = 0;
    }

    pub fn close_gate(&mut self) {
        self.only_admin();
        self.only_if_gate_open();

        require!(self.is_created, "Sale is not created");
        require!(self.token != ZERO_ID, "Token is not set");
        require!(self.tokens_deposited, "Tokens are not deposited");
        require!(
            self.registration.start_datetime != 0 && self.registration.end_datetime != 0,
            "Registration params are not set"
        );

        self.gate_closed = true;

        msg::reply(SaleEvent::GateClosed(exec::block_timestamp()), 0).unwrap();
    }

    pub fn set_allocation_sizes(&mut self, allocations_sizes: BTreeMap<ActorId, u128>) {
        self.only_admin();   

        for (user, allocation_size) in allocations_sizes.iter() {
            let registered_user = self.registration.users.get(user);
            match registered_user {
                Some(_) => {
                    self.registration.users
                        .entry(*user)
                        .and_modify(|allocation| *allocation += allocation_size)
                        .or_insert(*allocation_size);

                        msg::reply(SaleEvent::MaxAllocationSizeSet((*user, *allocation_size)), 0).unwrap();
                },
                None => panic!("User is not registered"),
            }
        }
    }

    pub fn set_sale_parameters(&mut self, parameters: SaleParameters) {
        self.only_admin();

        require!(!self.is_created, "Sale must not be created");
        require!(parameters.owner != ZERO_ID, "Invalid sale owner address");
        require!(parameters.tokens_to_sell > 0, "Amout of tokens must be greater than zero");

        self.owner = parameters.owner;
        self.token = parameters.token;
        self.staking = parameters.staking;
        self.token_price_in_gear = parameters.token_price_in_gear;
        self.tokens_to_sell = parameters.tokens_to_sell;
        self.registration_fee_gear = parameters.registration_fee_gear;

        self.is_created = true;

        msg::reply(SaleEvent::SaleCreated(parameters.clone()), 0).unwrap();
    }

    pub fn set_registration_time(&mut self, start_datetime: u64, end_datetime: u64) {
        self.only_admin();
        self.only_if_gate_open();

        require!(self.is_created, "Sale must be created");
        require!(self.sale.end_datetime > end_datetime, "Registration end date must be earlier than sale's end date");
        require!(start_datetime >= exec::block_timestamp() && start_datetime < end_datetime, "Registration's start date must be in future");

        self.registration = RegistrationRound {
            start_datetime,
            end_datetime,
            ..Default::default()
        };

        msg::reply(SaleEvent::RegistrationTimeSet(exec::block_timestamp()), 0).unwrap();
    }

    pub fn set_sale_time(&mut self, start_datetime: u64, end_datetime: u64) {
        self.only_admin();
        self.only_if_gate_open();

        require!(self.is_created, "Sale must be created");
        require!(start_datetime >= exec::block_timestamp() && start_datetime < end_datetime, "Sale's start date must be in future");

        self.sale.start_datetime = start_datetime;
        self.sale.end_datetime = end_datetime;

        msg::reply(SaleEvent::SaleTimeSet(exec::block_timestamp()), 0).unwrap();
    }

    pub fn set_sale_token(&mut self, sale_token: ActorId) {
        self.only_admin();
        self.only_if_gate_open();

        self.token = sale_token;

        msg::reply(SaleEvent::SaleTokenSet(sale_token), 0).unwrap();
    }

    pub fn get_allocation_size_of(&self, participiant: ActorId) {
        let allocation_size = self.registration.users
            .get(&participiant)
            .expect("Polkapad Sale: user is not registered to the sale");
        
        msg::reply(SaleEvent::AllocationSize(*allocation_size), 0).unwrap();
    }

    pub fn get_participation_of(&self, participiant: ActorId) {
        let participation = self.sale.participants
            .get(&participiant)
            .expect("Polkapad Sale:  user is not a participiant of this sale");
        
        msg::reply(SaleEvent::Participation(*participation), 0).unwrap();
    }

    pub fn get_sale_token(&self) {
        msg::reply(SaleEvent::SaleToken(self.token), 0).unwrap();
    }

    pub fn get_total_raised(&self) {
        msg::reply(SaleEvent::TotalRaised(self.tokens_raised), 0).unwrap();
    }

    pub fn get_total_sold(&self) {
        msg::reply(SaleEvent::TotalSold(self.tokens_sold), 0).unwrap();
    }

    fn only_admin(&self) {
        require!(self.admin == msg::source(), "Allows only admin address");
    }

    fn only_sale_owner(&self) {
        require!(self.owner == msg::source(), "Allows only sale owner address");
    }

    fn only_if_gate_open(&self) {
        require!(!self.gate_closed, "Gate must not be closed");
    }
}

async fn transfer_tokens(
    token_address: &ActorId,
    from: &ActorId,
    to: &ActorId,
    amount: u128,
) {
    msg::send_for_reply(
        *token_address,
        FTAction::TransferFrom {
            from: *from,
            to: *to,
            amount,
        },
        0,
    )
    .expect("Polkapad Sale: error in sending message")
    .await
    .expect("Polkapd Sale: error in transfer");
}