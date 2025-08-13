use crate::{mock::*, Error};
use frame::testing_prelude::*;
use frame::deps::frame_support::traits::fungibles::InspectHold;
use polkadot_sdk::pallet_assets::{Instance1, Error as AssetError};

#[test]
fn hold_with_1_reason() {
	new_test_ext().execute_with(|| {
		assert_eq!(Assets::balance(1, 0), 100);
		assert_ok!(Template::make_hold_other(RuntimeOrigin::signed(0), 30));
		assert_ok!(Template::make_hold_other(RuntimeOrigin::signed(0), 30));
		assert_eq!(Assets::balance(1, 0), 40);
		assert_eq!(AssetsHolder::total_balance_on_hold(1, &0), 60);
		assert_noop!(Assets::transfer(RuntimeOrigin::signed(0), codec::Compact(1), 1, 60), AssetError::<Test, Instance1>::BalanceLow);
	});
}

#[test]
fn hold_with_2_reason() {
	new_test_ext().execute_with(|| {
		assert_eq!(Assets::balance(1, 0), 100);
		assert_ok!(Template::make_hold_other(RuntimeOrigin::signed(0), 30));
		assert_ok!(Template::make_hold_staking(RuntimeOrigin::signed(0), 30));
		assert_eq!(Assets::balance(1, 0), 40);
		assert_eq!(AssetsHolder::total_balance_on_hold(1, &0), 60);
		assert_noop!(Assets::transfer(RuntimeOrigin::signed(0), codec::Compact(1), 1, 60), AssetError::<Test, Instance1>::BalanceLow);
	});
}

#[test]
fn releaes_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(Assets::balance(1, 0), 100);
		assert_ok!(Template::make_hold_other(RuntimeOrigin::signed(0), 30));
		assert_ok!(Template::make_hold_staking(RuntimeOrigin::signed(0), 30));
		assert_ok!(Template::release_hold_other(RuntimeOrigin::signed(0), 20));
		assert_eq!(Assets::balance(1, 0), 60);
		assert_eq!(AssetsHolder::total_balance_on_hold(1, &0), 40);
		assert_ok!(Assets::transfer(RuntimeOrigin::signed(0), codec::Compact(1), 1, 59));
	});
}


#[test]
fn releaes_works_2() {
	new_test_ext().execute_with(|| {
		assert_eq!(Assets::balance(1, 0), 100);
		assert_ok!(Template::make_hold_other(RuntimeOrigin::signed(0), 30));
		assert_ok!(Template::make_hold_staking(RuntimeOrigin::signed(0), 30));
		assert_ok!(Template::release_hold_other(RuntimeOrigin::signed(0), 30));
		assert_ok!(Template::release_hold_staking(RuntimeOrigin::signed(0), 30));
		assert_eq!(Assets::balance(1, 0), 100);
		assert_eq!(AssetsHolder::total_balance_on_hold(1, &0), 0);
		assert_ok!(Assets::transfer(RuntimeOrigin::signed(0), codec::Compact(1), 1, 100));
	});
}

#[test]
fn cant_hold_or_release_too_much() {
	new_test_ext().execute_with(|| {
		assert_eq!(Assets::balance(1, 0), 100);
		assert_ok!(Template::make_hold_other(RuntimeOrigin::signed(0), 60));
		assert_noop!(Template::make_hold_other(RuntimeOrigin::signed(0), 50), Error::<Test>::NotEnoughFundsToHold);
		assert_noop!(Template::make_hold_staking(RuntimeOrigin::signed(0), 50), Error::<Test>::NotEnoughFundsToHold);
		assert_noop!(Template::release_hold_other(RuntimeOrigin::signed(0), 80), Error::<Test>::NotEnoughFundsToRelease);
		assert_noop!(Template::release_hold_staking(RuntimeOrigin::signed(0), 10), Error::<Test>::NotEnoughFundsToRelease);
	});
}