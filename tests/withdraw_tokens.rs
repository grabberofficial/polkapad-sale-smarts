use ft_io::{FTAction, FTEvent};
use gstd::Encode;
use gtest::{System, Log};

use sale_io::*;

mod shared;
use shared::*;

#[test]
fn widthdraw_registration_fees_when_nothing_to_withdraw_should_failed() {
    let system = System::new();
    init(&system);
    
    let registration_fee_gear = 1000;
    let plpd_amount = 100;
    let plpd_to_stake = 50;
    let total_user_gear_amount = 100 * 10e18 as u128;
    let to_participate = 5 * 10e18 as u128; 

    prepare_user_for_registration(&system, ALICE, total_user_gear_amount, plpd_amount, plpd_to_stake);

    let sale = system.get_program(SALE_ADDRESS);
    configure_sale(&system, &sale, registration_fee_gear);

    set_max_allocation_size_to_user(&system, ALICE, 5 * 10e16 as u128);

    sale.send_with_value(ALICE, SaleAction::Participate, to_participate);

    system.spend_blocks(5000);

    let result = sale.send(SALE_ADMIN, SaleAction::WithdrawRegistrationFees);
    assert!(result.main_failed());
}

#[test]
fn widthdraw_registration_fees_when_user_not_participated_should_withdrawed() {
    let system = System::new();
    init(&system);
    
    let registration_fee_gear = 1000;
    let plpd_amount = 100;
    let plpd_to_stake = 50;
    let total_user_gear_amount = 100 * 10e18 as u128;

    prepare_user_for_registration(&system, ALICE, total_user_gear_amount, plpd_amount, plpd_to_stake);

    let sale = system.get_program(SALE_ADDRESS);
    configure_sale(&system, &sale, registration_fee_gear);
    set_max_allocation_size_to_user(&system, ALICE, 5 * 10e16 as u128);

    system.spend_blocks(5000);

    let result = sale.send(SALE_ADMIN, SaleAction::WithdrawRegistrationFees);
    let mailbox = system.get_mailbox(SALE_ADMIN);
    let log = Log::builder()
        .dest(SALE_ADMIN)
        .payload(SaleEvent::RegistrationFeeWithdrawn(registration_fee_gear));

    mailbox.claim_value(log.clone());

    assert!(result.contains(&log));
    assert!(system.balance_of(SALE_ADMIN) == registration_fee_gear);
}

#[test]
fn widthdraw_earnings_should_withdrawed() {
    let system = System::new();
    init(&system);
    
    let registration_fee_gear = 1000;
    let plpd_amount = 100;
    let plpd_to_stake = 50;
    let total_user_gear_amount = 100 * 10e18 as u128;
    let to_participate = 5 * 10e18 as u128; 

    prepare_user_for_registration(&system, ALICE, total_user_gear_amount, plpd_amount, plpd_to_stake);

    let sale = system.get_program(SALE_ADDRESS);
    configure_sale(&system, &sale, registration_fee_gear);

    set_max_allocation_size_to_user(&system, ALICE, 5 * 10e16 as u128);

    sale.send_with_value(ALICE, SaleAction::Participate, to_participate);

    system.spend_blocks(5000);

    let result = sale.send(SALE_OWNER, SaleAction::WithdrawEarnings);
    let mailbox = system.get_mailbox(SALE_OWNER);
    let log = Log::builder()
        .dest(SALE_OWNER)
        .payload(SaleEvent::EarningsWithdrawn(to_participate));

    mailbox.claim_value(log.clone());

    assert!(result.contains(&log));
    assert!(system.balance_of(SALE_OWNER) == to_participate);
}

#[test]
fn widthdraw_leftover_should_withdrawed() {
    let system = System::new();
    init(&system);
    
    let registration_fee_gear = 1000;
    let plpd_amount = 100;
    let plpd_to_stake = 50;
    let total_user_gear_amount = 100 * 10e18 as u128;
    let to_participate = 5 * 10e18 as u128; 
    let tokens_bought = 5 * 10e16 as u128; 

    prepare_user_for_registration(&system, ALICE, total_user_gear_amount, plpd_amount, plpd_to_stake);

    let sale = system.get_program(SALE_ADDRESS);
    let token = system.get_program(SALE_TOKEN_ADDRESS);

    configure_sale(&system, &sale, registration_fee_gear);
    set_max_allocation_size_to_user(&system, ALICE, tokens_bought);
    sale.send_with_value(ALICE, SaleAction::Participate, to_participate);
    system.spend_blocks(5000);

    sale.send(SALE_OWNER, SaleAction::WithdrawLeftover);

    let result = token.send(SALE_OWNER, FTAction::BalanceOf(SALE_OWNER.into()));
    assert!(result.contains(&(SALE_OWNER, FTEvent::Balance(TOKENS_TO_SELL - tokens_bought).encode())));
}

#[test]
fn widthdraw_user_allocation_should_withdrawed() {
    let system = System::new();
    init(&system);
    
    let registration_fee_gear = 1000;
    let plpd_amount = 100;
    let plpd_to_stake = 50;
    let total_user_gear_amount = 100 * 10e18 as u128;
    let to_participate = 5 * 10e18 as u128; 
    let tokens_bought = 5 * 10e16 as u128; 

    prepare_user_for_registration(&system, ALICE, total_user_gear_amount, plpd_amount, plpd_to_stake);

    let sale = system.get_program(SALE_ADDRESS);
    let token = system.get_program(SALE_TOKEN_ADDRESS);

    configure_sale(&system, &sale, registration_fee_gear);
    set_max_allocation_size_to_user(&system, ALICE, tokens_bought);
    sale.send_with_value(ALICE, SaleAction::Participate, to_participate);
    system.spend_blocks(5000);

    let result = sale.send(ALICE, SaleAction::WithdrawAllocation);
    assert!(result.contains(&(ALICE, SaleEvent::AllocationWithdrawn(ALICE.into(), tokens_bought).encode())));

    let result = token.send(ALICE, FTAction::BalanceOf(ALICE.into()));
    assert!(result.contains(&(ALICE, FTEvent::Balance(tokens_bought).encode())));
}