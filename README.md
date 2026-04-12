# FlashStat POC

> **One Block: The Transparency Layer for Ethereum**

A Proof-of-Concept for the **FlashStat** infrastructure project — the first real-time soft-finality monitor for the Unichain network.

## 📍 What This Demonstrates

Unichain produces blocks every **200ms** (Flashblocks). These pre-confirmations are "soft" until finalized on L1. This POC proves we can:

1. **Track the `parent_hash` chain** at 200ms cadence to detect "Soft Reorgs" in real-time.
2. **Calculate a Confidence Score** using the formula `1 - (0.5 ^ persistence)` where persistence is the number of consecutive valid block sightings.
3. **Alert instantly** when a block is replaced before L1 finality.

## 📍 Technical Case Study (60s Baseline on Unichain Sepolia)

```text
🏮 FlashStat POC: Unichain Soft-Finality Monitor
Connecting to https://sepolia.unichain.org...

Timestamp            | Block #    | Confidence   | Status
------------------------------------------------------------
00:01:43.983         | 49182476   |      50.00% | PENDING
00:01:44.532         | 49182477   |      75.00% | PENDING

[!] ALERT: Soft Reorg Detected at Block 49182477!
00:01:45.384         | 49182477   |      50.00% | PENDING
00:01:45.835         | 49182478   |      75.00% | PENDING
```

Soft reorgs were detected multiple times per minute, proving that the **visibility gap** FlashStat aims to fill is a real, ongoing phenomenon on the live testnet.

## 📍 Running the POC

```bash
cargo run --release
```

> Connects to Unichain Sepolia and runs a 60-second baseline.

## 📍 Tech Stack

- **Rust** — High-performance, zero-cost systems programming
- **ethers-rs** — Ethereum RPC client
- **tokio** — Async runtime for high-frequency polling

---

*This is a Proof-of-Concept in support of the FlashStat grant application to the Unichain Foundation.*
