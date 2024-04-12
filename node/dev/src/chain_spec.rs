use fp_evm::GenesisAccount;
use hex_literal::hex;
use sc_chain_spec::Properties;
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{ed25519, sr25519, ByteArray, Pair, Public};
use zenlayer_dev_runtime::{
	opaque::SessionKeys, AccountId, Balance, GenesisConfig, Precompiles, SS58Prefix, WASM_BINARY,
};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const ZEN: Balance = 1_000_000_000_000_000_000;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

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
	let wasm_binary = WASM_BINARY.expect("WASM not available");

	ChainSpec::from_genesis(
		// Name
		"Zenlayer Dev",
		// ID
		"dev",
		ChainType::Development,
		move || {
			dev_genesis(
				wasm_binary,
				// Sudo account (Alith)
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
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Fork ID
		None,
		// Properties
		Some(properties()),
		// Extensions
		None,
	)
}

/// Configure initial storage state for FRAME modules.
fn dev_genesis(
	wasm_binary: &[u8],
	sudo_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	initial_authorities: Vec<(AuraId, GrandpaId, AccountId)>,
	chain_id: u64,
) -> GenesisConfig {
	use zenlayer_dev_runtime::{
		AuraConfig, BalancesConfig, EVMChainIdConfig, EVMConfig, GrandpaConfig, SessionConfig,
		SudoConfig, SystemConfig, ValidatorSetConfig,
	};
	// (PUSH1 0x00 PUSH1 0x00 REVERT)
	let revert_bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xFD];

	GenesisConfig {
		// System
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			..Default::default()
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(sudo_key),
		},

		// Monetary
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1_000_000 * ZEN)).collect(),
		},
		transaction_payment: Default::default(),

		// Consensus
		aura: AuraConfig { authorities: vec![] },
		validator_set: ValidatorSetConfig {
			initial_validators: initial_authorities.iter().map(|x| x.2.clone()).collect::<Vec<_>>(),
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| (x.2.clone(), x.2.clone(), session_keys(x.0.clone(), x.1.clone())))
				.collect::<Vec<_>>(),
		},
		grandpa: GrandpaConfig { authorities: vec![], ..Default::default() },

		// EVM compatibility
		evm_chain_id: EVMChainIdConfig { chain_id, ..Default::default() },
		evm: EVMConfig {
			// We need _some_ code inserted at the precompile address so that
			// the evm will actually call the address.
			accounts: Precompiles::used_addresses()
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
				.collect(),
			..Default::default()
		},
		ethereum: Default::default(),
		dynamic_fee: Default::default(),
		base_fee: Default::default(),
	}
}
