use node_template_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig,
	SystemConfig, WASM_BINARY,
	// NodeAuthorizationConfig,
	ValidatorSetConfig, //增加动态的validator
	SessionConfig,
	opaque::SessionKeys,

};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::UncheckedInto,sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_core::OpaquePeerId; // A struct wraps Vec<u8>, represents as our `PeerId`.

use hex_literal::hex;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// 改为由下面的函数提供
/*pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}*/

/// Generate an Aura GrandpaId authority key.
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AuraId, GrandpaId) {
	(
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s)
	)
}

fn session_keys(
	aura: AuraId,
	grandpa: GrandpaId,
) -> SessionKeys {
	SessionKeys { aura, grandpa }
}


/// --dev 命令行用这个，会执行这个函数
pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")
				],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

/// --chain=local 命令用这个，会用到这个
pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					// (
					// 	// refs: https://github.com/starit/sharing-docs/wiki/Substrate-%E5%B8%B8%E8%A7%81%E8%BF%90%E7%BB%B4%E6%93%8D%E4%BD%9C#%E6%95%B0%E6%8D%AE%E5%90%8C%E6%AD%A5%E8%8A%82%E7%82%B9%E7%9A%84%E5%8F%82%E7%85%A7%E5%91%BD%E4%BB%A4
					//
					// 	//auro: sr25519 ss58 address: 5EcHTMuipQV9wRFTYZMV81g9Xk7LfcFfaCkCs8dNnfA7f4nw.  Public key (hex):
					// 	hex!["708e2d070222049c6684ce608845d6c28168aa45946593d9a0ce69d3d6c9be56"].unchecked_into(),
					// 	//gran: ed25519 ss58 address:5FKFgvcLx8vmzwDCDbUDDAddsMWNFB3ad6GQ1ehwUgbGrCkU.  Public key (hex):
					// 	hex!["8fcd4ac76751ca962f1fe92713c12710f4bdf9c23100baa0030b463aa6218fb8"].unchecked_into(),
					// ),
					// (
					// 	hex!["72b8edbe194dfa3973bf16c2f67de0d369f706a50ce75598ce719a23709ec761"].unchecked_into(),
					// 	hex!["f3adf3464c7f62f7d5f546ad7d4ec878451b62e239fad3833feac6cf12559eff"].unchecked_into(),
					// )
					authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")
				],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("ddl"),
		// Properties
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		//顺序很重要，在BalancesConfig 之后，
		validator_set: ValidatorSetConfig {
			validators: initial_authorities.iter().map(|x| x.0.clone()).collect::<Vec<_>>(),
		},
		session: SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.0.clone(), session_keys(x.1.clone(), x.2.clone()))
			}).collect::<Vec<_>>(),
		},
		// validator_set 和 session 在 aura、grandpa之前
		aura: AuraConfig {
			//authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
			authorities: vec![], // 改为由 session里提供
		},
		grandpa: GrandpaConfig {
			//authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
			authorities: vec![], // 改为由 session里提供
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: root_key,
		},
		// node_authorization: NodeAuthorizationConfig {
		// 	nodes: vec![
		// 		(
		// 			// 因为 alice 已经配置为auro和gran的key，这里用 subkey generate-node-key 生成一个node key做Alice的节点key。
		// 			// endowed_accounts[0] 是 Alice， endowed_accounts[1]是 Bob
		// 			// 213216127a8a8756f4017d2aaafa7e0054e9958e7ded5d5784c5e2f6f6365e0f node-key, peer id:
		// 			OpaquePeerId(bs58::decode("12D3KooWFzXtYJhUkMsWTXQodYaXhs6ah52xVExicuFPvmUQoZrE").into_vec().unwrap()),
		// 			endowed_accounts[0].clone()
		// 		),
		// 		(
		// 			// 3b7a5239d28e90a6941dba9266b8f7135b966885e21988fb0c6e7f55516c73f3 node-key, peer id:
		// 			OpaquePeerId(bs58::decode("12D3KooWFqHLgcSSB6dgAZ3SeCFYR4Dpr5w8TEhTdq8JKDxTUvGH").into_vec().unwrap()),
		// 			endowed_accounts[1].clone()
		// 		),
		// 	],
		// },
	}
}
