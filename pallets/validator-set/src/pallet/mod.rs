mod impls;

use crate::{LOG_TARGET, weights::WeightInfo};
use frame_support::{ensure, pallet_prelude::*, traits::Get, BoundedVec, DefaultNoBound};
use frame_system::pallet_prelude::BlockNumberFor;
use log;
pub use pallet::*;
use sp_std::{collections::btree_set::BTreeSet, prelude::*};

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it
	/// depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_session::Config {
		/// The Event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// RuntimeOrigin for adding or removing a validator.
		type AddRemoveOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Minimum number of validators to leave in the validator set during
		/// auto removal.
		type MinAuthorities: Get<u32>;

		type MaxAuthorities: Get<u32>;

		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn validators)]
	pub type Validators<T: Config> =
		StorageValue<_, BoundedVec<T::AccountId, T::MaxAuthorities>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn approved_validators)]
	pub type ApprovedValidators<T: Config> =
		StorageValue<_, BoundedVec<T::AccountId, T::MaxAuthorities>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn validators_to_remove)]
	pub type OfflineValidators<T: Config> =
		StorageValue<_, BoundedVec<T::AccountId, T::MaxAuthorities>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New validator addition initiated. Effective in ~2 sessions.
		ValidatorAdditionInitiated(T::AccountId),

		/// Validator removal initiated. Effective in ~2 sessions.
		ValidatorRemovalInitiated(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Target (post-removal) validator count is below the minimum.
		TooLowValidatorCount,
		/// Validator is already in the validator set.
		Duplicate,
		/// Validator is not approved for re-addition.
		ValidatorNotApproved,
		/// Only the validator can add itself back after coming online.
		BadOrigin,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::genesis_config]
	#[derive(DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub initial_validators: Vec<T::AccountId>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			log::debug!(target: LOG_TARGET, "Validator initiated.");

			Pallet::<T>::initialize_validators(&self.initial_validators);
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Add a new validator.
		///
		/// New validator's session keys should be set in Session pallet before
		/// calling this.
		///
		/// The origin can be configured using the `AddRemoveOrigin` type in the
		/// host runtime. Can also be set to sudo/root.
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::add_validator())]
		pub fn add_validator(origin: OriginFor<T>, validator_id: T::AccountId) -> DispatchResult {
			T::AddRemoveOrigin::ensure_origin(origin)?;

			Self::do_add_validator(validator_id.clone())?;
			Self::approve_validator(validator_id)?;

			Ok(())
		}

		/// Remove a validator.
		///
		/// The origin can be configured using the `AddRemoveOrigin` type in the
		/// host runtime. Can also be set to sudo/root.
		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::remove_validator())]
		pub fn remove_validator(
			origin: OriginFor<T>,
			validator_id: T::AccountId,
		) -> DispatchResult {
			T::AddRemoveOrigin::ensure_origin(origin)?;

			Self::do_remove_validator(validator_id.clone())?;
			Self::unapprove_validator(validator_id)?;

			Ok(())
		}

		/// Add an approved validator again when it comes back online.
		///
		/// For this call, the dispatch origin must be the validator itself.
		#[pallet::call_index(2)]
		#[pallet::weight(Weight::from_parts(0, 0))]
		pub fn add_validator_again(
			origin: OriginFor<T>,
			validator_id: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(who == validator_id, Error::<T>::BadOrigin);

			let approved_set: BTreeSet<_> = <ApprovedValidators<T>>::get().into_iter().collect();
			ensure!(approved_set.contains(&validator_id), Error::<T>::ValidatorNotApproved);

			Self::do_add_validator(validator_id)?;

			Ok(())
		}
	}
}
