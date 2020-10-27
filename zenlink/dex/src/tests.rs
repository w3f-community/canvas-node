//! Unit tests for the dex module.

#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{
	DexModule, ExtBuilder, Origin, Runtime, System, TestEvent, Tokens, ZLK, ALICE, ZUSD, AUSD_DOT_PAIR, AUSD_XBTC_PAIR,
	BOB, DOT, XBTC,
};

#[test]
fn get_liquidity_work() {
	ExtBuilder::default().build().execute_with(|| {
		LiquidityPool::insert(AUSD_DOT_PAIR, (1000, 20));
		assert_eq!(DexModule::liquidity_pool(AUSD_DOT_PAIR), (1000, 20));
		assert_eq!(DexModule::get_liquidity(ZUSD, DOT), (1000, 20));
		assert_eq!(DexModule::get_liquidity(DOT, ZUSD), (20, 1000));
	});
}

#[test]
fn get_target_amount_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(DexModule::get_target_amount(10000, 0, 1000), 0);
		assert_eq!(DexModule::get_target_amount(0, 20000, 1000), 0);
		assert_eq!(DexModule::get_target_amount(10000, 20000, 0), 0);
		assert_eq!(DexModule::get_target_amount(10000, 1, 1000000), 0);
		assert_eq!(DexModule::get_target_amount(10000, 20000, 10000), 9949);
		assert_eq!(DexModule::get_target_amount(10000, 20000, 1000), 1801);
	});
}

#[test]
fn get_supply_amount_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(DexModule::get_supply_amount(10000, 0, 1000), 0);
		assert_eq!(DexModule::get_supply_amount(0, 20000, 1000), 0);
		assert_eq!(DexModule::get_supply_amount(10000, 20000, 0), 0);
		assert_eq!(DexModule::get_supply_amount(10000, 1, 1), 0);
		assert_eq!(DexModule::get_supply_amount(10000, 20000, 9949), 9999);
		assert_eq!(DexModule::get_target_amount(10000, 20000, 9999), 9949);
		assert_eq!(DexModule::get_supply_amount(10000, 20000, 1801), 1000);
		assert_eq!(DexModule::get_target_amount(10000, 20000, 1000), 1801);
	});
}

#[test]
fn get_target_amounts_work() {
	ExtBuilder::default().build().execute_with(|| {
		LiquidityPool::insert(AUSD_DOT_PAIR, (50000, 10000));
		LiquidityPool::insert(AUSD_XBTC_PAIR, (100000, 10));
		assert_noop!(
			DexModule::get_target_amounts(&vec![DOT], 10000, None),
			Error::<Runtime>::InvalidTradingPathLength,
		);
		assert_noop!(
			DexModule::get_target_amounts(&vec![DOT, ZUSD, XBTC, DOT], 10000, None),
			Error::<Runtime>::InvalidTradingPathLength,
		);
		assert_noop!(
			DexModule::get_target_amounts(&vec![DOT, ZUSD, ZLK], 10000, None),
			Error::<Runtime>::TradingPairNotAllowed,
		);
		assert_eq!(
			DexModule::get_target_amounts(&vec![DOT, ZUSD], 10000, None),
			Ok(vec![10000, 24874])
		);
		assert_eq!(
			DexModule::get_target_amounts(&vec![DOT, ZUSD], 10000, Ratio::checked_from_rational(50, 100)),
			Ok(vec![10000, 24874])
		);
		assert_noop!(
			DexModule::get_target_amounts(&vec![DOT, ZUSD], 10000, Ratio::checked_from_rational(49, 100)),
			Error::<Runtime>::ExceedPriceImpactLimit,
		);
		assert_eq!(
			DexModule::get_target_amounts(&vec![DOT, ZUSD, XBTC], 10000, None),
			Ok(vec![10000, 24874, 1])
		);
		assert_noop!(
			DexModule::get_target_amounts(&vec![DOT, ZUSD, XBTC], 100, None),
			Error::<Runtime>::ZeroTargetAmount,
		);
		assert_noop!(
			DexModule::get_target_amounts(&vec![DOT, XBTC], 100, None),
			Error::<Runtime>::InsufficientLiquidity,
		);
	});
}

#[test]
fn calculate_amount_for_big_number_work() {
	ExtBuilder::default().build().execute_with(|| {
		LiquidityPool::insert(
			AUSD_DOT_PAIR,
			(171_000_000_000_000_000_000_000, 56_000_000_000_000_000_000_000),
		);
		assert_eq!(
			DexModule::get_supply_amount(
				171_000_000_000_000_000_000_000,
				56_000_000_000_000_000_000_000,
				1_000_000_000_000_000_000_000
			),
			3_140_495_867_768_595_041_323
		);
		assert_eq!(
			DexModule::get_target_amount(
				171_000_000_000_000_000_000_000,
				56_000_000_000_000_000_000_000,
				3_140_495_867_768_595_041_323
			),
			1_000_000_000_000_000_000_000
		);
	});
}

#[test]
fn get_supply_amounts_work() {
	ExtBuilder::default().build().execute_with(|| {
		LiquidityPool::insert(AUSD_DOT_PAIR, (50000, 10000));
		LiquidityPool::insert(AUSD_XBTC_PAIR, (100000, 10));
		assert_noop!(
			DexModule::get_supply_amounts(&vec![DOT], 10000, None),
			Error::<Runtime>::InvalidTradingPathLength,
		);
		assert_noop!(
			DexModule::get_supply_amounts(&vec![DOT, ZUSD, XBTC, DOT], 10000, None),
			Error::<Runtime>::InvalidTradingPathLength,
		);
		assert_noop!(
			DexModule::get_supply_amounts(&vec![DOT, ZUSD, ZLK], 10000, None),
			Error::<Runtime>::TradingPairNotAllowed,
		);
		assert_eq!(
			DexModule::get_supply_amounts(&vec![DOT, ZUSD], 24874, None),
			Ok(vec![10000, 24874])
		);
		assert_eq!(
			DexModule::get_supply_amounts(&vec![DOT, ZUSD], 25000, Ratio::checked_from_rational(50, 100)),
			Ok(vec![10102, 25000])
		);
		assert_noop!(
			DexModule::get_supply_amounts(&vec![DOT, ZUSD], 25000, Ratio::checked_from_rational(49, 100)),
			Error::<Runtime>::ExceedPriceImpactLimit,
		);
		assert_noop!(
			DexModule::get_supply_amounts(&vec![DOT, ZUSD, XBTC], 10000, None),
			Error::<Runtime>::ZeroSupplyAmount,
		);
		assert_noop!(
			DexModule::get_supply_amounts(&vec![DOT, XBTC], 10000, None),
			Error::<Runtime>::InsufficientLiquidity,
		);
	});
}

#[test]
fn _swap_work() {
	ExtBuilder::default().build().execute_with(|| {
		LiquidityPool::insert(AUSD_DOT_PAIR, (50000, 10000));

		assert_eq!(DexModule::get_liquidity(ZUSD, DOT), (50000, 10000));
		DexModule::_swap(ZUSD, DOT, 1000, 1000);
		assert_eq!(DexModule::get_liquidity(ZUSD, DOT), (51000, 9000));
		DexModule::_swap(DOT, ZUSD, 100, 800);
		assert_eq!(DexModule::get_liquidity(ZUSD, DOT), (50200, 9100));
	});
}

#[test]
fn _swap_by_path_work() {
	ExtBuilder::default().build().execute_with(|| {
		LiquidityPool::insert(AUSD_DOT_PAIR, (50000, 10000));
		LiquidityPool::insert(AUSD_XBTC_PAIR, (100000, 10));

		assert_eq!(DexModule::get_liquidity(ZUSD, DOT), (50000, 10000));
		assert_eq!(DexModule::get_liquidity(ZUSD, XBTC), (100000, 10));
		DexModule::_swap_by_path(&vec![DOT, ZUSD], &vec![10000, 25000]);
		assert_eq!(DexModule::get_liquidity(ZUSD, DOT), (25000, 20000));
		DexModule::_swap_by_path(&vec![DOT, ZUSD, XBTC], &vec![4000, 10000, 2]);
		assert_eq!(DexModule::get_liquidity(ZUSD, DOT), (15000, 24000));
		assert_eq!(DexModule::get_liquidity(ZUSD, XBTC), (110000, 8));
	});
}

#[test]
fn add_liquidity_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_noop!(
			DexModule::add_liquidity(Origin::signed(ALICE), ZLK, ZUSD, 100_000_000, 100_000_000),
			Error::<Runtime>::TradingPairNotAllowed
		);
		assert_noop!(
			DexModule::add_liquidity(Origin::signed(ALICE), ZUSD, DOT, 0, 100_000_000),
			Error::<Runtime>::InvalidLiquidityIncrement
		);

		assert_eq!(DexModule::get_liquidity(ZUSD, DOT), (0, 0));
		assert_eq!(Tokens::free_balance(ZUSD, &DexModule::account_id()), 0);
		assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 0);
		assert_eq!(
			Tokens::free_balance(AUSD_DOT_PAIR.get_dex_share_currency_id().unwrap(), &ALICE),
			0
		);
		assert_eq!(Tokens::free_balance(ZUSD, &ALICE), 1_000_000_000_000_000_000);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 1_000_000_000_000_000_000);

		assert_ok!(DexModule::add_liquidity(
			Origin::signed(ALICE),
			ZUSD,
			DOT,
			5_000_000_000_000,
			1_000_000_000_000
		));
		let add_liquidity_event_1 = TestEvent::dex(RawEvent::AddLiquidity(
			ALICE,
			ZUSD,
			5_000_000_000_000,
			DOT,
			1_000_000_000_000,
			5_000_000_000_000,
		));
		assert!(System::events()
			.iter()
			.any(|record| record.event == add_liquidity_event_1));

		assert_eq!(
			DexModule::get_liquidity(ZUSD, DOT),
			(5_000_000_000_000, 1_000_000_000_000)
		);
		assert_eq!(Tokens::free_balance(ZUSD, &DexModule::account_id()), 5_000_000_000_000);
		assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 1_000_000_000_000);
		assert_eq!(
			Tokens::free_balance(AUSD_DOT_PAIR.get_dex_share_currency_id().unwrap(), &ALICE),
			5_000_000_000_000
		);
		assert_eq!(Tokens::free_balance(ZUSD, &ALICE), 999_995_000_000_000_000);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 999_999_000_000_000_000);
		assert_eq!(
			Tokens::free_balance(AUSD_DOT_PAIR.get_dex_share_currency_id().unwrap(), &BOB),
			0
		);
		assert_eq!(Tokens::free_balance(ZUSD, &BOB), 1_000_000_000_000_000_000);
		assert_eq!(Tokens::free_balance(DOT, &BOB), 1_000_000_000_000_000_000);

		assert_ok!(DexModule::add_liquidity(
			Origin::signed(BOB),
			ZUSD,
			DOT,
			50_000_000_000_000,
			8_000_000_000_000
		));
		let add_liquidity_event_2 = TestEvent::dex(RawEvent::AddLiquidity(
			BOB,
			ZUSD,
			40_000_000_000_000,
			DOT,
			8_000_000_000_000,
			40_000_000_000_000,
		));
		assert!(System::events()
			.iter()
			.any(|record| record.event == add_liquidity_event_2));

		assert_eq!(
			DexModule::get_liquidity(ZUSD, DOT),
			(45_000_000_000_000, 9_000_000_000_000)
		);
		assert_eq!(Tokens::free_balance(ZUSD, &DexModule::account_id()), 45_000_000_000_000);
		assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 9_000_000_000_000);
		assert_eq!(
			Tokens::free_balance(AUSD_DOT_PAIR.get_dex_share_currency_id().unwrap(), &BOB),
			40_000_000_000_000
		);
		assert_eq!(Tokens::free_balance(ZUSD, &BOB), 999_960_000_000_000_000);
		assert_eq!(Tokens::free_balance(DOT, &BOB), 999_992_000_000_000_000);
	});
}

#[test]
fn remove_liquidity_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(DexModule::add_liquidity(
			Origin::signed(ALICE),
			ZUSD,
			DOT,
			5_000_000_000_000,
			1_000_000_000_000
		));
		assert_noop!(
			DexModule::remove_liquidity(
				Origin::signed(ALICE),
				AUSD_DOT_PAIR.get_dex_share_currency_id().unwrap(),
				DOT,
				100_000_000
			),
			Error::<Runtime>::InvalidCurrencyId
		);

		assert_eq!(
			DexModule::get_liquidity(ZUSD, DOT),
			(5_000_000_000_000, 1_000_000_000_000)
		);
		assert_eq!(Tokens::free_balance(ZUSD, &DexModule::account_id()), 5_000_000_000_000);
		assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 1_000_000_000_000);
		assert_eq!(
			Tokens::free_balance(AUSD_DOT_PAIR.get_dex_share_currency_id().unwrap(), &ALICE),
			5_000_000_000_000
		);
		assert_eq!(Tokens::free_balance(ZUSD, &ALICE), 999_995_000_000_000_000);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 999_999_000_000_000_000);

		assert_ok!(DexModule::remove_liquidity(
			Origin::signed(ALICE),
			ZUSD,
			DOT,
			4_000_000_000_000
		));
		let remove_liquidity_event_1 = TestEvent::dex(RawEvent::RemoveLiquidity(
			ALICE,
			ZUSD,
			4_000_000_000_000,
			DOT,
			800_000_000_000,
			4_000_000_000_000,
		));
		assert!(System::events()
			.iter()
			.any(|record| record.event == remove_liquidity_event_1));

		assert_eq!(
			DexModule::get_liquidity(ZUSD, DOT),
			(1_000_000_000_000, 200_000_000_000)
		);
		assert_eq!(Tokens::free_balance(ZUSD, &DexModule::account_id()), 1_000_000_000_000);
		assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 200_000_000_000);
		assert_eq!(
			Tokens::free_balance(AUSD_DOT_PAIR.get_dex_share_currency_id().unwrap(), &ALICE),
			1_000_000_000_000
		);
		assert_eq!(Tokens::free_balance(ZUSD, &ALICE), 999_999_000_000_000_000);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 999_999_800_000_000_000);

		assert_ok!(DexModule::remove_liquidity(
			Origin::signed(ALICE),
			ZUSD,
			DOT,
			1_000_000_000_000
		));
		let remove_liquidity_event_2 = TestEvent::dex(RawEvent::RemoveLiquidity(
			ALICE,
			ZUSD,
			1_000_000_000_000,
			DOT,
			200_000_000_000,
			1_000_000_000_000,
		));
		assert!(System::events()
			.iter()
			.any(|record| record.event == remove_liquidity_event_2));

		assert_eq!(DexModule::get_liquidity(ZUSD, DOT), (0, 0));
		assert_eq!(Tokens::free_balance(ZUSD, &DexModule::account_id()), 0);
		assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 0);
		assert_eq!(
			Tokens::free_balance(AUSD_DOT_PAIR.get_dex_share_currency_id().unwrap(), &ALICE),
			0
		);
		assert_eq!(Tokens::free_balance(ZUSD, &ALICE), 1_000_000_000_000_000_000);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 1_000_000_000_000_000_000);
	});
}

#[test]
fn do_swap_with_exact_supply_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(DexModule::add_liquidity(
			Origin::signed(ALICE),
			ZUSD,
			DOT,
			500_000_000_000_000,
			100_000_000_000_000
		));
		assert_ok!(DexModule::add_liquidity(
			Origin::signed(ALICE),
			ZUSD,
			XBTC,
			100_000_000_000_000,
			10_000_000_000
		));

		assert_eq!(
			DexModule::get_liquidity(ZUSD, DOT),
			(500_000_000_000_000, 100_000_000_000_000)
		);
		assert_eq!(
			DexModule::get_liquidity(ZUSD, XBTC),
			(100_000_000_000_000, 10_000_000_000)
		);
		assert_eq!(
			Tokens::free_balance(ZUSD, &DexModule::account_id()),
			600_000_000_000_000
		);
		assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 100_000_000_000_000);
		assert_eq!(Tokens::free_balance(XBTC, &DexModule::account_id()), 10_000_000_000);
		assert_eq!(Tokens::free_balance(ZUSD, &BOB), 1_000_000_000_000_000_000);
		assert_eq!(Tokens::free_balance(DOT, &BOB), 1_000_000_000_000_000_000);
		assert_eq!(Tokens::free_balance(XBTC, &BOB), 1_000_000_000_000_000_000);

		assert_noop!(
			DexModule::do_swap_with_exact_supply(&BOB, &[DOT, ZUSD], 100_000_000_000_000, 250_000_000_000_000, None),
			Error::<Runtime>::InsufficientTargetAmount
		);
		assert_noop!(
			DexModule::do_swap_with_exact_supply(
				&BOB,
				&[DOT, ZUSD],
				100_000_000_000_000,
				0,
				Ratio::checked_from_rational(10, 100)
			),
			Error::<Runtime>::ExceedPriceImpactLimit,
		);
		assert_noop!(
			DexModule::do_swap_with_exact_supply(&BOB, &[DOT, ZUSD, XBTC, DOT], 100_000_000_000_000, 0, None),
			Error::<Runtime>::InvalidTradingPathLength,
		);
		assert_noop!(
			DexModule::do_swap_with_exact_supply(&BOB, &[DOT, ZLK], 100_000_000_000_000, 0, None),
			Error::<Runtime>::TradingPairNotAllowed,
		);

		assert_ok!(DexModule::do_swap_with_exact_supply(
			&BOB,
			&[DOT, ZUSD],
			100_000_000_000_000,
			200_000_000_000_000,
			None
		));
		let swap_event_1 = TestEvent::dex(RawEvent::Swap(
			BOB,
			vec![DOT, ZUSD],
			100_000_000_000_000,
			248_743_718_592_964,
		));
		assert!(System::events().iter().any(|record| record.event == swap_event_1));

		assert_eq!(
			DexModule::get_liquidity(ZUSD, DOT),
			(251_256_281_407_036, 200_000_000_000_000)
		);
		assert_eq!(
			DexModule::get_liquidity(ZUSD, XBTC),
			(100_000_000_000_000, 10_000_000_000)
		);
		assert_eq!(
			Tokens::free_balance(ZUSD, &DexModule::account_id()),
			351_256_281_407_036
		);
		assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 200_000_000_000_000);
		assert_eq!(Tokens::free_balance(XBTC, &DexModule::account_id()), 10_000_000_000);
		assert_eq!(Tokens::free_balance(ZUSD, &BOB), 1_000_248_743_718_592_964);
		assert_eq!(Tokens::free_balance(DOT, &BOB), 999_900_000_000_000_000);
		assert_eq!(Tokens::free_balance(XBTC, &BOB), 1_000_000_000_000_000_000);

		assert_ok!(DexModule::do_swap_with_exact_supply(
			&BOB,
			&[DOT, ZUSD, XBTC],
			200_000_000_000_000,
			1,
			None
		));
		let swap_event_2 = TestEvent::dex(RawEvent::Swap(
			BOB,
			vec![DOT, ZUSD, XBTC],
			200_000_000_000_000,
			5_530_663_837,
		));
		assert!(System::events().iter().any(|record| record.event == swap_event_2));

		assert_eq!(
			DexModule::get_liquidity(ZUSD, DOT),
			(126_259_437_892_983, 400_000_000_000_000)
		);
		assert_eq!(
			DexModule::get_liquidity(ZUSD, XBTC),
			(224_996_843_514_053, 4_469_336_163)
		);
		assert_eq!(
			Tokens::free_balance(ZUSD, &DexModule::account_id()),
			351_256_281_407_036
		);
		assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 400_000_000_000_000);
		assert_eq!(Tokens::free_balance(XBTC, &DexModule::account_id()), 4_469_336_163);
		assert_eq!(Tokens::free_balance(ZUSD, &BOB), 1_000_248_743_718_592_964);
		assert_eq!(Tokens::free_balance(DOT, &BOB), 999_700_000_000_000_000);
		assert_eq!(Tokens::free_balance(XBTC, &BOB), 1_000_000_005_530_663_837);
	});
}

#[test]
fn do_swap_with_exact_target_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(DexModule::add_liquidity(
			Origin::signed(ALICE),
			ZUSD,
			DOT,
			500_000_000_000_000,
			100_000_000_000_000
		));
		assert_ok!(DexModule::add_liquidity(
			Origin::signed(ALICE),
			ZUSD,
			XBTC,
			100_000_000_000_000,
			10_000_000_000
		));

		assert_eq!(
			DexModule::get_liquidity(ZUSD, DOT),
			(500_000_000_000_000, 100_000_000_000_000)
		);
		assert_eq!(
			DexModule::get_liquidity(ZUSD, XBTC),
			(100_000_000_000_000, 10_000_000_000)
		);
		assert_eq!(
			Tokens::free_balance(ZUSD, &DexModule::account_id()),
			600_000_000_000_000
		);
		assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 100_000_000_000_000);
		assert_eq!(Tokens::free_balance(XBTC, &DexModule::account_id()), 10_000_000_000);
		assert_eq!(Tokens::free_balance(ZUSD, &BOB), 1_000_000_000_000_000_000);
		assert_eq!(Tokens::free_balance(DOT, &BOB), 1_000_000_000_000_000_000);
		assert_eq!(Tokens::free_balance(XBTC, &BOB), 1_000_000_000_000_000_000);

		assert_noop!(
			DexModule::do_swap_with_exact_target(&BOB, &[DOT, ZUSD], 250_000_000_000_000, 100_000_000_000_000, None),
			Error::<Runtime>::ExcessiveSupplyAmount
		);
		assert_noop!(
			DexModule::do_swap_with_exact_target(
				&BOB,
				&[DOT, ZUSD],
				250_000_000_000_000,
				200_000_000_000_000,
				Ratio::checked_from_rational(10, 100)
			),
			Error::<Runtime>::ExceedPriceImpactLimit,
		);
		assert_noop!(
			DexModule::do_swap_with_exact_target(
				&BOB,
				&[DOT, ZUSD, XBTC, DOT],
				250_000_000_000_000,
				200_000_000_000_000,
				None
			),
			Error::<Runtime>::InvalidTradingPathLength,
		);
		assert_noop!(
			DexModule::do_swap_with_exact_target(&BOB, &[DOT, ZLK], 250_000_000_000_000, 200_000_000_000_000, None),
			Error::<Runtime>::TradingPairNotAllowed,
		);

		assert_ok!(DexModule::do_swap_with_exact_target(
			&BOB,
			&[DOT, ZUSD],
			250_000_000_000_000,
			200_000_000_000_000,
			None
		));
		let swap_event_1 = TestEvent::dex(RawEvent::Swap(
			BOB,
			vec![DOT, ZUSD],
			101_010_101_010_102,
			250_000_000_000_000,
		));
		assert!(System::events().iter().any(|record| record.event == swap_event_1));

		assert_eq!(
			DexModule::get_liquidity(ZUSD, DOT),
			(250_000_000_000_000, 201_010_101_010_102)
		);
		assert_eq!(
			DexModule::get_liquidity(ZUSD, XBTC),
			(100_000_000_000_000, 10_000_000_000)
		);
		assert_eq!(
			Tokens::free_balance(ZUSD, &DexModule::account_id()),
			350_000_000_000_000
		);
		assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 201_010_101_010_102);
		assert_eq!(Tokens::free_balance(XBTC, &DexModule::account_id()), 10_000_000_000);
		assert_eq!(Tokens::free_balance(ZUSD, &BOB), 1_000_250_000_000_000_000);
		assert_eq!(Tokens::free_balance(DOT, &BOB), 999_898_989_898_989_898);
		assert_eq!(Tokens::free_balance(XBTC, &BOB), 1_000_000_000_000_000_000);

		assert_ok!(DexModule::do_swap_with_exact_target(
			&BOB,
			&[DOT, ZUSD, XBTC],
			5_000_000_000,
			2_000_000_000_000_000,
			None
		));
		let swap_event_2 = TestEvent::dex(RawEvent::Swap(
			BOB,
			vec![DOT, ZUSD, XBTC],
			137_654_580_386_993,
			5_000_000_000,
		));
		assert!(System::events().iter().any(|record| record.event == swap_event_2));

		assert_eq!(
			DexModule::get_liquidity(ZUSD, DOT),
			(148_989_898_989_898, 338_664_681_397_095)
		);
		assert_eq!(
			DexModule::get_liquidity(ZUSD, XBTC),
			(201_010_101_010_102, 5_000_000_000)
		);
		assert_eq!(
			Tokens::free_balance(ZUSD, &DexModule::account_id()),
			350_000_000_000_000
		);
		assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 338_664_681_397_095);
		assert_eq!(Tokens::free_balance(XBTC, &DexModule::account_id()), 5_000_000_000);
		assert_eq!(Tokens::free_balance(ZUSD, &BOB), 1_000_250_000_000_000_000);
		assert_eq!(Tokens::free_balance(DOT, &BOB), 999_761_335_318_602_905);
		assert_eq!(Tokens::free_balance(XBTC, &BOB), 1_000_000_005_000_000_000);
	});
}
