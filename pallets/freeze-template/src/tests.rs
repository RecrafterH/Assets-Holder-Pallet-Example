use crate::mock::*;
use frame::testing_prelude::*;
use frame::deps::frame_support::traits::fungibles::InspectFreeze;
use polkadot_sdk::pallet_assets::{Instance1, Error as AssetError};
use primitives::DummyFreezeReason;
use frame::deps::sp_runtime::{TokenError, ArithmeticError};

#[test]
fn hold_with_1_reason() {
	new_test_ext().execute_with(|| {
		assert_eq!(Assets::balance(1, 0), 100);
		assert_ok!(Template::make_freeze_other(RuntimeOrigin::signed(0), 30));
		assert_ok!(Template::make_freeze_other(RuntimeOrigin::signed(0), 30));
		assert_eq!(Assets::balance(1, 0), 100);
		assert_eq!(AssetsFreezer::balance_frozen(1, &DummyFreezeReason::Other, &0), 60);
		assert_noop!(Assets::transfer(RuntimeOrigin::signed(0), codec::Compact(1), 1, 60), AssetError::<Test, Instance1>::BalanceLow);
	});
}

#[test]
fn hold_with_2_reason() {
	new_test_ext().execute_with(|| {
		assert_eq!(Assets::balance(1, 0), 100);
		assert_ok!(Template::make_freeze_other(RuntimeOrigin::signed(0), 60));
		assert_ok!(Template::make_freeze_staking(RuntimeOrigin::signed(0), 60));
		assert_eq!(Assets::balance(1, 0), 100);
		assert_eq!(AssetsFreezer::balance_frozen(1, &DummyFreezeReason::Other, &0), 60);
		assert_eq!(AssetsFreezer::balance_frozen(1, &DummyFreezeReason::Staking, &0), 60);
		assert_noop!(Assets::transfer(RuntimeOrigin::signed(0), codec::Compact(1), 1, 60), AssetError::<Test, Instance1>::BalanceLow);
		assert_ok!(Assets::transfer(RuntimeOrigin::signed(0), codec::Compact(1), 1, 40));
	});
}

#[test]
fn releaes_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(Assets::balance(1, 0), 100);
		assert_ok!(Template::make_freeze_other(RuntimeOrigin::signed(0), 30));
		assert_ok!(Template::make_freeze_staking(RuntimeOrigin::signed(0), 30));
		assert_ok!(Template::release_freeze_other(RuntimeOrigin::signed(0), 20));
		assert_eq!(Assets::balance(1, 0), 100);
		assert_eq!(AssetsFreezer::balance_frozen(1, &DummyFreezeReason::Other, &0), 10);
		assert_eq!(AssetsFreezer::balance_frozen(1, &DummyFreezeReason::Staking, &0), 30);
		assert_noop!(Assets::transfer(RuntimeOrigin::signed(0), codec::Compact(1), 1, 80), AssetError::<Test, Instance1>::BalanceLow);
	});
}

#[test]
fn releaes_works_2() {
	new_test_ext().execute_with(|| {
		assert_eq!(Assets::balance(1, 0), 100);
		assert_ok!(Template::make_freeze_other(RuntimeOrigin::signed(0), 30));
		assert_ok!(Template::make_freeze_staking(RuntimeOrigin::signed(0), 30));
		assert_ok!(Template::release_freeze_other(RuntimeOrigin::signed(0), 30));
		assert_ok!(Template::release_freeze_staking(RuntimeOrigin::signed(0), 30));
		assert_eq!(Assets::balance(1, 0), 100);
		assert_eq!(AssetsFreezer::balance_frozen(1, &DummyFreezeReason::Other, &0), 0);
		assert_eq!(AssetsFreezer::balance_frozen(1, &DummyFreezeReason::Staking, &0), 0);
		assert_ok!(Assets::transfer(RuntimeOrigin::signed(0), codec::Compact(1), 1, 100));
	});
}

#[test]
fn cant_hold_or_release_too_much() {
	new_test_ext().execute_with(|| {
		assert_eq!(Assets::balance(1, 0), 100);
		assert_ok!(Template::make_freeze_other(RuntimeOrigin::signed(0), 60));
		assert_noop!(Template::make_freeze_other(RuntimeOrigin::signed(0), 50), TokenError::FundsUnavailable);
		assert_noop!(Template::make_freeze_staking(RuntimeOrigin::signed(0), 101), TokenError::FundsUnavailable);
		assert_noop!(Template::release_freeze_other(RuntimeOrigin::signed(0), 80), ArithmeticError::Underflow);
		assert_noop!(Template::release_freeze_staking(RuntimeOrigin::signed(0), 10), ArithmeticError::Underflow);
	});
}