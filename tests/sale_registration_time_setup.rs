use codec::Encode;
use gtest::{System};

use sale_io::*;

mod shared;
use shared::*;

#[test]
fn set_registration_time_as_admin_should_created() {
    let system = System::new();
    init(&system);
   
    let sale = system.get_program(SALE_ADDRESS);

    sale.send(SALE_ADMIN, SaleAction::CreateSale(SaleParameters {
        token: SALE_TOKEN_ADDRESS.into(),
        owner: SALE_OWNER.into(),
        staking: STAKING_ADDRESS.into(),
        tokens_to_sell: 100,
        registration_fee_gear: 1,
        token_price_in_gear: TOKEN_PRICE_IN_GEAR,   
    }));

    let register_start_date = 1753535522084;
    let register_end_date = 1753621922084;

    sale.send(SALE_ADMIN, SaleAction::SetSaleTime(register_end_date + 10, register_end_date + 20));
    let result = sale.send(SALE_ADMIN, SaleAction::SetRegistrationTime(register_start_date, register_end_date));
    assert!(result.contains(&(SALE_ADMIN, SaleEvent::RegistrationTimeSet(system.block_timestamp()).encode())));
}

#[test]
fn set_registration_time_as_not_admin_should_failed() {
    let system = System::new();
    init(&system);
   
    let sale = system.get_program(SALE_ADDRESS);

    sale.send(SALE_ADMIN, SaleAction::CreateSale(SaleParameters {
        owner: SALE_OWNER.into(),
        tokens_to_sell: 100,
        registration_fee_gear: 1,
        ..Default::default()
    }));

    let register_start_date = 1753535522084; // Sat Jul 26 2025 16:12:02
    let register_end_date = 1753621922084;   // Sun Jul 27 2025 16:12:02 

    let result = sale.send(ALICE, SaleAction::SetRegistrationTime(register_start_date, register_end_date));
    assert!(result.main_failed())
}

#[test]
fn set_registration_time_as_admin_when_sale_is_not_created_should_failed() {
    let system = System::new();
    init(&system);
   
    let sale = system.get_program(SALE_ADDRESS);

    let register_start_date = 1753535522084; // Sat Jul 26 2025 16:12:02
    let register_end_date = 1753621922084;   // Sun Jul 27 2025 16:12:02 

    let result = sale.send(SALE_ADMIN, SaleAction::SetRegistrationTime(register_start_date, register_end_date));
    assert!(result.main_failed());
}

