// Spiking Neural Network (SNN) engine — Leaky Integrate-and-Fire (LIF) model.
// Each SAP agent discovered on-chain becomes a neuron.
// Signals (market data, sentiment, activity) accumulate as membrane potential.
// When potential exceeds threshold, the neuron "fires" — triggering a tool call + escrow payment.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Leaky Integrate-and-Fire neuron representing one SAP tool/agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neuron {
    pub id: String,
    pub label: String,
    pub membrane_potential: f64,
    pub threshold: f64,
    pub leak_rate: f64,   // fraction lost per tick (0.0–1.0)
    pub refractory: u32,  // ticks remaining in refractory period
    pub fired_count: u32,
    pub last_signal: f64,
}

impl Neuron {
    pub fn new(id: &str, label: &str, threshold: f64, leak_rate: f64) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            membrane_potential: 0.0,
            threshold,
            leak_rate,
            refractory: 0,
            fired_count: 0,
            last_signal: 0.0,
        }
    }

    /// Inject a signal and apply leakage. Returns true if the neuron fires.
    pub fn tick(&mut self, input_signal: f64) -> bool {
        if self.refractory > 0 {
            self.refractory -= 1;
            self.membrane_potential *= 1.0 - self.leak_rate;
            return false;
        }

        // Integrate input + apply leak
        self.membrane_potential = self.membrane_potential * (1.0 - self.leak_rate) + input_signal;
        self.last_signal = input_signal;

        if self.membrane_potential >= self.threshold {
            self.membrane_potential = 0.0; // reset after firing
            self.refractory = 3;           // 3-tick refractory period
            self.fired_count += 1;
            info!(
                neuron = self.label,
                fired_count = self.fired_count,
                "⚡ Neuron fired"
            );
            true
        } else {
            false
        }
    }
}

/// The SNN — a layer of neurons that can be stimulated and queried.
pub struct SpikingNetwork {
    pub neurons: HashMap<String, Neuron>,
    pub tick_count: u64,
    pub total_fires: u64,
}

impl SpikingNetwork {
    pub fn new() -> Self {
        Self {
            neurons: HashMap::new(),
            tick_count: 0,
            total_fires: 0,
        }
    }

    /// Add a neuron to the network.
    pub fn add_neuron(&mut self, id: &str, label: &str, threshold: f64, leak: f64) {
        self.neurons.insert(id.into(), Neuron::new(id, label, threshold, leak));
    }

    /// Stimulate all neurons with a signal map and return IDs of neurons that fired.
    pub fn stimulate(&mut self, signals: &HashMap<String, f64>) -> Vec<String> {
        self.tick_count += 1;
        let mut fired = Vec::new();

        for (id, neuron) in self.neurons.iter_mut() {
            let signal = signals.get(id).copied().unwrap_or(0.0);
            if neuron.tick(signal) {
                fired.push(id.clone());
                self.total_fires += 1;
            }
        }
        fired
    }

    /// Build signal map from raw market/activity data.
    /// Each metric is normalised to [0, 1] and weighted.
    pub fn build_signals(
        &self,
        price_change_pct: f64,   // e.g. SOL price Δ%
        search_volume: f64,       // normalised search spike (0–1)
        sap_activity: f64,        // SAP on-chain activity score (0–1)
    ) -> HashMap<String, f64> {
        let base = (price_change_pct.abs() / 10.0).min(1.0) * 0.4
            + search_volume * 0.35
            + sap_activity * 0.25;

        self.neurons
            .keys()
            .map(|id| {
                // Add per-neuron noise for biological realism
                use rand::Rng;
                let noise: f64 = rand::thread_rng().gen_range(-0.05..0.05);
                (id.clone(), (base + noise).clamp(0.0, 1.0))
            })
            .collect()
    }

}
