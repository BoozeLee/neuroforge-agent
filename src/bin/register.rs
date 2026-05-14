// One-time SAP agent registration for NeuroForge.
use anyhow::{Context, Result};
use borsh::BorshSerialize;
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_program,
    transaction::Transaction,
};
use std::str::FromStr;

const SAP_PROGRAM_ID: &str = "SAPpUhsWLJG1FfkGRcXagEDMrMsWGjbky7AyhGpFETZ";
const GLOBAL_REGISTRY: &str = "9odFrYBBZq6UQC6aGyzMPNXWJQn55kMtfigzhLg6S6L5";

#[derive(BorshSerialize)]
struct Capability { id: String, description: Option<String>, protocol_id: String, version: String }

#[derive(BorshSerialize)]
struct RegisterAgentArgs {
    name: String, description: String, capabilities: Vec<Capability>,
    pricing: Vec<()>, protocols: Vec<String>, agent_id: Option<String>,
    agent_uri: Option<String>, x402_endpoint: Option<String>,
}

fn anchor_discriminator(ns: &str) -> [u8; 8] {
    use sha2::{Digest, Sha256};
    Sha256::digest(ns.as_bytes())[..8].try_into().unwrap()
}

fn pda(seeds: &[&[u8]], program: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(seeds, program).0
}

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let keypair_path = std::env::var("SOLANA_KEYPAIR_PATH").unwrap_or_else(|_| "keys/agent.json".into());
    let rpc_url = std::env::var("SYNAPSE_RPC_URL").unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".into());

    let raw = std::fs::read_to_string(&keypair_path).context("read keypair")?;
    let bytes: Vec<u8> = serde_json::from_str(raw.trim())?;
    let payer = Keypair::try_from(bytes.as_slice()).context("build keypair")?;

    let rpc = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let program = Pubkey::from_str(SAP_PROGRAM_ID).unwrap();
    let global_registry = Pubkey::from_str(GLOBAL_REGISTRY).unwrap();

    let agent_pda   = pda(&[b"sap_agent",   payer.pubkey().as_ref()], &program);
    let stats_pda   = pda(&[b"sap_stats",   payer.pubkey().as_ref()], &program);
    let pricing_pda = pda(&[b"sap_pricing", payer.pubkey().as_ref()], &program);

    if rpc.get_account(&agent_pda).is_ok() {
        println!("Already registered: {}", payer.pubkey());
        return Ok(());
    }

    println!("Registering NeuroForge on SAP mainnet…");

    let args = RegisterAgentArgs {
        name: "NeuroForge".into(),
        description: "Neuromorphic autonomous agent using Leaky Integrate-and-Fire SNN. \
                      SAP agents = neurons. Market signals drive membrane potential. \
                      Threshold crossing fires escrow payments on-chain."
            .into(),
        capabilities: vec![Capability {
            id: "neuro:snn-escrow".into(),
            description: Some("LIF spiking neural network → SAP escrow settlement".into()),
            protocol_id: "sap-escrow".into(),
            version: "1.0.0".into(),
        }],
        pricing: vec![],
        protocols: vec!["sap-escrow".into()],
        agent_id: Some("neuroforge-v1".into()),
        agent_uri: None,
        x402_endpoint: None,
    };

    let disc = anchor_discriminator("global:register_agent");
    let mut data = disc.to_vec();
    borsh::to_writer(&mut data, &args)?;

    let ix = Instruction {
        program_id: program,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(agent_pda, false),
            AccountMeta::new(stats_pda, false),
            AccountMeta::new(pricing_pda, false),
            AccountMeta::new(global_registry, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data,
    };

    let blockhash = rpc.get_latest_blockhash()?;
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);
    let tx = Transaction::new(&[&payer], msg, blockhash);
    let sig = rpc.send_and_confirm_transaction_with_spinner(&tx).context("register tx")?;

    println!("✓ NeuroForge registered! Tx: {sig}");
    println!("  Explorer: https://explorer.oobeprotocol.ai/agents/{}", payer.pubkey());
    Ok(())
}
