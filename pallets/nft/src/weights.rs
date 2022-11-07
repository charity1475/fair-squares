// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_nft
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-29, STEPS: `100`, REPEAT: 40, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ubuntu`, CPU: `AMD Ryzen 9 4900HS with Radeon Graphics`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/fs-node
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_nft
// --extrinsic
// *
// --steps
// 100
// --repeat
// 40
// --output
// pallets/nft/src/weights.rs
// --template
// assets/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_nft.
pub trait WeightInfo {
	fn create_collection() -> Weight;
	fn mint() -> Weight;
	fn transfer() -> Weight;
	fn destroy_collection() -> Weight;
	fn burn() -> Weight;
}

/// Weights for pallet_nft using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: RoleModule AccountsRolesLog (r:1 w:0)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	// Storage: NftModule Collections (r:0 w:1)
	fn create_collection() -> Weight {
		Weight::from_ref_time(64_040_000_u64)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: RoleModule AccountsRolesLog (r:1 w:0)
	// Storage: NftModule ItemsCount (r:1 w:1)
	// Storage: NftModule Collections (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Uniques Account (r:0 w:1)
	// Storage: NftModule Items (r:0 w:1)
	fn mint() -> Weight {
		Weight::from_ref_time(97_272_000_u64)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Account (r:0 w:2)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	fn transfer() -> Weight {
		Weight::from_ref_time(58_109_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: RoleModule AccountsRolesLog (r:1 w:0)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	// Storage: Uniques ClassMetadataOf (r:0 w:1)
	// Storage: Uniques CollectionMaxSupply (r:0 w:1)
	// Storage: NftModule Collections (r:0 w:1)
	fn destroy_collection() -> Weight {
		Weight::from_ref_time(92_182_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	// Storage: RoleModule AccountsRolesLog (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Uniques Account (r:0 w:1)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	// Storage: NftModule Items (r:0 w:1)
	fn burn() -> Weight {
		Weight::from_ref_time(78_597_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: RoleModule AccountsRolesLog (r:1 w:0)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	// Storage: NftModule Collections (r:0 w:1)
	fn create_collection() -> Weight {
		Weight::from_ref_time(64_040_000_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	// Storage: RoleModule AccountsRolesLog (r:1 w:0)
	// Storage: NftModule ItemsCount (r:1 w:1)
	// Storage: NftModule Collections (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Uniques Account (r:0 w:1)
	// Storage: NftModule Items (r:0 w:1)
	fn mint() -> Weight {
		Weight::from_ref_time(97_272_000_u64)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Account (r:0 w:2)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	fn transfer() -> Weight {
		Weight::from_ref_time(58_109_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	// Storage: RoleModule AccountsRolesLog (r:1 w:0)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	// Storage: Uniques ClassMetadataOf (r:0 w:1)
	// Storage: Uniques CollectionMaxSupply (r:0 w:1)
	// Storage: NftModule Collections (r:0 w:1)
	fn destroy_collection() -> Weight {
		Weight::from_ref_time(92_182_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	// Storage: RoleModule AccountsRolesLog (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Uniques Account (r:0 w:1)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	// Storage: NftModule Items (r:0 w:1)
	fn burn() -> Weight {
		Weight::from_ref_time(78_597_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
}