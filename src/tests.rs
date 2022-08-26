use codec::Encode;
use gstd::{String};
use gtest::{Program, System};

use ft_io::*;
use staking_io::*;
use sale_io::*;

const PLPD_TOKEN_ADDRESS: u64 = 1;
const SALE_TOKEN_ADDRESS: u64 = 2;
const STAKING_ADDRESS: u64 = 3;
const SALE_ADDRESS: u64 = 4;

const DEPLOYER: u64 = 10;
const SALE_OWNER: u64 = 11;
const SALE_ADMIN: u64 = 12;
const ALICE: u64 = 13;

#[test]
fn test() {
    let system = System::new();
    init_sale(&system)
}

fn init_sale(system: &System) {
    system.init_logger();
    
    let sale = Program::current(system);
    let result = sale.send(
        DEPLOYER,
        SaleInitialConfiguration {},
    );

    assert!(result.log().is_empty());

    let result = sale.send(ALICE, SaleAction::RegisterOnSale);
    assert!(result.contains(&(ALICE, SaleEvent::RegisteredOnSale.encode())));
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

fn init_sale_token(system: &System) {
    let token = Program::from_file(system, "../fungible-token/target/wasm32-unknown-unknown/release/fungible_token.wasm");

    let result = token.send(
        DEPLOYER,
        FTInitialConfiguration {
            name: String::from("SaleToken"),
            symbol: String::from("ST"),
            decimals: 18
        },
    );

    assert!(!result.main_failed());
    
    token.send(
        DEPLOYER,
        FTAction::Transfer {
            from: DEPLOYER.into(),
            to: ALICE.into(),
            amount: 100
        },
    );
    
    let result = token.send(ALICE, FTAction::BalanceOf(ALICE.into()));
    assert!(result.contains(&(ALICE, FTEvent::Balance(100).encode())));
}