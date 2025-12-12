// Copyright (c) 2025 Selim Öztürk
// SPDX-License-Identifier: MIT

use futures::future::join_all;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::time::{Duration, timeout};
use clap::Parser;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 's', long = "start", default_value_t = 1)]
    start_port: u16,

    #[arg(short = 'e', long = "end", default_value_t = 1024)]
    end_port: u16,

    #[arg(short = 't', long = "timeout", default_value_t = 800)]
    timeout_ms: u64,

    #[arg(short = 'c', long = "concurrency", default_value_t = 1000)]
    concurrency: usize, 

    #[arg(required = true)]
    host: String,
}

async fn scan_port(host: &str, port: u16, timeout_ms: u64) -> Option<u16> {
    let addr = format!("{}:{}", host, port);
    match timeout(Duration::from_millis(timeout_ms), TcpStream::connect(&addr)).await {
        Ok(Ok(_)) => Some(port),
        _ => None,
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let host = Arc::new(args.host);
    let start = args.start_port;
    let end = args.end_port + 1; // inclusive
    let timeout_ms = args.timeout_ms;
    let concurrency = args.concurrency.max(1); // en az 1

    let total = (end - start) as usize;
    let scanned = Arc::new(AtomicUsize::new(0));
    let progress_scanned = scanned.clone();

    // Progress bar task (tek thread yazıyor → lock contention yok)
    let progress_host = host.clone();
    let progress_total = total;
    tokio::spawn(async move {
        let mut last = 0usize;
        loop {
            let current = progress_scanned.load(Ordering::Relaxed);
            if current >= progress_total {
                println!("\rScanned {} / {} ports on {} - Done!{}", progress_total, progress_total, progress_host, " ".repeat(20));
                break;
            }
            if current != last && current % 10 == 0 { 
                print!("\rScanned {} / {} ports on {}...", current, progress_total, progress_host);
                let _ = std::io::stdout().flush();
                last = current;
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    });

    
    let sem = Arc::new(Semaphore::new(concurrency));
    let mut tasks = Vec::with_capacity(total);

    for port in start..end {
        let host = host.clone();
        let scanned = scanned.clone();
        let sem = sem.clone();

        let permit = sem.acquire_owned().await.unwrap(); 

        tasks.push(tokio::spawn(async move {
            let result = scan_port(&host, port, timeout_ms).await;
            scanned.fetch_add(1, Ordering::Relaxed);
            drop(permit); 
            result
        }));
    }


    let results = join_all(tasks).await;

    let mut open_ports: Vec<u16> = results
        .into_iter()
        .filter_map(|r| r.ok().flatten())
        .collect();

    open_ports.sort_unstable();

    println!("\nScan completed in full speed!");
    if open_ports.is_empty() {
        println!("  No open ports found.");
    } else {
        println!("  Open ports ({}):", open_ports.len());
        for port in open_ports {
            println!("    {} [OPEN]", port);
        }
    }
    println!("Total scanned: {} ports with max {} concurrent connections.", total, concurrency);
}