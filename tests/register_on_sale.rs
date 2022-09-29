use codec::Encode;
use gtest::{Log, System};

use sale_io::*;

mod shared;
use shared::*;

#[test]
fn register_on_sale_should_registered() {
    let system = System::new();
    init(&system);
    
    let registration_fee_gear = 1000;
    prepare_user_for_registration(&system, ALICE, registration_fee_gear, 100, 50);


    let sale = system.get_program(SALE_ADDRESS);
    sale.send(SALE_ADMIN, SaleAction::CreateSale(SaleParameters {
        token: SALE_TOKEN_ADDRESS.into(),
        owner: SALE_OWNER.into(),
        staking: STAKING_ADDRESS.into(),
        tokens_to_sell: 100,
        token_price_in_gear: 5,   
        registration_fee_gear: 1000,
    }));

    let register_start_date = system.block_timestamp();
    let register_end_date = system.block_timestamp() + 20000;

    sale.send(SALE_ADMIN, SaleAction::SetSaleTime(register_end_date + 20000, register_end_date + 40000));
    sale.send(SALE_ADMIN, SaleAction::SetRegistrationTime(register_start_date, register_end_date));

    let result = sale.send_with_value(ALICE, SaleAction::RegisterOnSale, registration_fee_gear);

    assert!(result.contains(&(ALICE, SaleEvent::UserRegistered(ALICE.into()).encode())));
    assert_eq!(system.balance_of(SALE_ADDRESS), registration_fee_gear);
}

#[test]
fn register_on_sale_when_no_stakes_should_failed() {
    let system = System::new();
    init(&system);

    let registration_fee_gear = 1000;
    prepare_user_for_registration(&system, ALICE, registration_fee_gear, 100, 0);

    let sale = system.get_program(SALE_ADDRESS);
    sale.send(SALE_ADMIN, SaleAction::CreateSale(SaleParameters {
        token: SALE_TOKEN_ADDRESS.into(),
        owner: SALE_OWNER.into(),
        staking: STAKING_ADDRESS.into(),
        tokens_to_sell: 100,
        token_price_in_gear: 5,   
        registration_fee_gear: 1000,
    }));

    let register_start_date = system.block_timestamp();
    let register_end_date = system.block_timestamp() + 20000;

    sale.send(SALE_ADMIN, SaleAction::SetSaleTime(register_end_date + 20000, register_end_date + 40000));
    sale.send(SALE_ADMIN, SaleAction::SetRegistrationTime(register_start_date, register_end_date));

    let result = sale.send_with_value(ALICE, SaleAction::RegisterOnSale, registration_fee_gear);
    assert!(result.main_failed());
}

#[test]
fn register_on_sale_when_not_enough_fee_deposited_should_failed() {
    let system = System::new();
    init(&system);
    
    let registration_fee_gear = 500;
    prepare_user_for_registration(&system, ALICE, registration_fee_gear, 100, 50);

    let sale = system.get_program(SALE_ADDRESS);
    sale.send(SALE_ADMIN, SaleAction::CreateSale(SaleParameters {
        token: SALE_TOKEN_ADDRESS.into(),
        owner: SALE_OWNER.into(),
        staking: STAKING_ADDRESS.into(),
        tokens_to_sell: 100,
        token_price_in_gear: 5,   
        registration_fee_gear: 1000,
    }));

    let register_start_date = system.block_timestamp();
    let register_end_date = system.block_timestamp() + 20000;

    sale.send(SALE_ADMIN, SaleAction::SetSaleTime(register_end_date + 20000, register_end_date + 40000));
    sale.send(SALE_ADMIN, SaleAction::SetRegistrationTime(register_start_date, register_end_date));

    let result = sale.send_with_value(ALICE, SaleAction::RegisterOnSale, registration_fee_gear);
    assert!(result.main_failed());
}

#[test]
fn register_on_sale_when_user_already_registered_should_failed() {
    let system = System::new();
    init(&system);

    let registration_fee_gear = 500;
    prepare_user_for_registration(&system, ALICE, 1000, 100, 50);

    let sale = system.get_program(SALE_ADDRESS);
    sale.send(SALE_ADMIN, SaleAction::CreateSale(SaleParameters {
        token: SALE_TOKEN_ADDRESS.into(),
        owner: SALE_OWNER.into(),
        staking: STAKING_ADDRESS.into(),
        tokens_to_sell: 100,
        token_price_in_gear: 5,   
        registration_fee_gear: 500,
    }));

    let register_start_date = system.block_timestamp();
    let register_end_date = system.block_timestamp() + 20000;

    sale.send(SALE_ADMIN, SaleAction::SetSaleTime(register_end_date + 20000, register_end_date + 40000));
    sale.send(SALE_ADMIN, SaleAction::SetRegistrationTime(register_start_date, register_end_date));

    sale.send_with_value(ALICE, SaleAction::RegisterOnSale, registration_fee_gear);
    let result = sale.send_with_value(ALICE, SaleAction::RegisterOnSale, registration_fee_gear);
    assert!(result.main_failed());
}