//! Polymarket venue metadata.
//!
//! The execution core, risk layer, and strategy implementations live in the
//! shared engine crate. This module just describes the venue this binary targets.

/// Display name of this venue.
pub const NAME: &str = "Polymarket";

/// Venue category.
pub const VENUE_TYPE: &str = "Decentralized (Polygon / USDC)";

/// Strategies this venue runs on the shared engine.
pub const STRATEGIES: &[&str] = &[
    "Copy Trading",
    "BTC 5m / 15m / 1hr Arbitrage",
    "Cross-Market Arbitrage",
    "Directional Arbitrage",
    "Spread Farming",
    "Sports Execution",
    "Resolution Sniper",
    "Orderbook Imbalance",
    "Market Making",
    "On-Chain Whale Signal",
];
