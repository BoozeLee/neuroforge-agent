// SAP on-chain escrow v2 — open, deposit, and settle calls.
// Used when a neuron fires: opens escrow with target agent, settles after execution.

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
use tracing::info;

const SAP_PROGRAM_ID: &str = "SAPpUhsWLJG1FfkGRcXagEDMrMsWGjbky7AythGpFETZ";

fn anchor_discriminator(ns: &str) -> [u8; 8] {
    use sha2::{Digest, Sha256};
    Sha256::digest(ns.as_bytes())[..8].try_into().unwrap()
}

fn pda(seeds: &[&[u8]], program: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(seeds, program).0
}

#[derive(BorshSerialize)]
struct CreateEscrowArgs {
    deposit_lamports: u64,
    max_calls: u32,
}

#[derive(BorshSerialize)]
struct SettleCallsArgs {
    num_calls: u32,
    service_label: String,
}

/// Open an escrow account with a SAP agent (neuron fires → open escrow).
pub fn open_escrow(
    rpc: &RpcClient,
    payer: &Keypair,
    agent_wallet: &Pubkey,
    deposit_lamports: u64,
    max_calls: u32,
) -> Result<String> {
    let program = Pubkey::from_str(SAP_PROGRAM_ID).unwrap();
    let escrow_pda = pda(
        &[b"sap_escrow_v2", payer.pubkey().as_ref(), agent_wallet.as_ref()],
        &program,
    );

    let disc = anchor_discriminator("global:create_escrow_v2");
    let args = CreateEscrowArgs { deposit_lamports, max_calls };
    let mut data = disc.to_vec();
    borsh::to_writer(&mut data, &args)?;

    let ix = Instruction {
        program_id: program,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(*agent_wallet, false),
            AccountMeta::new(escrow_pda, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data,
    };

    let blockhash = rpc.get_latest_blockhash()?;
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);
    let tx = Transaction::new(&[payer], msg, blockhash);

    let sig = rpc
        .send_and_confirm_transaction_with_spinner(&tx)
        .context("open_escrow tx failed")?;

    info!(escrow = %escrow_pda, sig = %sig, deposit = deposit_lamports, "escrow opened");
    Ok(sig.to_string())
}

/// Settle calls against an open escrow (payment flows to agent).
pub fn settle_calls(
    rpc: &RpcClient,
    payer: &Keypair,
    agent_wallet: &Pubkey,
    num_calls: u32,
    service: &str,
) -> Result<String> {
    let program = Pubkey::from_str(SAP_PROGRAM_ID).unwrap();
    let escrow_pda = pda(
        &[b"sap_escrow_v2", payer.pubkey().as_ref(), agent_wallet.as_ref()],
        &program,
    );

    let disc = anchor_discriminator("global:settle_calls_v2");
    let args = SettleCallsArgs { num_calls, service_label: service.into() };
    let mut data = disc.to_vec();
    borsh::to_writer(&mut data, &args)?;

    let ix = Instruction {
        program_id: program,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(*agent_wallet, false),
            AccountMeta::new(escrow_pda, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data,
    };

    let blockhash = rpc.get_latest_blockhash()?;
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);
    let tx = Transaction::new(&[payer], msg, blockhash);

    let sig = rpc
        .send_and_confirm_transaction_with_spinner(&tx)
        .context("settle_calls tx failed")?;

    info!(sig = %sig, calls = num_calls, service, "escrow settled");
    Ok(sig.to_string())
}

pub fn escrow_pda(payer: &Pubkey, agent: &Pubkey) -> Pubkey {
    let program = Pubkey::from_str(SAP_PROGRAM_ID).unwrap();
    pda(&[b"sap_escrow_v2", payer.as_ref(), agent.as_ref()], &program)
}
