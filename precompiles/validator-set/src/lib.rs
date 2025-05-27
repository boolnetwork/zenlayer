#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::{GetDispatchInfo, PostDispatchInfo};

use precompile_utils::prelude::*;

use sp_core::H160;
use sp_runtime::traits::Dispatchable;
use sp_std::{marker::PhantomData, vec::Vec};
use pallet_evm::AddressMapping;

/// A precompile to wrap the functionality from pallet_balances
pub struct ValidatorSetPrecompile<Runtime>(PhantomData<Runtime>);

#[precompile_utils::precompile]
impl<Runtime> ValidatorSetPrecompile<Runtime>
where
	Runtime: pallet_utility::Config + pallet_validator_set::Config + pallet_evm::Config + frame_system::Config,
	<Runtime as pallet_utility::Config>::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<<Runtime as pallet_utility::Config>::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<Runtime::AccountId>>,
	<Runtime as frame_system::Config>::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<<Runtime as frame_system::Config>::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<Runtime::AccountId>>,
	<Runtime as pallet_utility::Config>::RuntimeCall: From<pallet_validator_set::Call<Runtime>>,
	<Runtime as frame_system::Config>::RuntimeCall: From<pallet_validator_set::Call<Runtime>>,
	<Runtime as frame_system::Config>::RuntimeCall: From<pallet_utility::Call<Runtime>>,
	Runtime::AccountId: Into<H160>,
{
	// Storage getters

	#[precompile::public("validators()")]
	#[precompile::view]
	fn validators(handle: &mut impl PrecompileHandle) -> EvmResult<Vec<Address>> {
		handle.record_cost(RuntimeHelper::<Runtime>::db_read_gas_cost())?;
		let vs = <pallet_validator_set::Pallet<Runtime>>::validators();

		Ok(vs.into_iter().map(|v| Address(v.into())).collect())
	}

	#[precompile::public("approved_validators()")]
	#[precompile::public("approvedValidators()")]
	#[precompile::view]
	fn approved_validators(handle: &mut impl PrecompileHandle) -> EvmResult<Vec<Address>> {
		handle.record_cost(RuntimeHelper::<Runtime>::db_read_gas_cost())?;
		let approved_vs = <pallet_validator_set::Pallet<Runtime>>::approved_validators();

		Ok(approved_vs.into_iter().map(|v| Address(v.into())).collect())
	}

	// Storage setters

	#[precompile::public("add_validator(address)")]
	#[precompile::public("addValidator(address)")]
	fn add_validator(handle: &mut impl PrecompileHandle, validator_id: Address) -> EvmResult {
		let validator_id = Runtime::AddressMapping::into_account_id(validator_id.0);

		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = pallet_validator_set::Call::<Runtime>::add_validator { validator_id };

		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}

	#[precompile::public("remove_validator(address)")]
	#[precompile::public("removeValidator(address)")]
	fn remove_validator(handle: &mut impl PrecompileHandle, validator_id: Address) -> EvmResult {
		let validator_id = Runtime::AddressMapping::into_account_id(validator_id.0);

		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = pallet_validator_set::Call::<Runtime>::remove_validator { validator_id };

		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}

	#[precompile::public("add_validators(address[])")]
	#[precompile::public("addValidators(address[])")]
	fn add_validators(handle: &mut impl PrecompileHandle, validator_ids: Vec<Address>) -> EvmResult {
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);

		let mut calls = Vec::new();
		for validator_id in validator_ids {
			let id = Runtime::AddressMapping::into_account_id(validator_id.0);
            let call = pallet_validator_set::Call::<Runtime>::add_validator { validator_id: id };
            calls.push(call.into());
        }
		let call = pallet_utility::Call::<Runtime>::batch_all{ calls };
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}
}
