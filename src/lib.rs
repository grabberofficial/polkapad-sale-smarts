#![no_std]

mod sale;
mod utils;

use gstd::{msg, prelude::*};
use sale::Sale;
use sale_io::*;

static mut SALE: Option<Sale> = None;

gstd::metadata! {
    title: "PolkapadSale",
    handle:
        input: SaleAction,
        output: SaleEvent,
    state:
        input: SaleState,
        output: SaleReply,
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let sale = Sale {
        admin: msg::source(),
        ..Sale::default()
    };

    SALE = Some(sale);
}

#[gstd::async_main]
async unsafe fn main() {
    let sale = unsafe { SALE.get_or_insert(Sale::default()) };

    let action: SaleAction = msg::load()
        .expect("Polkapad Sale: unable to decode configuration");

    match action {
        SaleAction::CreateSale(parameters) => {
            sale.set_sale_parameters(parameters);
        },
        SaleAction::SetSaleToken(token_address) => {
            sale.set_sale_token(token_address);
        },
        SaleAction::SetRegistrationTime(start_datetime, end_datetime) => {
            sale.set_registration_time(start_datetime, end_datetime);
        },
        SaleAction::SetSaleTime(start_datetime, end_datetime) => {
            sale.set_sale_time(start_datetime, end_datetime);
        },
        SaleAction::SetMaxAllocationSizes(users) => {
            sale.set_allocation_sizes(users);
        },
        SaleAction::DepositTokens => {
            sale.deposit_tokens().await;
        },
        SaleAction::RemoveRegistered(who) => {
            sale.remove_registered(who);
        },
        SaleAction::CloseGate => {
            sale.close_gate();
        },
        SaleAction::RegisterOnSale => {
            sale.register().await;
        },
        SaleAction::Participate => {
            sale.participate().await;
        },
        SaleAction::WithdrawAllocation => {
            sale.withdraw_allocation().await;
        },
        SaleAction::WithdrawEarnings => {
            sale.withdraw_earnings();
        },
        SaleAction::WithdrawLeftover => {
            sale.withdraw_leftover().await;
        },
        SaleAction::WithdrawRegistrationFees => {
            sale.withdraw_registration_fees();
        },
        SaleAction::GetSaleToken => {
            sale.get_sale_token();
        },
        SaleAction::GetTotalRaised => {
            sale.get_total_raised();
        },
        SaleAction::GetTotalSold => {
            sale.get_total_sold();
        },
        SaleAction::GetAllocationSizeOf(participiant) => {
            sale.get_allocation_size_of(participiant);
        },
        SaleAction::GetParticipationOf(participiant) => {
            sale.get_participation_of(participiant);
        },
    }

}

#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: SaleState = msg::load().expect("Polkapad Sale: unable to decode state");
    let sale: &mut Sale = SALE.get_or_insert(Sale::default());

    let encoded = match query {
        SaleState::GetAllocationSizeOf(who) => 
            SaleReply::AllocationSize(*sale.registration.users.get(&who).unwrap_or(&0)),
        SaleState::GetParticipationOf(who) => 
            SaleReply::Participation(*sale.sale.participants.get(&who).unwrap_or(&Participate { ..Default::default() })),
        SaleState::GetSaleRoundTime => 
            SaleReply::SaleRoundTime(sale.registration.start_datetime, sale.registration.end_datetime),
        SaleState::GetRegistrationRoundTime => 
            SaleReply::RegistrationRoundTime(sale.sale.start_datetime, sale.sale.end_datetime),
        SaleState::GetSaleToken => 
            SaleReply::SaleToken(sale.token),
        SaleState::GetSaleOwner => 
            SaleReply::SaleOwner(sale.owner),
        SaleState::GetTotalSold => 
            SaleReply::TotalSold(sale.tokens_sold),
        SaleState::GetTotalRaised => 
            SaleReply::TotalSold(sale.tokens_raised),
    }
    .encode();
    gstd::util::to_leak_ptr(encoded)
}