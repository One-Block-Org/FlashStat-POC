use ethers::prelude::*;
use std::time::{Duration, Instant};
use chrono::Local;
use eyre::Result;

const UNICHAIN_RPC: &str = "https://sepolia.unichain.org";
const POLL_INTERVAL: u64 = 200; // 200ms

#[tokio::main]
async fn main() -> Result<()> {
    println!("🏮 FlashStat POC: Unichain Soft-Finality Monitor");
    println!("Connecting to {}...", UNICHAIN_RPC);

    let provider = Provider::<Http>::try_from(UNICHAIN_RPC)?;
    
    let mut last_hash: Option<H256> = None;
    let mut persistence: u32 = 0;
    let start_time = Instant::now();

    println!("{:<20} | {:<10} | {:<12} | {:<10}", "Timestamp", "Block #", "Confidence", "Status");
    println!("{:-<60}", "");

    loop {
        if start_time.elapsed().as_secs() > 60 {
            println!("\n[!] 60-second baseline complete.");
            break;
        }

        match provider.get_block(BlockNumber::Pending).await {
            Ok(Some(block)) => {
                let current_hash = block.hash.unwrap_or_default();
                let current_number = block.number.unwrap_or_default();
                
                if Some(current_hash) != last_hash {
                    // New Block Detected
                    let is_sequential = match last_hash {
                        Some(prev_hash) => block.parent_hash == prev_hash,
                        None => true,
                    };

                    if is_sequential {
                        persistence += 1;
                    } else {
                        println!("\n[!] ALERT: Soft Reorg Detected at Block {}!", current_number);
                        persistence = 1;
                    }

                    // Simple Confidence Model: 1 - (0.5 ^ persistence)
                    let confidence = (1.0 - 0.5f64.powi(persistence as i32)) * 100.0;
                    
                    let timestamp = Local::now().format("%H:%M:%S%.3f").to_string();
                    let status = if persistence > 5 { "STABLE" } else { "PENDING" };

                    println!(
                        "{:<20} | {:<10} | {:>10.2}% | {:<10}",
                        timestamp, current_number, confidence, status
                    );

                    last_hash = Some(current_hash);
                }
            }
            Err(e) => eprintln!("[!] RPC Error: {:?}", e),
            _ => (),
        }

        tokio::time::sleep(Duration::from_millis(POLL_INTERVAL)).await;
    }

    Ok(())
}
