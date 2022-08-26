#![no_std]

#[cfg(test)]
mod tests;
mod sale;
mod utils;

use gstd::{msg, prelude::*};
use sale::Sale;
use sale_io::*;

static mut SALE: Option<Sale> = None;

gstd::metadata! {
    title: "PolkapadSale",
    init:
        input: SaleInitialConfiguration,
    handle:
        input: SaleAction,
        output: SaleEvent,
    state:
        input: SaleState,
        output: SaleReply,
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let config: SaleInitialConfiguration = msg::load()
        .expect("Polkapad Sale: unable to decode configuration");

    let sale = Sale {
        owner: msg::source(),
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
        SaleAction::SetRegistrationTime((start_datetime, end_datetime)) => {
            sale.set_registration_time(start_datetime, end_datetime);
        },
        SaleAction::SetMaxAllocationSizes(users) => {
            sale.set_allocation_sizes(users);
        },
        SaleAction::DepositTokens => {
            sale.deposit_tokens().await;
        },
        SaleAction::CloseGate => {
            sale.close_gate();
        },
        SaleAction::RegisterOnSale => {
            sale.register();
        },
        SaleAction::Participate(tokens_to_buy_in_gear) => {
            sale.participate(tokens_to_buy_in_gear).await;
        },
        SaleAction::WithdrawAllocation => {
            sale.widthdraw_allocation().await;
        },
        SaleAction::WithdrawEarnings => {
            sale.widthdraw_earnings().await;
        },
        SaleAction::WithdrawLeftover => {
            sale.widthdraw_leftover().await;
        },
        SaleAction::WithdrawRegistrationFees => {
            sale.widthdraw_registration_fees();
        }
    }

}

// #[no_mangle]
// pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
// }