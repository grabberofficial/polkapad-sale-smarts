use codec::Encode;
use gtest::{System, Log};

use sale_io::*;

mod shared;
use shared::*;

#[test]
fn participate_in_sale_should_participated() {
    let system = System::new();
    init(&system);
    
    let registration_fee_gear = 1000;
    let plpd_amount = 100;
    let plpd_to_stake = 50;
    let total_user_gear_amount = 100 * 10e18 as u128;
    let to_participate = 5 * 10e18 as u128; 
    let total_user_ftr_amount = to_participate * 10_u128.pow(DECIMALS) / TOKEN_PRICE_IN_GEAR; // 5 * 10e16

    prepare_user_for_registration(&system, ALICE, total_user_gear_amount, plpd_amount, plpd_to_stake);

    let sale = system.get_program(SALE_ADDRESS);
    configure_sale(&system, &sale, registration_fee_gear);

    set_max_allocation_size_to_user(&system, ALICE, 5 * 10e16 as u128);

    let result = sale.send_with_value(ALICE, SaleAction::Participate, to_participate);
    let mailbox = system.get_mailbox(ALICE);
    let log = Log::builder()
        .dest(ALICE)
        .payload(SaleEvent::RegistrationGEARRefunded(ALICE.into(), registration_fee_gear));

    mailbox.claim_value(log.clone());

    assert!(result.contains(&log));
    assert!(sale.balance() == to_participate);
    assert!(system.balance_of(ALICE) == total_user_gear_amount - to_participate);
    
    let result = sale.send(ALICE, SaleAction::GetParticipationOf(ALICE.into()));
    assert!(result.contains(&(ALICE, SaleEvent::Participation(Participate {
        amount_bought: total_user_ftr_amount,
        amount_paid_gear: to_participate,
        participated_datetime: system.block_timestamp()
    }).encode())));

    let result = sale.send(ALICE, SaleAction::GetTotalRaised);
    assert!(result.contains(&(ALICE, SaleEvent::TotalRaised(to_participate).encode())));

    let result = sale.send(ALICE, SaleAction::GetTotalSold);
    assert!(result.contains(&(ALICE, SaleEvent::TotalSold(total_user_ftr_amount).encode())));
}