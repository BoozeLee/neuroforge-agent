# NeuroForge — Neuromorphic Autonomous SAP Agent

**Category:** General Payment Volume (SAP on-chain escrow)

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

## GitHub
https://github.com/BoozeLee/neuroforge-agent
