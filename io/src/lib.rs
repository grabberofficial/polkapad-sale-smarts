#![no_std]

use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo, Default, Clone, Copy)]
pub struct Participate {
    pub amount_bought: u128,
    pub amount_paid_gear: u128,
    pub participated_datetime: u64 
}

#[derive(Debug, Decode, Encode, TypeInfo, Default, Clone, Copy)]
pub struct SaleParameters {
    pub token: ActorId,
    pub owner: ActorId,
    pub tokens_to_sell: u128,
    pub token_price_in_gear: u128,
    pub registration_fee_gear: u128,
    pub start_datetime: u64,
    pub end_datetime: u64
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct SaleInitialConfiguration {
    pub sale_admin: ActorId,
    pub staking_contract: ActorId
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum SaleAction {
    CreateSale(SaleParameters),

    SetSaleToken(ActorId),
    SetRegistrationTime(u64, u64),
    SetSaleTime(u64, u64),
    SetMaxAllocationSizes(BTreeMap<ActorId, u128>),

    GetAllocationSizeOf(ActorId),
    GetParticipationOf(ActorId),
    GetSaleToken,
    GetTotalSold,
    GetTotalRaised,

    RegisterOnSale,
    Participate,

    DepositTokens,

    WithdrawAllocation,
    WithdrawLeftover,
    WithdrawEarnings,
    WithdrawRegistrationFees,

    RemoveRegistered(ActorId),
    CloseGate
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum SaleEvent {
    SaleCreated(SaleParameters),
    UserRegistered(ActorId),
    RegistrationGEARRefunded(ActorId, u128),

    AllocationWithdrawn(ActorId, u128),
    RegistrationFeeWithdrawn(u128),
    EarningsWithdrawn(u128),

    RegistrationTimeSet(u64),
    SaleTimeSet(u64),
    SaleTokenSet(ActorId),
    MaxAllocationSizeSet((ActorId, u128)),

    RegisteredRemoved(ActorId),
    GateClosed(u64),

    SaleToken(ActorId),
    AllocationSize(u128),
    Participation(Participate),
    TotalSold(u128),
    TotalRaised(u128),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum SaleState {
    GetAllocationSizeOf(ActorId),
    GetParticipationOf(ActorId),
    GetSaleRoundTime,
    GetRegistrationRoundTime,
    GetSaleOwner,
    GetSaleToken,
    GetTotalSold,
    GetTotalRaised,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum SaleReply {
    SaleToken(ActorId),
    SaleOwner(ActorId),
    SaleRoundTime(u64, u64),
    RegistrationRoundTime(u64, u64),
    AllocationSize(u128),
    Participation(Participate),
    TotalSold(u128),
    TotalRaised(u128),
}