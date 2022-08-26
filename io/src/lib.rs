#![no_std]

use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct SaleParameters {
    pub token: ActorId,
    pub owner: ActorId,
    pub tokens_to_sell: u128,
    pub registration_fee_gear: u128,
    pub start_datetime: u64,
    pub end_datetime: u64
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct SaleInitialConfiguration {
    pub admin: ActorId,
    pub staking_contract: ActorId
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum SaleAction {
    CreateSale(SaleParameters),

    SetSaleToken(ActorId),
    SetRegistrationTime((u64, u64)),
    SetMaxAllocationSizes(BTreeMap<ActorId, u128>),

    RegisterOnSale,
    Participate(u128),

    DepositTokens,

    WithdrawAllocation,
    WithdrawLeftover,
    WithdrawEarnings,
    WithdrawRegistrationFees,

    CloseGate
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum SaleEvent {
    SaleCreated(u64),
    SaleTokenSet(ActorId),
    UserRegistered(ActorId),
    RegistrationGEARRefunded((ActorId, u128)),
    TokenSold((ActorId, u128)),

    AllocationWithdrawed((ActorId, u128)),
    RegistrationFeeWithdrawed(u128),

    RegistrationTimeSet(u64),
    MaxAllocationSizeSet((ActorId, u128)),
    GateClosed(u64)
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum SaleState {
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum SaleReply {
}