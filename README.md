# Polymarket Trading Bot

**🌐 Language / 语言 / Язык:** [English](README.md) · [简体中文](README.zh-CN.md) · [Русский](README.ru.md)

![Status](https://img.shields.io/badge/status-🟢_live-2ea44f?style=flat-square)
[![Engine](https://img.shields.io/badge/engine-shared_core-6e40c9?style=flat-square)](https://github.com/HarrierOnChain/Prediction-Markets-Trading-Bot-Toolkits)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](LICENSE)
[![CI](https://github.com/HarrierOnChain/Polymarket/actions/workflows/ci.yml/badge.svg)](https://github.com/HarrierOnChain/Polymarket/actions/workflows/ci.yml)

> Automated **Polymarket trading bot** — Decentralized (Polygon / USDC). Part of the [Prediction Market Toolkits](https://github.com/HarrierOnChain/Prediction-Markets-Trading-Bot-Toolkits) suite: one execution core, one risk layer, every venue.

**Polymarket** is **live in production today.**

> Full coverage — all 10 strategies run live.

---

## Live on Polymarket

<div align="center">

<img width="820" alt="Polymarket trading bot TUI" src="https://github.com/user-attachments/assets/b6c51ba1-14c6-4582-858c-e9441516dd1d" />
<img width="820" alt="Polymarket trading bot TUI" src="https://github.com/user-attachments/assets/66d9cb72-e14a-414f-93e5-600fb1d3f49f" />

<sub>The shared TUI running against Polymarket — live positions, P&L, and circuit-breaker state. <!-- TODO: swap in Polymarket-specific captures --></sub>

</div>

---

## Strategies on Polymarket

These bots run on Polymarket through a single venue adapter on the shared engine — same risk controls, same TUI, full dry-run support.

| Strategy |
|----------|
| 🎯 **Copy Trading** — mirror wallets that already proved they have alpha |
| ⚡ **BTC 5m / 15m / 1hr Arbitrage** — speed on short-window BTC Up/Down (~42ms end-to-end) |
| 💰 **Cross-Market Arbitrage** — lock the spread, not the direction |
| 🎯 **Directional Arbitrage** — arb base (Up + Down < $1), tilted toward the side with more edge |
| 📈 **Spread Farming** — a thousand 0.5¢ wins compound into one number |
| 🏆 **Sports Execution** — click, filled, done — under 50ms FAK |
| 🎯 **Resolution Sniper** — 95¢ near-certainty → guaranteed $1.00 payout |
| 📊 **Orderbook Imbalance** — the signal *is* the order book, no external feeds |
| 💰 **Market Making** — be the house, not the gambler (two-sided GTD, inventory skew) |
| ⚡ **On-Chain Whale Signal** — 3–30s ahead of the public positions API |

> Want a strategy not listed here on Polymarket? Adapter coverage is demand-driven — [ask](https://t.me/HarrierOnChain).

---

## Quickstart

Clone, drop in your keys, and run — the TUI lets you pick a strategy.

```bash
git clone https://github.com/HarrierOnChain/Polymarket.git
cd Polymarket
cp config.example.yaml config.yaml   # add your keys
cargo run --release                  # launch the TUI
# headless: cargo run --release -- run copy-trading
```

---

## One engine, every venue

This repo is the **Polymarket** entry point. The execution core, risk layer, and all 20+ venue adapters live in the main toolkit:

### 👉 **[Prediction-Markets-Trading-Bot-Toolkits](https://github.com/HarrierOnChain/Prediction-Markets-Trading-Bot-Toolkits)** — the full suite

| | |
|---|---|
| **Order execution** | < 100ms end-to-end |
| **Event processing** | < 1ms per event |
| **Safety** | Circuit breaker · depth guard · dry-run · trade floor |
| **Venues** | Polymarket · Kalshi · Limitless live — 20+ on roadmap |

Adding a venue means writing **one adapter** — not rebuilding a bot.

---

## Get access

| Platform | Link |
|----------|------|
| **Full toolkit** | [Prediction-Markets-Trading-Bot-Toolkits](https://github.com/HarrierOnChain/Prediction-Markets-Trading-Bot-Toolkits) |
| **Telegram** | [@HarrierOnChain](https://t.me/HarrierOnChain) |
| **Discussions** | [GitHub Discussions](https://github.com/HarrierOnChain/Prediction-Markets-Trading-Bot-Toolkits/discussions) |

*Response time is typically within a few hours.*

---

## Related venues

[Kalshi](https://github.com/HarrierOnChain/Kalshi) · [Drift BET](https://github.com/HarrierOnChain/Drift-BET) · [Interactive Brokers ForecastTrader](https://github.com/HarrierOnChain/Interactive-Brokers-ForecastTrader) · [Limitless](https://github.com/HarrierOnChain/Limitless-Exchange)

> Browse the full venue directory in the [main toolkit →](https://github.com/HarrierOnChain/Prediction-Markets-Trading-Bot-Toolkits#venue-coverage)

---

## Disclaimer

> Trading prediction markets involves real financial risk. This software is provided as-is, without warranty. It is not financial advice. Always test with `enable_trading: false` before deploying real capital. Ensure compliance with Polymarket's terms of service and your local regulations.

---

<div align="center">

**Polymarket trading bot · built on the [Prediction Market Toolkits](https://github.com/HarrierOnChain/Prediction-Markets-Trading-Bot-Toolkits) engine**

[![Telegram](https://img.shields.io/badge/Telegram-@HarrierOnChain-26A5E4?style=flat-square&logo=telegram)](https://t.me/HarrierOnChain)

</div>
