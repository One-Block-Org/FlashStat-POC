# FlashStat: Unichain Soft-Finality Infrastructure 🏮

> **One Block: The Transparency Layer for Ethereum**

FlashStat is the first dedicated infrastructure suite for monitoring **Soft Finality** on the Unichain network. By tracking 200ms Flashblocks in real-time, we provide the "Confidence Layer" necessary for wallets and DApps to safely adopt Unichain's extreme performance.

---

## 📍 Part 1: Technical Proof-of-Concept (POC)

This repository contains a high-performance Rust skeleton that proves the feasibility of real-time soft-reorg detection.

### 🏮 High-Frequency Monitoring
Unichain produces blocks every **200ms**. This POC demonstrates:
1. **Chain Integrity**: Verifies the `parent_hash` chain at a 200ms cadence.
2. **Confidence Modeling**: Calculates a score (0-100) based on block persistence.
3. **Soft Reorg Detection**: Instantly alerts when a pre-confirmation is replaced.

### 🏮 Live Case Study (60s Baseline)
Captured from `sepolia.unichain.org` on April 13, 2026.

```text
Timestamp            | Block #    | Confidence   | Status
------------------------------------------------------------
00:35:20.918         | 49184493   |      50.00% | PENDING

[!] ALERT: Soft Reorg Detected at Block 49184494!
00:35:22.304         | 49184494   |      50.00% | PENDING
00:35:22.741         | 49184495   |      75.00% | PENDING

[!] ALERT: Soft Reorg Detected at Block 49184496!
00:35:24.027         | 49184496   |      50.00% | PENDING
...
```

**Key Finding**: We detected **18 soft reorgs in 60 seconds**. This proves that every user on Unichain is currently blind to a reorg happening on average every 3.3 seconds.

---

## 📍 Part 2: $60,000 Grant Proposal & Roadmap

**Track**: Unichain Foundation Infrastructure  
**Timeline**: 6 Months (3 Milestones)  
**Deliverables**: Indexer, API, and Public Dashboard.

### 🏮 The Problem: The "Visibility-Trust" Gap
Wallets cannot safely show a "Confirmed" checkmark to retail users in 200ms without knowing the probability of a revert. Professional LPs cannot protect their Uniswap v4 hook executions from silent drops. FlashStat bridge this gap.

### 🏮 Implementation Roadmap

#### Milestone 1: Production Indexer ($20,000 / Month 1-2)
- Transition this POC into a production-grade Rust indexing service.
- Launch a public GraphQL endpoint for historical soft-reorg analytics.

#### Milestone 2: Confidence API ($20,000 / Month 3-4)
- Launch of the `flash_getConfidence` JSON-RPC API.
- Implementation of the **FlashConfidence Formula**: `Confidence = V(TEE) * (P^M) * D`.

#### Milestone 3: flashstat.io Dashboard ($20,000 / Month 5-6)
- Public-facing network health portal.
- **TEE Attestation**: Real-time hardware-level verification of the Unichain sequencer via Intel TDX.

### 🏮 Ecosystem Impact
FlashStat turns Unichain's speed from a "risk" into a "trust factor." We enable wallets to confidently display sub-second transactions, directly increasing the safety and depth of liquidity on Uniswap v4.

---

## 📍 Running the POC

```bash
cargo run --release
```

## 📍 Team: One Block
We are a high-performance Rust collective and the authors of **Atupa** (crates.io). We specialize in the "Transparency Layer" of the Ethereum stack.

- **GitHub**: [github.com/One-Block-Org](https://github.com/One-Block-Org)
- **Atupa**: [crates.io/crates/atupa](https://crates.io/crates/atupa)
