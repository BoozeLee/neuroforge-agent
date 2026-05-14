use anyhow::{Context, Result};
use solana_sdk::signature::Keypair;

#[derive(Debug, Clone)]
pub struct Config {
    pub synapse_rpc_url: String,
    pub solana_keypair_path: String,
    pub usdc_mint: String,
    pub tick_interval_secs: u64,
    pub fire_threshold: f64,
    pub leak_rate: f64,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();
        Ok(Self {
            synapse_rpc_url: std::env::var("SYNAPSE_RPC_URL")
                .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".into()),
            solana_keypair_path: std::env::var("SOLANA_KEYPAIR_PATH")
                .unwrap_or_else(|_| "keys/agent.json".into()),
            usdc_mint: std::env::var("USDC_MINT")
                .unwrap_or_else(|_| "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".into()),
            tick_interval_secs: std::env::var("TICK_INTERVAL_SECS")
                .ok().and_then(|v| v.parse().ok()).unwrap_or(300),
            fire_threshold: std::env::var("FIRE_THRESHOLD")
                .ok().and_then(|v| v.parse().ok()).unwrap_or(0.75),
            leak_rate: std::env::var("LEAK_RATE")
                .ok().and_then(|v| v.parse().ok()).unwrap_or(0.15),
        })
    }

    pub fn load_keypair(&self) -> Result<Keypair> {
        let contents = std::fs::read_to_string(&self.solana_keypair_path)
            .with_context(|| format!("read keypair: {}", self.solana_keypair_path))?;
        let bytes: Vec<u8> = serde_json::from_str(contents.trim())?;
        Keypair::try_from(bytes.as_slice()).context("build keypair")
    }
}
