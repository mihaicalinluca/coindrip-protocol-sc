use coindrip::{
    coindrip_proxy,
    errors::{
        ERR_CANCEL_ONLY_OWNERS, ERR_CANT_CANCEL, ERR_END_TIME, ERR_INVALID_STREAM,
        ERR_ONLY_RECIPIENT_CLAIM, ERR_ONLY_RECIPIENT_SENDER_CAN_CLAIM, ERR_START_TIME,
        ERR_STREAM_IS_NOT_CANCELLED, ERR_STREAM_TO_CALLER, ERR_STREAM_TO_SC, ERR_ZERO_CLAIM,
        ERR_ZERO_DEPOSIT,
    },
};
use contract_setup::{
    setup, COINDRIP_ADDRESS, CURRENT_TIMESTAMP, FIRST_USER, INITIAL_OWNER_TOKEN_BALANCE,
    OWNER_ADDRESS, SECOND_USER, TOKEN_ID,
};
use multiversx_sc::{
    codec::multi_types::OptionalValue,
    types::{BigUint, ReturnsResult},
};
use multiversx_sc_scenario::imports::*;

mod contract_setup;

#[test]
fn deploy_test() {
    let _ = setup(); // deploy and check inside
}

#[test]
fn create_stream_test() {
    let mut world = setup();

    world.current_block().block_timestamp(CURRENT_TIMESTAMP);

    // Create a valid stream of 3K tokens
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .create_stream(
            FIRST_USER,
            CURRENT_TIMESTAMP + 60,
            CURRENT_TIMESTAMP + 60 * 60,
            OptionalValue::<bool>::None,
        )
        .single_esdt(&TOKEN_ID.into(), 0, &BigUint::from(3_000u64))
        .run();

    let user_deposit = world
        .query()
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .streams_list(FIRST_USER)
        .returns(ReturnsResult)
        .run();

    assert_eq!(user_deposit.len(), 1);

    // Create an invalid stream of 0 tokens
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .create_stream(
            FIRST_USER,
            CURRENT_TIMESTAMP + 60,
            CURRENT_TIMESTAMP + 60 * 60,
            OptionalValue::<bool>::None,
        )
        .single_esdt(&TOKEN_ID.into(), 0, &BigUint::zero())
        .returns(ExpectError(4u64, ERR_ZERO_DEPOSIT))
        .run();

    // Stream towards the SC
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .create_stream(
            COINDRIP_ADDRESS,
            CURRENT_TIMESTAMP + 60,
            CURRENT_TIMESTAMP + 60 * 60,
            OptionalValue::<bool>::None,
        )
        .single_esdt(&TOKEN_ID.into(), 0, &BigUint::from(3_000u64))
        .returns(ExpectError(4u64, ERR_STREAM_TO_SC))
        .run();

    // Stream towards the caller
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .create_stream(
            OWNER_ADDRESS,
            CURRENT_TIMESTAMP + 60,
            CURRENT_TIMESTAMP + 60 * 60,
            OptionalValue::<bool>::None,
        )
        .single_esdt(&TOKEN_ID.into(), 0, &BigUint::from(3_000u64))
        .returns(ExpectError(4u64, ERR_STREAM_TO_CALLER))
        .run();

    // Start time before current time
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .create_stream(
            FIRST_USER,
            CURRENT_TIMESTAMP - 60,
            CURRENT_TIMESTAMP + 60 * 60,
            OptionalValue::<bool>::None,
        )
        .single_esdt(&TOKEN_ID.into(), 0, &BigUint::from(3_000u64))
        .returns(ExpectError(4u64, ERR_START_TIME))
        .run();

    // End time before start time
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .create_stream(
            FIRST_USER,
            CURRENT_TIMESTAMP + 60 * 60,
            CURRENT_TIMESTAMP + 60,
            OptionalValue::<bool>::None,
        )
        .single_esdt(&TOKEN_ID.into(), 0, &BigUint::from(3_000u64))
        .returns(ExpectError(4u64, ERR_END_TIME))
        .run();
}

#[test]
fn claim_from_stream_test() {
    let mut world = setup();

    world.current_block().block_timestamp(CURRENT_TIMESTAMP);

    // Create a valid stream of 3K tokens
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .create_stream(
            FIRST_USER,
            CURRENT_TIMESTAMP + 60,
            CURRENT_TIMESTAMP + 60 * 3,
            OptionalValue::<bool>::None,
        )
        .single_esdt(&TOKEN_ID.into(), 0, &BigUint::from(3_000u64))
        .run();

    // Claim from stream wrong recipient
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .claim_from_stream(1u64)
        .returns(ExpectError(4u64, ERR_ONLY_RECIPIENT_CLAIM))
        .run();

    // Amount to claim is zero
    world
        .tx()
        .from(FIRST_USER)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .claim_from_stream(1u64)
        .returns(ExpectError(4u64, ERR_ZERO_CLAIM))
        .run();

    world
        .current_block()
        .block_timestamp(CURRENT_TIMESTAMP + 60 * 2);

    // Claim 1.5K tokens
    world
        .tx()
        .from(FIRST_USER)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .claim_from_stream(1u64)
        .run();

    world.check_account(FIRST_USER).esdt_balance(TOKEN_ID, 1500);
    world
        .current_block()
        .block_timestamp(CURRENT_TIMESTAMP + 60 * 5);

    // Claim rest of the 1.5K tokens
    world
        .tx()
        .from(FIRST_USER)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .claim_from_stream(1u64)
        .run();

    world.check_account(FIRST_USER).esdt_balance(TOKEN_ID, 3000);

    // Stream is deleted
    world
        .tx()
        .from(FIRST_USER)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .claim_from_stream(1u64)
        .returns(ExpectError(4u64, ERR_INVALID_STREAM))
        .run();

    // Check storage updates
    let user_deposit = world
        .query()
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .streams_list(FIRST_USER)
        .returns(ReturnsResult)
        .run();

    assert_eq!(user_deposit.len(), 0);
}

#[test]
fn cancel_stream_test() {
    let mut world = setup();

    world.current_block().block_timestamp(CURRENT_TIMESTAMP);

    // Create a valid stream of 3K tokens
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .create_stream(
            FIRST_USER,
            CURRENT_TIMESTAMP + 60,
            CURRENT_TIMESTAMP + 60 * 3,
            OptionalValue::<bool>::None,
        )
        .single_esdt(&TOKEN_ID.into(), 0, &BigUint::from(3_000u64))
        .run();

    // Only sender and recipient can cancel stream
    world
        .tx()
        .from(SECOND_USER)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .cancel_stream(1u64, OptionalValue::<bool>::None)
        .returns(ExpectError(4u64, ERR_CANCEL_ONLY_OWNERS))
        .run();

    world
        .current_block()
        .block_timestamp(CURRENT_TIMESTAMP + 60 * 2);

    // Cancel stream in the middle
    world
        .tx()
        .from(FIRST_USER)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .cancel_stream(1u64, OptionalValue::<bool>::None)
        .run();

    world.check_account(FIRST_USER).esdt_balance(TOKEN_ID, 1500);
    world
        .check_account(OWNER_ADDRESS)
        .esdt_balance(TOKEN_ID, INITIAL_OWNER_TOKEN_BALANCE - 3000u64);

    world.current_block().block_timestamp(CURRENT_TIMESTAMP);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .create_stream(
            FIRST_USER,
            CURRENT_TIMESTAMP + 60,
            CURRENT_TIMESTAMP + 60 * 3,
            OptionalValue::Some(false),
        )
        .single_esdt(&TOKEN_ID.into(), 0u64, &BigUint::from(3000u64))
        .run();

    world
        .tx()
        .from(FIRST_USER)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .cancel_stream(2u64, OptionalValue::<bool>::None)
        .returns(ExpectError(4u64, ERR_CANT_CANCEL))
        .run();
}

#[test]
fn claim_from_stream_after_cancel_test() {
    let mut world = setup();

    world.current_block().block_timestamp(CURRENT_TIMESTAMP);

    // Create a valid stream of 3K tokens
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .create_stream(
            FIRST_USER,
            CURRENT_TIMESTAMP + 60,
            CURRENT_TIMESTAMP + 60 * 3,
            OptionalValue::<bool>::None,
        )
        .single_esdt(&TOKEN_ID.into(), 0, &BigUint::from(3_000u64))
        .run();

    world
        .current_block()
        .block_timestamp(CURRENT_TIMESTAMP + 60 * 2);

    world
        .tx()
        .from(FIRST_USER)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .claim_from_stream_after_cancel(1u64)
        .returns(ExpectError(4u64, ERR_STREAM_IS_NOT_CANCELLED))
        .run();

    // Cancel stream in the middle
    world
        .tx()
        .from(FIRST_USER)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .cancel_stream(1u64, OptionalValue::Some(false))
        .run();

    world.check_account(FIRST_USER).esdt_balance(TOKEN_ID, 0);

    world
        .check_account(OWNER_ADDRESS)
        .esdt_balance(TOKEN_ID, INITIAL_OWNER_TOKEN_BALANCE - 3_000u64);

    world
        .current_block()
        .block_timestamp(CURRENT_TIMESTAMP + 60 * 6);

    world
        .tx()
        .from(SECOND_USER)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .claim_from_stream_after_cancel(1u64)
        .returns(ExpectError(4u64, ERR_ONLY_RECIPIENT_SENDER_CAN_CLAIM))
        .run();

    world
        .tx()
        .from(FIRST_USER)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .claim_from_stream_after_cancel(1u64)
        .run();

    world.check_account(FIRST_USER).esdt_balance(TOKEN_ID, 1500);

    world
        .check_account(OWNER_ADDRESS)
        .esdt_balance(TOKEN_ID, INITIAL_OWNER_TOKEN_BALANCE - 3_000u64);

    world
        .tx()
        .from(FIRST_USER)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .claim_from_stream_after_cancel(1u64)
        .returns(ExpectError(4u64, ERR_ZERO_CLAIM))
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .claim_from_stream_after_cancel(1u64)
        .run();

    world.check_account(FIRST_USER).esdt_balance(TOKEN_ID, 1500);
    world
        .check_account(OWNER_ADDRESS)
        .esdt_balance(TOKEN_ID, INITIAL_OWNER_TOKEN_BALANCE - 1500u64);
}

#[test]
fn streamed_so_far_test() {
    let mut world = setup();

    world.current_block().block_timestamp(CURRENT_TIMESTAMP);

    // Create a valid stream of 3K tokens
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .create_stream(
            FIRST_USER,
            CURRENT_TIMESTAMP + 60,
            CURRENT_TIMESTAMP + 60 * 3,
            OptionalValue::<bool>::None,
        )
        .single_esdt(&TOKEN_ID.into(), 0, &BigUint::from(3_000u64))
        .run();

    // Streamed before start
    let streamed_start = world
        .query()
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .recipient_balance(1u64)
        .returns(ReturnsResult)
        .run();
    assert_eq!(streamed_start, BigUint::zero());

    world
        .current_block()
        .block_timestamp(CURRENT_TIMESTAMP + 60 * 2);

    // Streamed at half of the period
    let streamed_half = world
        .query()
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .recipient_balance(1u64)
        .returns(ReturnsResult)
        .run();
    assert_eq!(streamed_half, BigUint::from(1500u64));

    world
        .current_block()
        .block_timestamp(CURRENT_TIMESTAMP + 60 * 6);

    // Streamed after end time
    let streamed_end = world
        .query()
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .recipient_balance(1u64)
        .returns(ReturnsResult)
        .run();
    assert_eq!(streamed_end, BigUint::from(3000u64));
}

#[test]
fn claim_from_stream_rounding_test() {
    let mut world = setup();

    world.current_block().block_timestamp(CURRENT_TIMESTAMP);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .create_stream(
            FIRST_USER,
            CURRENT_TIMESTAMP + 60,
            CURRENT_TIMESTAMP + 60 * 31,
            OptionalValue::<bool>::None,
        )
        .single_esdt(&TOKEN_ID.into(), 0, &BigUint::from(2u64))
        .run();

    world
        .current_block()
        .block_timestamp(CURRENT_TIMESTAMP + 60 * 5);

    // Claim 0 tokens
    world
        .tx()
        .from(FIRST_USER)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .claim_from_stream(1u64)
        .returns(ExpectError(4u64, ERR_ZERO_CLAIM))
        .run();

    world.check_account(FIRST_USER).esdt_balance(TOKEN_ID, 0);

    world
        .current_block()
        .block_timestamp(CURRENT_TIMESTAMP + 60 * 26);

    // Claim 1 token
    world
        .tx()
        .from(FIRST_USER)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .claim_from_stream(1u64)
        .run();

    world.check_account(FIRST_USER).esdt_balance(TOKEN_ID, 1);

    world
        .current_block()
        .block_timestamp(CURRENT_TIMESTAMP + 60 * 31 + 60);

    // Claim 1 token
    world
        .tx()
        .from(FIRST_USER)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .claim_from_stream(1u64)
        .run();

    world.check_account(FIRST_USER).esdt_balance(TOKEN_ID, 2);

    // Stream is deleted
    world
        .tx()
        .from(FIRST_USER)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .claim_from_stream(1u64)
        .returns(ExpectError(4u64, ERR_INVALID_STREAM))
        .run();

    // Check storage updates
    let user_deposit = world
        .query()
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .streams_list(FIRST_USER)
        .returns(ReturnsResult)
        .run();
    assert_eq!(user_deposit.len(), 0);
}

#[test]
fn claim_from_stream_egld_test() {
    let mut world = setup();

    world.current_block().block_timestamp(CURRENT_TIMESTAMP);

    // Create a valid stream of 3K tokens
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .create_stream(
            FIRST_USER,
            CURRENT_TIMESTAMP + 60,
            CURRENT_TIMESTAMP + 60 * 3,
            OptionalValue::<bool>::None,
        )
        .egld(100)
        .run();

    // Amount to claim is zero
    world
        .tx()
        .from(FIRST_USER)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .claim_from_stream(1u64)
        .returns(ExpectError(4u64, ERR_ZERO_CLAIM))
        .run();

    world
        .current_block()
        .block_timestamp(CURRENT_TIMESTAMP + 60 * 2);

    // Claim 50 EGLD
    world
        .tx()
        .from(FIRST_USER)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .claim_from_stream(1u64)
        .run();

    world.check_account(FIRST_USER).balance(50);

    world
        .current_block()
        .block_timestamp(CURRENT_TIMESTAMP + 60 * 5);

    // Claim rest of the 50 EGLD
    world
        .tx()
        .from(FIRST_USER)
        .to(COINDRIP_ADDRESS)
        .typed(coindrip_proxy::CoinDripProxy)
        .claim_from_stream(1u64)
        .run();

    world.check_account(FIRST_USER).balance(100);
}
