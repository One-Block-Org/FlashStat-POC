# Grant Proposal: FlashStat (Unichain Infrastructure)

**Applicant**: One Block ('The Transparency Layer')  
**Lead**: Michael Dean Oyewole  
**Track**: Unichain Foundation Infrastructure Track  
**Budget**: $60,000  
**Timeline**: 6 Months (3 Milestones)

---

## 🏮 1. Project Overview
**FlashStat** is the first dedicated "Soft Finality" monitor for the Unichain ecosystem. While Unichain achieves sub-second block times (200ms Flashblocks), these confirmations remain "soft" until finalized on-chain. FlashStat provides the essential real-time confidence scores and network health analytics that Wallets, DApps, and Institutional LPs need to operate safely at Unichain's speed.

## 🏮 1a. Technical Feasibility — Live POC Evidence
To validate this proposal, One Block has already built and deployed a working Proof-of-Concept: **[github.com/One-Block-Org/FlashStat-POC](https://github.com/One-Block-Org/FlashStat-POC)**

During a 60-second baseline test against the live **Unichain Sepolia** testnet (April 13, 2026), our POC detected the following:

| Metric | Result |
| :--- | :--- |
| **Blocks Observed** | ~60 blocks |
| **Soft Reorgs Detected** | **18 events** |
| **Reorg Frequency** | **~1 every 3.3 seconds** |
| **Max Soft Confidence** | 75% (chains rarely stabilized beyond 2 consecutive sightings) |

This is not a theoretical concern — the Soft Finality Trust Gap is a **live, ongoing vulnerability** affecting every user on Unichain today. The POC is written in Rust, runs in `< 5ms` per polling cycle, and requires zero modifications to Unichain infrastructure.

## 🏮 2. The Problem: The "Visibility-Trust" Gap
Unichain's performance is a competitive advantage, but it introduces a new UX/Security challenge:
1.  **Ambiguity**: Users see a "confirmed" trade in 200ms, but have no way to measure the risk of a "Soft Reorg."
2.  **Lack of Monitoring**: There is currently no standard API to measure the "Confidence Metric" of a pre-confirmed transaction.
3.  **Hardware Invisibility**: As Unichain scales, the health of the TEE-sequencer and the Rollup-Boost layer needs independent, hardware-level verification.

## 🏮 3. The Solution: Real-Time Network Pulse
FlashStat leverages One Block's high-performance Rust engineering to build a three-layer infrastructure suite:
1.  **The Finality Engine**: A high-frequency indexing core that monitors Unichain's `pending` state, detecting "Soft Reorgs" by verifying the `parent_hash` chain integrity across 200ms increments.
2.  **The Confidence API**: Calculates the security score (0-100) using the **FlashConfidence Formula**: `Confidence = V(TEE) * (P^M) * D`, where `V` is TEE attestation validity, `P` is block persistence, and `D` is the distance to L1 finalization.
3.  **The Pulse Dashboard**: A high-fidelity portal providing real-time network health, sequencer latency, and verified Intel TDX DCAP attestation proofs.

## 🏮 4. Implementation Roadmap

### Milestone 1: The Flashblock Indexer (Month 1-2) — $20,000
- Launch of the Rust-based indexing service optimized for 200ms polling.
- Real-time tracking of `pending` vs `finalized` block discrepancies.
- Public GraphQL endpoint for historical reorg data.

### Milestone 2: The Confidence Scoring Engine & API (Month 3-4) — $20,000
- Implementation of the 'FlashConfidence' algorithm integrated with Unichain's Rollup-Boost streams.
- **Pre-confirmation Risk Monitoring**: Using `debug_traceTransaction` to detect potential reverts before the 1s final block.
- Beta release of the `flash_getConfidence` API for wallet integrators.

### Milestone 3: Professional Dashboard & Public Portal (Month 5-6) — $20,000
- Launch of `flashstat.io`: The public-facing network health dashboard.
- **TEE Heartbeat**: Real-time monitoring of sequencer hardware via the **Intel TDX DCAP Registry Protocol**.
- Documentation and onboarding for 3+ Unichain ecosystem partners (Wallets/Exchanges).

## 🏮 5. Ecosystem Impact
FlashStat turns Unichain's speed from a "risk factor" into a "trust factor." By providing the industry-standard for soft-finality monitoring, **One Block** ensures that Unichain can be safely adopted by professional traders and retail consumers alike.

**Network-level impact at each milestone:**
- **M1**: Wallets can query live reorg rates. First historical reorg dataset for Unichain goes public.
- **M2**: `flash_getConfidence` becomes the standard API call for any DApp checking soft-finality safety.
- **M3**: `flashstat.io` becomes the go-to dashboard for Unichain network health — cited by auditors, validators, and ecosystem partners.

## 🏮 6. Team Background
**One Block** is a high-performance Rust engineering collective. We are the authors of **Atupa** — a published, production-grade EVM visual profiler available on `crates.io` (v0.1.0). Our track record demonstrates that we can deliver production-quality Rust infrastructure on schedule.

- **GitHub**: [github.com/One-Block-Org](https://github.com/One-Block-Org)
- **Atupa**: [crates.io/crates/atupa](https://crates.io/crates/atupa)
- **FlashStat POC**: [github.com/One-Block-Org/FlashStat-POC](https://github.com/One-Block-Org/FlashStat-POC)
