use zenlayer_dev_runtime::{
	opaque::SessionKeys, AccountId, Precompiles, SS58Prefix, WASM_BINARY
};
use zenlayer_dev_constants::currency::UNITS;
use fp_evm::GenesisAccount;
use sc_chain_spec::Properties;
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{ed25519, sr25519, ByteArray, Pair, Public, H160};

use hex_literal::hex;
use std::collections::BTreeMap;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec;
/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str, acc: AccountId) -> (AuraId, GrandpaId, AccountId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s), acc)
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

fn properties() -> Properties {
	let mut properties = Properties::new();
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("tokenSymbol".into(), "ZEN".into());
	properties.insert("ss58Format".into(), SS58Prefix::get().into());
	properties
}

pub fn authority_id_from_pk(
	aura_pk: &str,
	gran_pk: &str,
	acc: AccountId,
) -> (AuraId, GrandpaId, AccountId) {
	(
		sr25519::Public::from_slice(&hex::decode(&aura_pk[2..]).expect("aura pk decode failed"))
			.unwrap()
			.into(),
		ed25519::Public::from_slice(&hex::decode(&gran_pk[2..]).expect("gran pk decode failed"))
			.unwrap()
			.into(),
		acc,
	)
}

fn session_keys(aura: AuraId, grandpa: GrandpaId) -> SessionKeys {
	SessionKeys { aura, grandpa }
}

pub fn development_config() -> ChainSpec {
	ChainSpec::builder(WASM_BINARY.expect("WASM not available"), Default::default())
		.with_name("Zenlayer Dev")
		.with_id("dev")
		.with_chain_type(ChainType::Development)
		.with_properties(properties())
		.with_genesis_config_patch(development_genesis(
			AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")),
				// Pre-funded accounts
				vec![
					AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")), // Alith
					AccountId::from(hex!("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0")), // Baltathar
					AccountId::from(hex!("798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc")), // Charleth
					AccountId::from(hex!("773539d4Ac0e786233D90A233654ccEE26a613D9")), // Dorothy
					AccountId::from(hex!("Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB")), // Ethan
					AccountId::from(hex!("C0F0f4ab324C46e55D02D0033343B4Be8A55532d")), // Faith
				],
				// Initial PoA authorities
				vec![authority_keys_from_seed(
					"Alice",
					AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")),
				)],
				// Ethereum chain ID
				// SS58Prefix::get() as u64,
				17977,
		))
		.build()
}

/// Configure initial storage state for FRAME modules.
fn development_genesis(
	sudo_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	initial_authorities: Vec<(AuraId, GrandpaId, AccountId)>,
	chain_id: u64,
) -> serde_json::Value {
	let revert_bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xFD];

	serde_json::json!({
		"sudo": { "key": Some(sudo_key) },
		"balances": {
			"balances": endowed_accounts.iter().cloned().map(|k| (k, 1_000_000 * UNITS)).collect::<Vec<_>>(),
		},
		"validatorSet": {
			"initialValidators": initial_authorities.iter().map(|x| x.2.clone()).collect::<Vec<_>>(),
		},
		"session": {
			"keys": initial_authorities
				.iter()
				.map(|x| (x.2.clone(), x.2.clone(), session_keys(x.0.clone(), x.1.clone())))
				.collect::<Vec<_>>()
		},
		"evmChainId": { "chainId": chain_id },
		"evm": {
			"accounts":
				// We need _some_ code inserted at the precompile address so that
				// the evm will actually call the address.
				Precompiles::used_addresses()
				.into_iter()
				.map(|addr| {
					(
						addr.into(),
						GenesisAccount {
							nonce: Default::default(),
							balance: Default::default(),
							storage: Default::default(),
							code: revert_bytecode.clone(),
						},
					)
				})
				.collect::<BTreeMap<H160, GenesisAccount>>()
		}
	})
}
