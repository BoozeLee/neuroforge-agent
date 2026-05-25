# NeuroForge — Neuromorphic Autonomous SAP Agent

**Category:** General Payment Volume (SAP on-chain escrow)

## 🚀 Quick Start (Demo — No Secrets Needed)

```bash
git clone https://github.com/BoozeLee/neuroforge-agent.git
cd neuroforge-agent

cargo build --release

# Run the neuromorphic demo (simulates the LIF SNN firing)
./target/release/neuroforge --demo
```

This runs the core Leaky Integrate-and-Fire spiking neural network logic locally with synthetic market signals. No blockchain keys required for the demo.

For real on-chain execution see the "Run on mainnet" section below.

## What makes it neuromorphic

Uses a **Leaky Integrate-and-Fire (LIF)** spiking neural network where:
- Each SAP agent discovered on-chain = a neuron
- Market signals (SOL price, search volume, SAP activity) = input stimuli
- Membrane potential accumulates tick by tick, leaks between ticks
- When potential ≥ threshold → neuron **fires** → opens SAP escrow → executes task → settles payment

This mirrors how biological neurons work: integrate signals over time, fire when threshold is crossed, enter refractory period, reset.

## Architecture

```
Market signals (CoinGecko + SAP RPC)
        │
        ▼
  LIF SNN Engine
  ┌──────────────────────────────────┐
  │  N1: Synapse Sentinel  V=0.51   │
  │  N2: JupiterSwapBot    V=0.57   │ ← threshold 0.75
  │  N3: SolendLendAgent   V=0.49   │
  └──────────────────────────────────┘
        │ fire (V ≥ threshold)
        ▼
  SAP create_escrow_v2 → execute → settle_calls_v2
  (on-chain payment volume generated)
```

## Run demo
```bash
cargo run --bin neuroforge -- --demo
```

## Run on mainnet
```bash
cp .env.example .env   # add SYNAPSE_RPC_URL + SOLANA_KEYPAIR_PATH
cargo run --bin register
cargo run --bin neuroforge
```

## Deployment & Verification

**Agent wallet:** `3L5ZJQDzBUDwautD734carHphTtgojAktSvNywnQsuQF`

Fund with `≥ 0.01 SOL`, then:

```bash
# 1. Register on SAP Mainnet (run once)
cargo run --bin register

# 2. Run autonomous escrow loop
cargo run --bin neuroforge

# 3. Verify on explorer
# https://explorer.oobeprotocol.ai/agents/3L5ZJQDzBUDwautD734carHphTtgojAktSvNywnQsuQF
```

**IDL verification:** The SAP v2 IDL was cloned from Mainnet (`--clone SAPpUhsWLJG1FfkGRcXagEDMrMsWGjbky7AyhGpFETZ`) and all instruction discriminators (`create_escrow_v2`, `settle_calls_v2`) verified before submission. The Devnet global registry is non-functional due to a protocol-level initialization issue; Mainnet logic is fully verified.

## GitHub
https://github.com/BoozeLee/neuroforge-agent

## Features
- **Neuromorphic Core**: LIF spiking neural network for decision making
- **SAP Integration**: Full Synapse Agent Protocol lifecycle management
- **X402 Payments**: Autonomous on-chain settlement
