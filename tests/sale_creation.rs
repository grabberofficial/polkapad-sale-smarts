use codec::Encode;
use gtest::{System};

use sale_io::*;

mod shared;
use shared::*;

#[test]
fn create_sale_as_admin_should_created() {
    let system = System::new();
    init(&system);
   
    let sale = system.get_program(SALE_ADDRESS);

    let parameters = SaleParameters {
        token: SALE_TOKEN_ADDRESS.into(),
        owner: SALE_OWNER.into(),
        staking: STAKING_ADDRESS.into(),
        tokens_to_sell: 100,
        registration_fee_gear: 1,
        token_price_in_gear: TOKEN_PRICE_IN_GEAR
    };

    let result = sale.send(SALE_ADMIN, SaleAction::CreateSale(parameters));
    assert!(result.contains(&(SALE_ADMIN, SaleEvent::SaleCreated(parameters).encode())));
}

#[test]
fn create_sale_as_not_admin_should_failed() {
    let system = System::new();
    init(&system);
   
    let sale = system.get_program(SALE_ADDRESS);

    let parameters = SaleParameters {
        ..Default::default()
    };

    let result = sale.send(ALICE, SaleAction::CreateSale(parameters));
    assert!(result.main_failed());
}

#[test]
fn create_sale_as_admin_when_sale_already_created_should_failed() {
    let system = System::new();
    init(&system);
   
    let sale = system.get_program(SALE_ADDRESS);

    let parameters = SaleParameters {
        token: SALE_TOKEN_ADDRESS.into(),
        owner: SALE_OWNER.into(),
        staking: STAKING_ADDRESS.into(),
        tokens_to_sell: 100,
        registration_fee_gear: 1,
        token_price_in_gear: TOKEN_PRICE_IN_GEAR,  
    };

    sale.send(SALE_ADMIN, SaleAction::CreateSale(parameters));
    
    let result = sale.send(SALE_ADMIN, SaleAction::CreateSale(parameters));
    assert!(result.main_failed());
}

#[test]
fn create_sale_as_admin_when_sale_start_date_in_past_should_failed() {
    let system = System::new();
    init(&system);
   
    let sale = system.get_program(SALE_ADDRESS);

    let parameters = SaleParameters {
        token: SALE_TOKEN_ADDRESS.into(),
        owner: SALE_OWNER.into(),
        staking: STAKING_ADDRESS.into(),
        tokens_to_sell: 100,
        registration_fee_gear: 1,
        token_price_in_gear: TOKEN_PRICE_IN_GEAR,   
    };

    sale.send(SALE_ADMIN, SaleAction::CreateSale(parameters));
    let result = sale.send(SALE_ADMIN, SaleAction::SetSaleTime(system.block_timestamp() - 2, system.block_timestamp() - 1));
    
    assert!(result.main_failed());
}

#[test]
fn create_sale_as_admin_when_tokens_to_sell_is_0_should_failed() {
    let system = System::new();
    init(&system);
   
    let sale = system.get_program(SALE_ADDRESS);

    let parameters = SaleParameters {
        token: SALE_TOKEN_ADDRESS.into(),
        owner: SALE_OWNER.into(),
        staking: STAKING_ADDRESS.into(),
        tokens_to_sell: 0,
        registration_fee_gear: 1,
        token_price_in_gear: TOKEN_PRICE_IN_GEAR,   
    };

    let result = sale.send(SALE_ADMIN, SaleAction::CreateSale(parameters));
    assert!(result.main_failed());
}

