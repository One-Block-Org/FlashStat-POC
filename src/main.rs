use chrono::Local;
use colored::Colorize;
use ethers::prelude::*;
use eyre::Result;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

const UNICHAIN_RPC: &str = "https://sepolia.unichain.org";
const POLL_INTERVAL_MS: u64 = 200;
const STABLE_THRESHOLD: u32 = 5;
const DEFAULT_DURATION_SECS: u64 = 60;

// ---------------------------------------------------------------------------
// Statistics tracker
// ---------------------------------------------------------------------------

#[derive(Default)]
struct SessionStats {
    blocks_seen: u32,
    soft_reorgs: u32,
    confidence_sum: f64,
}

impl SessionStats {
    fn record(&mut self, confidence: f64, is_reorg: bool) {
        self.blocks_seen += 1;
        self.confidence_sum += confidence;
        if is_reorg {
            self.soft_reorgs += 1;
        }
    }

    fn avg_confidence(&self) -> f64 {
        if self.blocks_seen == 0 {
            return 0.0;
        }
        self.confidence_sum / self.blocks_seen as f64
    }

    fn print_summary(&self, elapsed: Duration) {
        println!("\n{}", "═".repeat(60).cyan());
        println!("{}", "  🏮 SESSION SUMMARY".bold().cyan());
        println!("{}", "═".repeat(60).cyan());
        println!(
            "  {:<28} {}",
            "Duration:".bold(),
            format!("{:.1}s", elapsed.as_secs_f64()).white()
        );
        println!(
            "  {:<28} {}",
            "Blocks Observed:".bold(),
            self.blocks_seen.to_string().white()
        );
        println!(
            "  {:<28} {}",
            "Soft Reorgs Detected:".bold(),
            self.soft_reorgs.to_string().red().bold()
        );
        println!(
            "  {:<28} {}",
            "Avg. Block Confidence:".bold(),
            format!("{:.2}%", self.avg_confidence()).yellow()
        );
        if self.blocks_seen > 0 {
            let reorg_rate = elapsed.as_secs_f64() / self.soft_reorgs.max(1) as f64;
            println!(
                "  {:<28} {}",
                "Reorg Frequency:".bold(),
                format!("~1 every {:.1}s", reorg_rate).red()
            );
        }
        println!("{}", "═".repeat(60).cyan());
    }
}

// ---------------------------------------------------------------------------
// Block monitor
// ---------------------------------------------------------------------------

struct BlockMonitor {
    provider: Provider<Http>,
    last_hash: Option<H256>,
    persistence: u32,
    stats: SessionStats,
}

impl BlockMonitor {
    fn new(provider: Provider<Http>) -> Self {
        Self {
            provider,
            last_hash: None,
            persistence: 0,
            stats: SessionStats::default(),
        }
    }

    async fn poll(&mut self) -> Result<()> {
        let Ok(Some(block)) = self.provider.get_block(BlockNumber::Pending).await else {
            return Ok(());
        };

        let current_hash = block.hash.unwrap_or_default();
        let current_number = block.number.unwrap_or_default();

        // Only process new blocks
        if Some(current_hash) == self.last_hash {
            return Ok(());
        }

        let is_sequential = self
            .last_hash
            .map_or(true, |prev| block.parent_hash == prev);

        let is_reorg = !is_sequential;
        if is_reorg {
            self.persistence = 1;
            println!(
                "\n{} {}",
                "[!] SOFT REORG at block".red().bold(),
                current_number.to_string().red().bold()
            );
        } else {
            self.persistence = self.persistence.saturating_add(1);
        }

        // FlashConfidence Formula: 1 - (0.5 ^ persistence)
        let confidence = (1.0 - 0.5f64.powi(self.persistence as i32)) * 100.0;
        let status = if self.persistence >= STABLE_THRESHOLD {
            "STABLE".green().bold()
        } else {
            "PENDING".yellow()
        };

        let timestamp = Local::now().format("%H:%M:%S%.3f").to_string();
        println!(
            "{}  |  {}  |  {}  |  {}",
            timestamp.white(),
            format!("#{current_number}").cyan(),
            format!("{confidence:>7.2}%").yellow(),
            status
        );

        self.stats.record(confidence, is_reorg);
        self.last_hash = Some(current_hash);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<()> {
    // Parse optional duration from argv: `flash-skeleton 120` runs for 2 min
    let duration_secs = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_DURATION_SECS);

    println!("{}", "🏮 FlashStat: Unichain Soft-Finality Monitor".bold().cyan());
    println!(
        "  {}: {}",
        "RPC".bold(),
        UNICHAIN_RPC.white()
    );
    println!(
        "  {}: {}",
        "Duration".bold(),
        format!("{duration_secs}s").white()
    );
    println!("{}", "─".repeat(60).cyan());
    println!(
        "  {}  |  {}  |  {}  |  {}",
        "Timestamp           ".bold(),
        "Block       ".bold(),
        "Confidence".bold(),
        "Status".bold()
    );
    println!("{}", "─".repeat(60).cyan());

    let provider = Provider::<Http>::try_from(UNICHAIN_RPC)?;
    let mut monitor = BlockMonitor::new(provider);

    // Graceful Ctrl+C
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = Arc::clone(&shutdown);
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        shutdown_clone.store(true, Ordering::SeqCst);
    });

    let start = Instant::now();
    let deadline = Duration::from_secs(duration_secs);

    loop {
        if shutdown.load(Ordering::SeqCst) {
            println!("\n{}", "[!] Interrupted by user.".yellow());
            break;
        }
        if start.elapsed() >= deadline {
            println!("\n{}", "[✓] Baseline complete.".green().bold());
            break;
        }

        if let Err(e) = monitor.poll().await {
            eprintln!("{} {:?}", "[!] RPC Error:".red(), e);
        }

        tokio::time::sleep(Duration::from_millis(POLL_INTERVAL_MS)).await;
    }

    monitor.stats.print_summary(start.elapsed());
    Ok(())
}
