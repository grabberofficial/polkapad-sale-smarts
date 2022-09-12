use gstd::{String, BTreeMap};
use gtest::{Program, System};

use ft_io::*;
use staking_io::*;
use sale_io::*;

pub const PLPD_TOKEN_ADDRESS: u64 = 1;
pub const SALE_TOKEN_ADDRESS: u64 = 2;
pub const STAKING_ADDRESS: u64 = 3;
pub const SALE_ADDRESS: u64 = 4;

pub const DEPLOYER: u64 = 10;
pub const SALE_OWNER: u64 = 11;
pub const SALE_ADMIN: u64 = 12;
pub const ALICE: u64 = 13;
pub const BOB: u64 = 14;

pub const TOKENS_TO_SELL: u128 = 100_000_000 * 10e18 as u128;
pub const TOKEN_PRICE_IN_GEAR: u128 =  10 * 10e18 as u128;
pub const DECIMALS: u32 =  18;

pub fn configure_sale(system: &System, sale: &Program, registration_fee_gear: u128) {
    sale.send(SALE_ADMIN, SaleAction::CreateSale(SaleParameters {
        token: SALE_TOKEN_ADDRESS.into(),
        owner: SALE_OWNER.into(),
        tokens_to_sell: TOKENS_TO_SELL,
        token_price_in_gear: TOKEN_PRICE_IN_GEAR,
        registration_fee_gear: registration_fee_gear,
        start_datetime: system.block_timestamp() - 15000,
        end_datetime: system.block_timestamp() + 40000,   
    }));

    let register_start_date = system.block_timestamp();
    let register_end_date = system.block_timestamp() + 20000;

    sale.send(SALE_OWNER, SaleAction::DepositTokens);
    sale.send(SALE_ADMIN, SaleAction::SetRegistrationTime(register_start_date, register_end_date));
    sale.send_with_value(ALICE, SaleAction::RegisterOnSale, registration_fee_gear);
}

pub fn prepare_user_for_registration(system: &System, user: u64, gear_amount: u128, plpd_amount: u128, to_stake: u128) {
    system.mint_to(user, gear_amount);
    
    transfer_tokens(system, PLPD_TOKEN_ADDRESS, DEPLOYER, user, plpd_amount);

    let staking = system.get_program(STAKING_ADDRESS);
    staking.send(user, StakingAction::Stake(to_stake));
}

pub fn set_max_allocation_size_to_user(system: &System, user: u64, allocation_size: u128) {
    let sale = system.get_program(SALE_ADDRESS);

    let mut allocations = BTreeMap::new();
    allocations.insert(user.into(), allocation_size);

    sale.send(SALE_ADMIN, SaleAction::SetMaxAllocationSizes(allocations));
}

pub fn transfer_tokens(system: &System, token: u64, from: u64, to: u64, amount: u128) {
    let token = system.get_program(token);

    token.send(DEPLOYER, FTAction::Transfer { 
        from: from.into(), 
        to: to.into(), 
        amount 
    });
}

pub fn init(system: &System) {
    system.init_logger();

    init_token(&system, "Test Polkapad", "TPLPD");
    init_token(&system, "Future", "FTR");
    init_staking(&system);
    init_sale(&system)
}

fn init_sale(system: &System) {
    
    let sale = Program::current(system);
    let result = sale.send(
        DEPLOYER,
        SaleInitialConfiguration { 
            sale_admin: SALE_ADMIN.into(), 
            staking_contract: STAKING_ADDRESS.into() 
        },
    );

    assert!(result.log().is_empty());
}

fn init_staking(system: &System) {
    let staking = Program::from_file(system, "../staking/target/wasm32-unknown-unknown/release/polkapad_staking.wasm");

    let result = staking.send(
        DEPLOYER,
        StakingInitialConfiguration {
            token_address: PLPD_TOKEN_ADDRESS.into()
        },
    );

    assert!(result.log().is_empty());
}

fn init_token(system: &System, name: &str, symbol: &str) {
    let token = Program::from_file(system, "../fungible-token/target/wasm32-unknown-unknown/release/fungible_token.wasm");

    let result = token.send(
        DEPLOYER,
        FTInitialConfiguration {
            name: String::from(name),
            symbol: String::from(symbol),
            decimals: DECIMALS as u8
        },
    );

    assert!(!result.main_failed());

    token.send(DEPLOYER, FTAction::Transfer { 
        from: DEPLOYER.into(),
        to: SALE_OWNER.into(), 
        amount: TOKENS_TO_SELL 
    });
}