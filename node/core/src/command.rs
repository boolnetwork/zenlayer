use crate::cli::{Cli, Subcommand};

use zenlayer_common_node::cli_opt::{BackendType, BackendTypeConfig, RpcConfig};

use frame_benchmarking_cli::{BenchmarkCmd, SUBSTRATE_REFERENCE_HARDWARE};

use sc_cli::SubstrateCli;
use sc_service::PartialComponents;

trait IdentifyChain {
	fn is_dev(&self) -> bool;
	fn is_testnet(&self) -> bool;
	fn is_mainnet(&self) -> bool;
}

impl IdentifyChain for dyn sc_service::ChainSpec {
	fn is_dev(&self) -> bool {
		self.id().starts_with("dev")
	}
	fn is_testnet(&self) -> bool {
		self.id().starts_with("testnet")
	}
	fn is_mainnet(&self) -> bool {
		self.id().starts_with("mainnet")
	}
}

impl<T: sc_service::ChainSpec + 'static> IdentifyChain for T {
	fn is_dev(&self) -> bool {
		<dyn sc_service::ChainSpec>::is_dev(self)
	}
	fn is_testnet(&self) -> bool {
		<dyn sc_service::ChainSpec>::is_testnet(self)
	}
	fn is_mainnet(&self) -> bool {
		<dyn sc_service::ChainSpec>::is_mainnet(self)
	}
}

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"zenlayer".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"support.anonymous.an".into()
	}

	fn copyright_start_year() -> i32 {
		2024
	}

	fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
		Ok(match id {
			"dev" => Box::new(zenlayer_dev_node::chain_spec::development_config()),
			"testnet_local" => Box::new(zenlayer_testnet_node::chain_spec::testnet_config()),
			"testnet" => Box::new(zenlayer_testnet_node::chain_spec::ChainSpec::from_json_file(
				std::path::PathBuf::from("./specs/zenlayer-testnet.json"),
			)?),
			"mainnet_local" => Box::new(zenlayer_mainnet_node::chain_spec::mainnet_config()),
			"mainnet" => Box::new(zenlayer_mainnet_node::chain_spec::ChainSpec::from_json_file(
				std::path::PathBuf::from("./specs/zenlayer-mainnet.json"),
			)?),
			path => Box::new(zenlayer_dev_node::chain_spec::ChainSpec::from_json_file(
				std::path::PathBuf::from(path),
			)?),
		})
	}
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
	let cli = Cli::from_args();

	let rpc_config = RpcConfig {
		ethapi: cli.ethapi.clone(),
		ethapi_max_permits: cli.ethapi_max_permits,
		ethapi_trace_max_count: cli.ethapi_trace_max_count,
		ethapi_trace_cache_duration: cli.ethapi_trace_cache_duration,
		eth_log_block_cache: cli.eth_log_block_cache,
		eth_statuses_cache: cli.eth_statuses_cache,
		fee_history_limit: cli.fee_history_limit,
		max_past_logs: cli.max_past_logs,
		logs_request_timeout: cli.logs_request_timeout,
		tracing_raw_max_memory_usage: cli.tracing_raw_max_memory_usage,
		frontier_backend_type: match cli.frontier_backend_type {
			BackendType::KeyValue => BackendTypeConfig::KeyValue,
			BackendType::Sql => BackendTypeConfig::Sql {
				pool_size: cli.frontier_sql_backend_pool_size,
				num_ops_timeout: cli.frontier_sql_backend_num_ops_timeout,
				thread_count: cli.frontier_sql_backend_thread_count,
				cache_size: cli.frontier_sql_backend_cache_size,
			},
		},
	};

	match &cli.subcommand {
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			if chain_spec.is_dev() {
				runner.async_run(|config| {
					let PartialComponents { client, task_manager, import_queue, .. } =
						zenlayer_dev_node::service::new_partial(&config, &rpc_config)?;
					Ok((cmd.run(client, import_queue), task_manager))
				})
			} else if chain_spec.is_testnet() {
				runner.async_run(|config| {
					let PartialComponents { client, task_manager, import_queue, .. } =
						zenlayer_testnet_node::service::new_partial(&config, &rpc_config)?;
					Ok((cmd.run(client, import_queue), task_manager))
				})
			} else if chain_spec.is_mainnet() {
				runner.async_run(|config| {
					let PartialComponents { client, task_manager, import_queue, .. } =
						zenlayer_mainnet_node::service::new_partial(&config, &rpc_config)?;
					Ok((cmd.run(client, import_queue), task_manager))
				})
			} else {
				runner.async_run(|config| {
					let PartialComponents { client, task_manager, import_queue, .. } =
						zenlayer_dev_node::service::new_partial(&config, &rpc_config)?;
					Ok((cmd.run(client, import_queue), task_manager))
				})
			}
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			if chain_spec.is_dev() {
				runner.async_run(|config| {
					let PartialComponents { client, task_manager, .. } =
						zenlayer_dev_node::service::new_partial(&config, &rpc_config)?;
					Ok((cmd.run(client, config.database), task_manager))
				})
			} else if chain_spec.is_testnet() {
				runner.async_run(|config| {
					let PartialComponents { client, task_manager, .. } =
						zenlayer_testnet_node::service::new_partial(&config, &rpc_config)?;
					Ok((cmd.run(client, config.database), task_manager))
				})
			} else {
				runner.async_run(|config| {
					let PartialComponents { client, task_manager, .. } =
						zenlayer_dev_node::service::new_partial(&config, &rpc_config)?;
					Ok((cmd.run(client, config.database), task_manager))
				})
			}
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			if chain_spec.is_dev() {
				runner.async_run(|config| {
					let PartialComponents { client, task_manager, .. } =
						zenlayer_dev_node::service::new_partial(&config, &rpc_config)?;
					Ok((cmd.run(client, config.chain_spec), task_manager))
				})
			} else if chain_spec.is_testnet() {
				runner.async_run(|config| {
					let PartialComponents { client, task_manager, .. } =
						zenlayer_testnet_node::service::new_partial(&config, &rpc_config)?;
					Ok((cmd.run(client, config.chain_spec), task_manager))
				})
			} else if chain_spec.is_mainnet() {
				runner.async_run(|config| {
					let PartialComponents { client, task_manager, .. } =
						zenlayer_mainnet_node::service::new_partial(&config, &rpc_config)?;
					Ok((cmd.run(client, config.chain_spec), task_manager))
				})
			} else {
				runner.async_run(|config| {
					let PartialComponents { client, task_manager, .. } =
						zenlayer_dev_node::service::new_partial(&config, &rpc_config)?;
					Ok((cmd.run(client, config.chain_spec), task_manager))
				})
			}
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			if chain_spec.is_dev() {
				runner.async_run(|config| {
					let PartialComponents { client, task_manager, import_queue, .. } =
						zenlayer_dev_node::service::new_partial(&config, &rpc_config)?;
					Ok((cmd.run(client, import_queue), task_manager))
				})
			} else if chain_spec.is_testnet() {
				runner.async_run(|config| {
					let PartialComponents { client, task_manager, import_queue, .. } =
						zenlayer_testnet_node::service::new_partial(&config, &rpc_config)?;
					Ok((cmd.run(client, import_queue), task_manager))
				})
			} else if chain_spec.is_mainnet() {
				runner.async_run(|config| {
					let PartialComponents { client, task_manager, import_queue, .. } =
						zenlayer_mainnet_node::service::new_partial(&config, &rpc_config)?;
					Ok((cmd.run(client, import_queue), task_manager))
				})
			} else {
				runner.async_run(|config| {
					let PartialComponents { client, task_manager, import_queue, .. } =
						zenlayer_dev_node::service::new_partial(&config, &rpc_config)?;
					Ok((cmd.run(client, import_queue), task_manager))
				})
			}
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.database))
		},
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			if chain_spec.is_dev() {
				runner.async_run(|config| {
					let PartialComponents { client, task_manager, backend, .. } =
						zenlayer_dev_node::service::new_partial(&config, &rpc_config)?;
					let aux_revert = Box::new(|client, _, blocks| {
						sc_consensus_grandpa::revert(client, blocks)?;
						Ok(())
					});
					Ok((cmd.run(client, backend, Some(aux_revert)), task_manager))
				})
			} else if chain_spec.is_testnet() {
				runner.async_run(|config| {
					let PartialComponents { client, task_manager, backend, .. } =
						zenlayer_testnet_node::service::new_partial(&config, &rpc_config)?;
					let aux_revert = Box::new(|client, _, blocks| {
						sc_consensus_grandpa::revert(client, blocks)?;
						Ok(())
					});
					Ok((cmd.run(client, backend, Some(aux_revert)), task_manager))
				})
			} else if chain_spec.is_mainnet() {
				runner.async_run(|config| {
					let PartialComponents { client, task_manager, backend, .. } =
						zenlayer_mainnet_node::service::new_partial(&config, &rpc_config)?;
					let aux_revert = Box::new(|client, _, blocks| {
						sc_consensus_grandpa::revert(client, blocks)?;
						Ok(())
					});
					Ok((cmd.run(client, backend, Some(aux_revert)), task_manager))
				})
			} else {
				runner.async_run(|config| {
					let PartialComponents { client, task_manager, backend, .. } =
						zenlayer_dev_node::service::new_partial(&config, &rpc_config)?;
					let aux_revert = Box::new(|client, _, blocks| {
						sc_consensus_grandpa::revert(client, blocks)?;
						Ok(())
					});
					Ok((cmd.run(client, backend, Some(aux_revert)), task_manager))
				})
			}
		},
		Some(Subcommand::Benchmark(cmd)) => {
			use zenlayer_common_node::rpc::Block;

			let runner = cli.create_runner(cmd)?;

			runner.sync_run(|config| {
				// This switch needs to be in the client, since the client decides
				// which sub-commands it wants to support.
				match cmd {
					BenchmarkCmd::Pallet(cmd) => {
						if !cfg!(feature = "runtime-benchmarks") {
							return Err(
								"Runtime benchmarking wasn't enabled when building the node. \
							You can enable it with `--features runtime-benchmarks`."
									.into(),
							);
						}

						cmd.run_with_spec::<sp_runtime::traits::HashingFor<Block>, ()>(Some(
							config.chain_spec,
						))
					},
					BenchmarkCmd::Block(cmd) => {
						let PartialComponents { client, .. } =
							zenlayer_dev_node::service::new_partial(&config, &rpc_config)?;
						cmd.run(client)
					},
					#[cfg(not(feature = "runtime-benchmarks"))]
					BenchmarkCmd::Storage(_) => Err(
						"Storage benchmarking can be enabled with `--features runtime-benchmarks`."
							.into(),
					),
					#[cfg(feature = "runtime-benchmarks")]
					BenchmarkCmd::Storage(cmd) => {
						let PartialComponents { client, backend, .. } =
							service::new_partial(&config)?;
						let db = backend.expose_db();
						let storage = backend.expose_storage();

						cmd.run(config, client, db, storage)
					},
					BenchmarkCmd::Machine(cmd) => {
						cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone())
					},
					_ => {
						return Err("Runtime benchmarking wasn't enabled when building the node. \
					You can enable it with `--features runtime-benchmarks`."
							.into())
					},
				}
			})
		},
		None => {
			let runner = cli.create_runner(&cli.run)?;
			let chain_spec = &runner.config().chain_spec;

			if chain_spec.is_dev() {
				if cli.sealing {
					runner.run_node_until_exit(|config| async move {
						zenlayer_dev_node::service::new_manual(config, rpc_config)
							.map_err(sc_cli::Error::Service)
					})
				} else {
					runner.run_node_until_exit(|config| async move {
						zenlayer_dev_node::service::new_full(config, rpc_config)
							.map_err(sc_cli::Error::Service)
					})
				}
			} else if chain_spec.is_testnet() {
				runner.run_node_until_exit(|config| async move {
					zenlayer_testnet_node::service::new_full(config, rpc_config)
						.map_err(sc_cli::Error::Service)
				})
			} else if chain_spec.is_mainnet() {
				runner.run_node_until_exit(|config| async move {
					zenlayer_mainnet_node::service::new_full(config, rpc_config)
						.map_err(sc_cli::Error::Service)
				})
			} else {
				runner.run_node_until_exit(|config| async move {
					zenlayer_dev_node::service::new_full(config, rpc_config)
						.map_err(sc_cli::Error::Service)
				})
			}
		},
	}
}
