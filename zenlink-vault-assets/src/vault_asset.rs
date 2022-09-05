use super::*;

/// Interface of the ERC4626 "Tokenized Vault Standard", as defined in
/// * https://eips.ethereum.org/EIPS/eip-4626[ERC-4626].
pub trait VaultAsset<T: Config> {
	/// Returns the id of the underlying token used for the Vault.
	fn asset(asset_id: T::AssetId) -> Result<T::AssetId, DispatchError>;

	/// Returns the total amount of the underlying asset that is “managed” by Vault.
	fn total_assets(asset_id: T::AssetId) -> Result<Balance, DispatchError>;

	/// Returns the amount of shares that the Vault would exchange for the amount of assets
	/// provided.
	fn convert_to_shares(asset_id: T::AssetId, amounts: Balance) -> Result<Balance, DispatchError>;

	/// Returns the amount of assets that the Vault would exchange for the amount of shares
	/// provided.
	fn convert_to_assets(asset_id: T::AssetId, shares: Balance) -> Result<Balance, DispatchError>;

	/// Returns the maximum amount of the underlying asset that can be deposited into the Vault for
	/// the receiver.
	fn max_deposit(asset_id: T::AssetId, receiver: &T::AccountId) -> Result<Balance, DispatchError>;

	/// Allows an on-chain or off-chain user to simulate the effects of their deposit at the current
	/// block.
	fn preview_deposit(asset_id: T::AssetId, amounts: Balance) -> Result<Balance, DispatchError>;

	/// Returns the maximum amount of the Vault shares that can be minted for the receiver, through
	/// a mint call.
	fn max_mint(asset_id: T::AssetId, receiver: &T::AccountId) -> Balance;

	/// Allows an on-chain or off-chain user to simulate the effects of their mint at the current
	/// block, given current on-chain conditions.
	fn preview_mint(asset_id: T::AssetId, shares: Balance) -> Result<Balance, DispatchError>;

	/// Returns the maximum amount of the underlying asset that can be withdrawn from the owner
	/// balance in the Vault, through a withdraw call.
	fn max_withdraw(asset_id: T::AssetId, owner: &T::AccountId) -> Result<Balance, DispatchError>;

	/// Allows an on-chain or off-chain user to simulate the effects of their withdrawal at the
	/// current block, given current on-chain conditions.
	fn preview_withdraw(asset_id: T::AssetId, amounts: Balance) -> Result<Balance, DispatchError>;

	/// Returns the maximum amount of Vault shares that can be redeemed from the owner balance in
	/// the Vault, through a redeem call.
	fn max_redeem(asset_id: T::AssetId, owner: &T::AccountId) -> Balance;

	/// Allows an on-chain or off-chain user to simulate the effects of their redeemption at the
	/// current block, given current on-chain conditions.
	fn preview_redeem(asset_id: T::AssetId, shares: Balance) -> Result<Balance, DispatchError>;

	/// Mints shares Vault shares to receiver by depositing exactly amount of underlying tokens.
	fn deposit(
		who: &T::AccountId,
		asset_id: T::AssetId,
		amounts: Balance,
		to: &T::AccountId,
	) -> Result<Balance, DispatchError>;

	/// Mints exactly shares Vault shares to receiver by depositing amount of underlying tokens.
	fn mint(
		who: &T::AccountId,
		asset_id: T::AssetId,
		shares: Balance,
		to: &T::AccountId,
	) -> Result<Balance, DispatchError>;

	/// Burns shares from owner and sends exactly assets of underlying tokens to receiver.
	fn withdraw(
		who: &T::AccountId,
		asset_id: T::AssetId,
		amounts: Balance,
		to: &T::AccountId,
		owner: &T::AccountId,
	) -> Result<Balance, DispatchError>;

	/// Burns exactly shares from owner and sends assets of underlying tokens to receiver.
	fn redeem(
		who: &T::AccountId,
		asset_id: T::AssetId,
		shares: Balance,
		to: &T::AccountId,
		owner: &T::AccountId,
	) -> Result<Balance, DispatchError>;
}
