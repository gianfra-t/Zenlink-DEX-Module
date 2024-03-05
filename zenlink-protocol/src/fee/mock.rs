// Copyright 2021-2022 Zenlink.
// Licensed under Apache 2.0.

//! Test utilities

use std::marker::PhantomData;

use crate as pallet_zenlink;
pub use crate::{
	AssetBalance, AssetId, Config, LocalAssetHandler, MultiAssetsHandler, PairLpGenerate, Pallet,
	ParaId, ZenlinkMultiAssets, LIQUIDITY, LOCAL, NATIVE, RESERVED,
};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	derive_impl, dispatch::DispatchResult, parameter_types, traits::Contains, PalletId,
};
use orml_traits::{parameter_type_with_key, MultiCurrency};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_runtime::{BuildStorage, DispatchError, RuntimeDebug};
type Block = frame_system::mocking::MockBlock<Test>;

#[derive(
	Encode,
	Decode,
	Eq,
	PartialEq,
	Copy,
	Clone,
	RuntimeDebug,
	MaxEncodedLen,
	PartialOrd,
	Ord,
	TypeInfo,
	Deserialize,
	Serialize,
)]
pub enum CurrencyId {
	Token(u8),
	ZenlinkLp(u8, u8),
}

frame_support::construct_runtime!(
	pub enum Test {
		System: frame_system = 0,
		Balances: pallet_balances = 8,
		Zenlink: pallet_zenlink = 9,
		Tokens: orml_tokens = 11,
	}
);

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;

	pub const BlockHashCount: u64 = 250;
	pub const ZenlinkPalletId: PalletId = PalletId(*b"/zenlink");
	pub const MaxReserves: u32 = 50;
	pub const MaxLocks:u32 = 50;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type AccountId = AccountId;
	type Lookup = sp_runtime::traits::IdentityLookup<Self::AccountId>;
	type Block = Block;
	type AccountData = pallet_balances::AccountData<u128>;
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> u128 {
		0
	};
}

pub struct MockDustRemovalWhitelist;
impl Contains<AccountId> for MockDustRemovalWhitelist {
	fn contains(_a: &AccountId) -> bool {
		true
	}
}

impl orml_tokens::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = u128;
	type Amount = i128;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type MaxLocks = ();
	type DustRemovalWhitelist = MockDustRemovalWhitelist;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type CurrencyHooks = ();
}

impl pallet_balances::Config for Test {
	type Balance = u128;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = frame_system::Pallet<Test>;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
	type FreezeIdentifier = ();
	type MaxHolds = ();
	type MaxFreezes = ();
}

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MultiAssetsHandler = ZenlinkMultiAssets<Zenlink, Balances, LocalAssetAdaptor<Tokens>>;
	type PalletId = ZenlinkPalletId;
	type AssetId = AssetId;
	type LpGenerate = PairLpGenerate<Self>;
	type TargetChains = ();
	type SelfParaId = ();
	type WeightInfo = ();
}

pub type DexPallet = Pallet<Test>;

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(1, 34028236692093846346337460743176821145),
			(2, 10),
			(3, 10),
			(4, 10),
			(5, 10),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	pallet_zenlink::GenesisConfig::<Test> { fee_receiver: None, fee_point: 5 }
		.assimilate_storage(&mut t)
		.unwrap();

	orml_tokens::GenesisConfig::<Test> {
		balances: vec![
			(1, CurrencyId::Token(1), 34028236692093846346337460743176821145),
			(1, CurrencyId::Token(2), 34028236692093846346337460743176821145),
			(1, CurrencyId::Token(3), 34028236692093846346337460743176821145),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	t.into()
}

pub struct LocalAssetAdaptor<Local>(PhantomData<Local>);

type AccountId = u128;

fn asset_id_to_currency_id(asset_id: &AssetId) -> Result<CurrencyId, ()> {
	if asset_id.asset_type == LIQUIDITY {
		let token0_id = ((asset_id.asset_index & 0xFFFF0000) >> 16) as u8;
		let token1_id = (asset_id.asset_index & 0x0000FFFF) as u8;
		return Ok(CurrencyId::ZenlinkLp(token0_id, token1_id))
	}
	if asset_id.asset_type == LOCAL {
		let token_id = asset_id.asset_index as u8;
		return Ok(CurrencyId::Token(token_id))
	}
	Err(())
}

impl<Local> LocalAssetHandler<AccountId> for LocalAssetAdaptor<Local>
where
	Local: MultiCurrency<AccountId, Balance = u128, CurrencyId = CurrencyId>,
{
	fn local_balance_of(asset_id: AssetId, who: &AccountId) -> AssetBalance {
		asset_id_to_currency_id(&asset_id)
			.map_or(AssetBalance::default(), |currency_id| Local::free_balance(currency_id, who))
	}

	fn local_total_supply(asset_id: AssetId) -> AssetBalance {
		asset_id_to_currency_id(&asset_id)
			.map_or(AssetBalance::default(), |currency_id| Local::total_issuance(currency_id))
	}

	fn local_is_exists(asset_id: AssetId) -> bool {
		asset_id_to_currency_id(&asset_id).map_or(false, |currency_id| {
			Local::total_issuance(currency_id) > AssetBalance::default()
		})
	}

	fn local_transfer(
		asset_id: AssetId,
		origin: &AccountId,
		target: &AccountId,
		amount: AssetBalance,
	) -> DispatchResult {
		asset_id_to_currency_id(&asset_id).map_or(Err(DispatchError::CannotLookup), |currency_id| {
			Local::transfer(currency_id, origin, target, amount)
		})
	}

	fn local_deposit(
		asset_id: AssetId,
		origin: &AccountId,
		amount: AssetBalance,
	) -> Result<AssetBalance, DispatchError> {
		asset_id_to_currency_id(&asset_id).map_or(Ok(AssetBalance::default()), |currency_id| {
			Local::deposit(currency_id, origin, amount).map(|_| amount)
		})
	}

	fn local_withdraw(
		asset_id: AssetId,
		origin: &AccountId,
		amount: AssetBalance,
	) -> Result<AssetBalance, DispatchError> {
		asset_id_to_currency_id(&asset_id).map_or(Ok(AssetBalance::default()), |currency_id| {
			Local::withdraw(currency_id, origin, amount).map(|_| amount)
		})
	}
}
