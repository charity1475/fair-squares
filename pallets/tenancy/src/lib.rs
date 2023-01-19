#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet_asset_management as Assets;
pub use pallet_identity as Ident;
pub use pallet_nft as Nft;
pub use pallet_roles as Roles;
pub use pallet_share_distributor as Share;
pub use pallet_payment as Payment;

mod functions;
mod types;
pub use functions::*;
pub use types::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;


	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config:
		frame_system::Config + Assets::Config + Ident::Config + Roles::Config + Nft::Config + Payment::Config
	{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type WeightInfo: WeightInfo;
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
	}

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn infos)]
	/// Stores Tenant informations
	pub type Tenants<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, RegisteredTenant<T>, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Invalid asset id given
		NotAnAsset,
		/// The caller is not a tenant
		NotATenant,
		/// Invalid representative given
		NotARepresentative,
		/// Asset is not linked to the representative
		AssetNotLinked,
		/// The payment request is non-existant
		NotAValidPayment,
		/// The yearly rent has already been paid in full
		NoRentToPay
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn pay_rent(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
			let tenant_account = ensure_signed(origin.clone())?;
			//Check that the Tenant is connected to the asset
			//Check that the remaining rent-to-pay is greater than 1

			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn request_asset(
			origin: OriginFor<T>,
			info: Box<IdentityInfo<T::MaxAdditionalFields>>,
			asset_type: Nft::PossibleCollections,
			asset_id: T::NftItemId,
		) -> DispatchResult {
			let caller = ensure_signed(origin.clone())?;
		// Ensure that the caller has the tenancy role
		ensure!(Roles::TenantLog::<T>::contains_key(caller), Error::<T>::NotATenant);

		// Ensure that the asset is valid
		let collection_id: T::NftCollectionId = asset_type.value().into();
		let ownership = Share::Pallet::<T>::virtual_acc(collection_id, asset_id);
		ensure!(ownership.is_some(), Error::<T>::NotAnAsset);
		let virtual_account = ownership.unwrap().virtual_account;
		Self::request_helper(origin.clone(),virtual_account,info).ok();
		Ok(())

		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
	pub fn pay_guaranty_deposit(
		origin: OriginFor<T>,
		asset_type: Nft::PossibleCollections,
		asset_id: T::NftItemId,
	) -> DispatchResult {
		let caller = ensure_signed(origin.clone())?;
		// Ensure that the caller has the tenancy role
		ensure!(Roles::TenantLog::<T>::contains_key(&caller), Error::<T>::NotATenant);

		// Ensure that the asset is valid
		let collection_id: T::NftCollectionId = asset_type.value().into();
		let ownership = Share::Pallet::<T>::virtual_acc(collection_id, asset_id);
		ensure!(ownership.is_some(), Error::<T>::NotAnAsset);
		let virtual_account = ownership.unwrap().virtual_account;

		//Ensure that payment request exists
		ensure!(!Assets::GuarantyPayment::<T>::contains_key(&caller,&virtual_account),Error::<T>::NotAValidPayment);
		let payment_infos = Assets::Pallet::<T>::guaranty(&caller,&virtual_account).unwrap();
		let status = payment_infos.state;
		ensure!(status == Payment::PaymentState::PaymentRequested,Error::<T>::NotAValidPayment);

		Self::payment_helper(origin,virtual_account,collection_id,asset_id).ok();

		
		

		Ok(())
	}


	}

	
}
