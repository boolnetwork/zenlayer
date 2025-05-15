use std::collections::BTreeMap;
use fp_evm::GenesisAccount;
use hex_literal::hex;
use sc_chain_spec::Properties;
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{ed25519, sr25519, ByteArray, Pair, Public, H160};
use zenlayer_testnet_runtime::{
	opaque::SessionKeys, AccountId, Balance, Precompiles, SS58Prefix, WASM_BINARY,
};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const ZEN: Balance = 1_000_000_000_000_000_000;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec;

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
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

pub fn testnet_config() -> ChainSpec {
	let faucet_amount = 100_000 * ZEN;
	let total_issuance = 1_000_000 * ZEN;

	ChainSpec::builder(WASM_BINARY.expect("WASM not available"), Default::default())
		.with_name("Zenlayer Testnet")
		.with_id("Testnet")
		.with_protocol_id("Zenlayer Testnet")
		.with_chain_type(ChainType::Local)
		.with_properties(properties())
		.with_genesis_config_patch(testnet_genesis(
			// Sudo account Key: 0xf5d3fd1138b192d13d189a70f9a83ca8ef5f00f7550e6d5c6a66a1e6cb96e28a
			AccountId::from(hex!("c8137e6424885651c9416c051098478bf2b003dd")),
			// Pre-funded accounts
			vec![
				(
					AccountId::from(hex!("c8137e6424885651c9416c051098478bf2b003dd")),
					total_issuance - faucet_amount,
				), //sudo account,
				(AccountId::from(hex!("74b89c0121870c9e75e1bdc098c0ea9db36df3fb")), faucet_amount), //official account Key: 0xdb9e07e4f35b1d6dd504878db31147829baffb6bf8edefc038f953aca8b909a9
			],
			// Initial PoA authorities
			vec![
				// aura Key: 0x19b8720a21158be6333aab6bbf64b23876b1259e158755c88009edf1381a49fa
				// gran Key: 0xada3b9b4ed2f2532850072056c2d891bd160d4d107451052d00b7e5bf339fac5
				// Account Key: 0x8236c9c442364584be1a75915f6da295649bf2a877252e5c1debab4a686f4575
				authority_id_from_pk(
					"0xd85d1a3e8b82ae2f9dae87d707a29927556bfb115c3c2447e81f189b18ca7641",
					"0x2730419ee3b96fe8c0183812618b1e6ba307e214357296d0f3093bc9b8eea882",
					AccountId::from(hex!("a44bf90602708ec5fc9d984dcdf9d43539bd68c1")),
				),
				// aura Key: 0x14fa9e0103a854102fa45feebf643df5d93942e4946f1ee50358d17e3ecc889d
				// gran Key: 0xce9c5e04ceea64d9c82e123e1f7feeea5ac79420e3306f052f6a6fcb7bacae44
				// Account Key: 0x591a9899803708c40e6bf69bc7f14ab8412865642ff786ac5f1f3d6a4569386a
				authority_id_from_pk(
					"0xfafaa9ad6290225a21d8a84bb8fca99ba8347ae3758f4ecfa4fe453103137f3b",
					"0xa19d3fcda429cad5add5eb939c474d8c8080f6a78f0d32fd926040d331dbf81b",
					AccountId::from(hex!("b06b13fa8345b59707624ff0f68a5ed6f60a4c59")),
				),
				// aura Key: 0x0e397595fd3b346136439dccb061890a1328557dccafde1095c3c9374f7f1892
				// gran Key: 0x0f175cbd49ba512094fa901c305e1343ad6c7858d2c7753b84bb3e4e1f52574a
				// Account Key: 0xd0dbc9d982d19039daf8cb7dcbcbdb5bc2fadc9ec6b491d7b51130d81d65c73f
				authority_id_from_pk(
					"0x28c0a26d79ec0fe47dc7aa6001c33a877b9cad6779953912d164353a5611d342",
					"0x5a8c4ca176d24805b258a8b491289a02bb3ac9a9284e7358096e8cf12e7a87a7",
					AccountId::from(hex!("1d9094591c98a0d95e83d7dea29fdc67dcc38fdf")),
				),
			],
			17978,
		))
		.build()
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	sudo_key: AccountId,
	endowed_accounts: Vec<(AccountId, Balance)>,
	initial_authorities: Vec<(AuraId, GrandpaId, AccountId)>,
	chain_id: u64,
) -> serde_json::Value {
	// (PUSH1 0x00 PUSH1 0x00 REVERT)
	let revert_bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xFD];

	serde_json::json!({
		"sudo": {
			// Assign network admin rights.
			"key": Some(sudo_key),
		},
		// Monetary
		"balances": { "balances": endowed_accounts },
		// Consensus
		"validatorSet": {
			"initialValidators": initial_authorities.iter().map(|x| x.2.clone()).collect::<Vec<_>>(),
		},
		"session": {
			"keys": initial_authorities
				.iter()
				.map(|x| (x.2.clone(), x.2.clone(), session_keys(x.0.clone(), x.1.clone())))
				.collect::<Vec<_>>(),
		},
		// EVM compatibility
		"evmChainId": { "chainId": chain_id },
		"evm": {
			// We need _some_ code inserted at the precompile address so that
			// the evm will actually call the address.
			"accounts": Precompiles::used_addresses()
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
