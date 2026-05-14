mod config;
mod demo;
mod escrow;
mod market;
mod snn;

use anyhow::Result;
use clap::Parser;
use reqwest::Client;
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signer::Signer;
use std::time::Duration;
use tracing::{error, info, warn};

#[derive(Parser, Debug)]
#[command(name = "neuroforge", about = "Neuromorphic autonomous SAP agent")]
struct Cli {
    #[arg(long)]
    demo: bool,
    #[arg(long)]
    once: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("neuroforge_agent=info".parse()?),
        )
        .init();

    let cli = Cli::parse();

    if cli.demo {
        return demo::run().await;
    }

    let config = config::Config::from_env()?;
    let payer = config.load_keypair()?;
    info!(wallet = %payer.pubkey(), "NeuroForge starting");

    let rpc = RpcClient::new_with_commitment(
        config.synapse_rpc_url.clone(),
        CommitmentConfig::confirmed(),
    );
    let http = Client::new();

    let mut net = snn::SpikingNetwork::new();
    for (id, label) in &[
        ("sentinel", "Synapse Sentinel"),
        ("jupiter",  "JupiterSwapBot"),
        ("solend",   "SolendLendAgent"),
        ("nft",      "NFTFloorTracker"),
        ("trend",    "TrendForge"),
    ] {
        net.add_neuron(id, label, config.fire_threshold, config.leak_rate);
    }

    let tick = Duration::from_secs(config.tick_interval_secs);

    loop {
        match market::fetch(&http).await {
            Ok(signals) => {
                let signal_map = net.build_signals(
                    signals.sol_change_pct_24h,
                    signals.search_volume_norm,
                    signals.sap_activity_norm,
                );
                let fired = net.stimulate(&signal_map);
                info!(
                    tick = net.tick_count,
                    fired = fired.len(),
                    total_fires = net.total_fires,
                    sol_price = signals.sol_price_usd,
                    "tick complete"
                );
                for id in &fired {
                    if let Some(n) = net.neurons.get(id) {
                        info!(neuron = n.label, fires = n.fired_count, "neuron action queued");
                    }
                }
            }
            Err(e) => warn!("market fetch: {e}"),
        }

        if cli.once { break; }
        tokio::time::sleep(tick).await;
    }

    Ok(())
}
