use futures::future::join_all;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::net::TcpStream;
use tokio::time::{Duration, timeout};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Starting port (like -s)
    #[arg(short = 's', long = "start", default_value_t = 1)]
    start_port: u16,

    /// Ending port (like -e)
    #[arg(short = 'e', long = "end", default_value_t = 1024)]
    end_port: u16,

    /// Timeout in milliseconds
    #[arg(short = 't', long = "timeout", default_value_t = 1000)]
    timeout_ms: u64,

    

    host :String,
}


async fn scan_port(host: &str, port: u16,time_out: u64) -> bool {
    let addr = format!("{}:{}", host, port);

    let result = timeout(Duration::from_millis(time_out), TcpStream::connect(addr)).await;

    match result {
        Ok(Ok(_)) => true, // connected
        _ => false,        // timed out or connection refused
    }
}
#[tokio::main]
async fn main() {
    let arg = Args::parse();
    let host = arg.host;
    let total_ports = arg.end_port; 
    let start_port=arg.start_port;
    let timeout_num=arg.timeout_ms;
    let counter = Arc::new(AtomicUsize::new(0));

    let mut tasks = Vec::new();

    for port in start_port..total_ports {
        let h = host.to_string();
        let counter_clone = counter.clone();

        tasks.push(tokio::spawn(async move {
            let is_open = scan_port(&h, port,timeout_num).await;

            let current = counter_clone.fetch_add(1, Ordering::SeqCst) + 1;
            print!("\rScanned {} / {}                  ", current, total_ports);

            if is_open { Some(port) } else { None }
        }));
    }
    let results = join_all(tasks).await;
    
    print!("\n\n\n");
    for r in results {
        if let Ok(Some(port)) = r {
            println!("OPEN {}", port);
        }
    }
}
