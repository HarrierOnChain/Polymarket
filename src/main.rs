//! Trading bot entry point — a venue-scoped launcher on the
//! Prediction Market Toolkits engine. The execution core, risk layer, and
//! strategy implementations live in the shared engine crate:
//!   https://github.com/HarrierOnChain/Prediction-Markets-Trading-Bot-Toolkits

mod venue;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use polymarket_toolkits::{
    bot::{self, BotKind},
    config::AppConfig,
    ui,
};
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "trading-bot")]
#[command(about = "Prediction-market trading bot on the shared engine.", long_about = None)]
struct Cli {
    /// Path to public config (JSON).
    #[arg(long, default_value = "config.json")]
    config: PathBuf,

    /// Path to credentials file (YAML).
    #[arg(long, default_value = "config.yaml")]
    credentials: PathBuf,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Launch the interactive TUI to pick a strategy. (Default if no subcommand.)
    Tui,
    /// Run one strategy headlessly (no TUI).
    Run {
        /// Which strategy to run.
        #[arg(value_enum)]
        strategy: Strategy,
    },
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum Strategy {
    CopyTrading,
    BtcArb,
    CrossArb,
    DirectionHunting,
    SpreadFarming,
    Sports,
    ResolutionSniper,
    OrderbookImbalance,
    MarketMaking,
    WhaleSignal,
}

impl From<Strategy> for BotKind {
    fn from(s: Strategy) -> Self {
        match s {
            Strategy::CopyTrading => BotKind::CopyTrading,
            Strategy::BtcArb => BotKind::BtcArb,
            Strategy::CrossArb => BotKind::CrossArb,
            Strategy::DirectionHunting => BotKind::DirectionHunting,
            Strategy::SpreadFarming => BotKind::SpreadFarming,
            Strategy::Sports => BotKind::Sports,
            Strategy::ResolutionSniper => BotKind::ResolutionSniper,
            Strategy::OrderbookImbalance => BotKind::OrderbookImbalance,
            Strategy::MarketMaking => BotKind::MarketMaking,
            Strategy::WhaleSignal => BotKind::WhaleSignal,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .init();

    info!(venue = venue::NAME, kind = venue::VENUE_TYPE, "starting {} trading bot", venue::NAME);

    let cli = Cli::parse();
    let cfg = AppConfig::load(&cli.config, &cli.credentials).context("loading configuration")?;

    match cli.command {
        Some(Command::Run { strategy }) => bot::run(strategy.into(), cfg).await,
        Some(Command::Tui) | None => ui::run(cfg).await,
    }
}
