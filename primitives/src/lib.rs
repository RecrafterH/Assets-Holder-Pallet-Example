#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen, DecodeWithMemTracking};
use scale_info::TypeInfo;
use frame::deps::frame_support::traits::VariantCount;

#[derive(
	Decode,
	DecodeWithMemTracking,
	Encode,
	MaxEncodedLen,
	PartialEq,
	Eq,
	Ord,
	PartialOrd,
	TypeInfo,
	Debug,
	Clone,
	Copy,
)]
pub enum DummyHoldReason {
	Governance,
	Staking,
	Other,
}

impl VariantCount for DummyHoldReason {
	// Intentionally set below the actual count of variants, to allow testing for `can_freeze`
	const VARIANT_COUNT: u32 = 3;
}