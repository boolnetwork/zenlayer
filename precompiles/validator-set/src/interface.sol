// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.0;

/**
 * @title The interface through which solidity contracts will interact with pallet_validator_set
 * We follow this same interface including four-byte function selectors, in the precompile that
 * wraps the pallet
 * Address :    0x0000000000000000000000000000000000010000
 */

interface ValidatorSet {

    function validators() external view returns (address[] memory);
    function approved_validators() external view returns (address[] memory);

    function add_validator(address validator_id) external;
    function remove_validator(address validator_id) external;
}
