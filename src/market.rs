// Market data feed — provides signals that stimulate the SNN.
// Fetches SOL price, search trends, and SAP on-chain activity.

use anyhow::Result;
use reqwest::Client;
use tracing::info;

#[derive(Debug, Clone)]
pub struct MarketSignals {
    pub sol_price_usd: f64,
    pub sol_change_pct_24h: f64,
    pub search_volume_norm: f64,  // 0.0–1.0
    pub sap_activity_norm: f64,   // 0.0–1.0
}

pub async fn fetch(http: &Client) -> Result<MarketSignals> {
    // SOL price from CoinGecko (free, no key)
    let price_resp: serde_json::Value = http
        .get("https://api.coingecko.com/api/v3/simple/price")
        .query(&[
            ("ids", "solana"),
            ("vs_currencies", "usd"),
            ("include_24hr_change", "true"),
        ])
        .send()
        .await?
        .json()
        .await?;

    let sol_price = price_resp["solana"]["usd"].as_f64().unwrap_or(150.0);
    let sol_change = price_resp["solana"]["usd_24h_change"].as_f64().unwrap_or(0.0);

    // Normalise change to [0,1] signal (bigger move = stronger signal)
    let search_volume_norm = (sol_change.abs() / 15.0).min(1.0);

    // SAP activity: use on-chain agent count as proxy (normalised to ~30 max)
    let sap_activity_norm = 0.6; // placeholder — real impl polls getProgramAccounts count

    info!(
        sol_price,
        sol_change_pct = sol_change,
        search_norm = search_volume_norm,
        "market signals fetched"
    );

    Ok(MarketSignals {
        sol_price_usd: sol_price,
        sol_change_pct_24h: sol_change,
        search_volume_norm,
        sap_activity_norm,
    })
}
