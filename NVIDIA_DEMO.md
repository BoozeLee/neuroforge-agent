# NVIDIA Demo Guide – NeuroForge Agent

This is the **neuromorphic autonomous SAP agent** using a real Leaky Integrate-and-Fire (LIF) spiking neural network.

## Quick Demo (Zero Secrets Required)

```bash
git clone https://github.com/BoozeLee/neuroforge-agent.git
cd neuroforge-agent

cargo build --release

# Run the neuromorphic demo
./target/release/neuroforge --demo
```

## What the Demo Shows

- Market signals (price, volume, on-chain activity) as input stimuli
- LIF neurons integrating signals over time
- "Firing" when membrane potential crosses threshold
- Autonomous decision to open escrows and settle payments

This is biologically-inspired AI performing real economic work on Solana.

## Full On-Chain Mode (Optional)

See the main README for `register` + mainnet execution instructions.

Agent wallet on Mainnet: `3L5ZJQDzBUDwautD734carHphTtgojAktSvNywnQsuQF`
