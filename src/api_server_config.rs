/// API Server Configuration with Fallback Support
/// Provides automatic failover to standby servers
///
/// Features:
/// - Primary server with 5 standby servers
/// - Automatic health checking
/// - Failover when server is down
/// - Load balancing (optional)
/// - Server status caching

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub url: String,
    pub priority: u8,  // 0 = highest priority
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct ServerStatus {
    pub url: String,
    pub is_healthy: bool,
    pub last_checked: Instant,
    pub response_time_ms: Option<u64>,
    pub consecutive_failures: u32,
}

lazy_static::lazy_static! {
    static ref SERVER_MANAGER: Arc<RwLock<ServerManager>> = {
        Arc::new(RwLock::new(ServerManager::new()))
    };
}

pub struct ServerManager {
    servers: Vec<ServerConfig>,
    status_cache: Vec<ServerStatus>,
    health_check_interval: Duration,
    max_failures_before_skip: u32,
}

impl ServerManager {
    pub fn new() -> Self {
        let servers = vec![
            // Primary server
            ServerConfig {
                url: "http://nas.haydenstudio.hk:21114".to_string(),
                priority: 0,
                enabled: true,
            },
            // Standby server 1 - CONFIGURE THIS
            ServerConfig {
                url: "http://standby1.example.com:21114".to_string(),
                priority: 1,
                enabled: false, // Set to true when configured
            },
            // Standby server 2 - CONFIGURE THIS
            ServerConfig {
                url: "http://standby2.example.com:21114".to_string(),
                priority: 2,
                enabled: false, // Set to true when configured
            },
            // Standby server 3 - CONFIGURE THIS
            ServerConfig {
                url: "http://standby3.example.com:21114".to_string(),
                priority: 3,
                enabled: false, // Set to true when configured
            },
            // Standby server 4 - CONFIGURE THIS
            ServerConfig {
                url: "http://standby4.example.com:21114".to_string(),
                priority: 4,
                enabled: false, // Set to true when configured
            },
            // Standby server 5 - CONFIGURE THIS
            ServerConfig {
                url: "http://standby5.example.com:21114".to_string(),
                priority: 5,
                enabled: false, // Set to true when configured
            },
        ];

        let status_cache = servers
            .iter()
            .map(|s| ServerStatus {
                url: s.url.clone(),
                is_healthy: true, // Assume healthy until proven otherwise
                last_checked: Instant::now(),
                response_time_ms: None,
                consecutive_failures: 0,
            })
            .collect();

        Self {
            servers,
            status_cache,
            health_check_interval: Duration::from_secs(30),
            max_failures_before_skip: 3,
        }
    }

    /// Get the best available server
    pub async fn get_server(&mut self) -> Option<String> {
        // Sort by priority (0 = highest)
        let mut available: Vec<_> = self.servers
            .iter()
            .enumerate()
            .filter(|(_, s)| s.enabled)
            .collect();
        available.sort_by_key(|(_, s)| s.priority);

        // Try each server in priority order
        for (idx, server) in available {
            let status = &self.status_cache[idx];

            // Skip if too many consecutive failures
            if status.consecutive_failures >= self.max_failures_before_skip {
                log::warn!(
                    "Skipping server {} due to {} consecutive failures",
                    server.url,
                    status.consecutive_failures
                );
                continue;
            }

            // Check if we need to health check
            if status.last_checked.elapsed() > self.health_check_interval {
                if self.health_check(idx).await {
                    log::info!("Using server: {} (priority {})", server.url, server.priority);
                    return Some(server.url.clone());
                }
            } else if status.is_healthy {
                return Some(server.url.clone());
            }
        }

        log::error!("No healthy servers available!");
        None
    }

    /// Perform health check on a server
    async fn health_check(&mut self, idx: usize) -> bool {
        let server = &self.servers[idx];
        let start = Instant::now();

        log::debug!("Health checking server: {}", server.url);

        match self.check_server_health(&server.url).await {
            Ok(()) => {
                let response_time = start.elapsed().as_millis() as u64;
                self.status_cache[idx].is_healthy = true;
                self.status_cache[idx].last_checked = Instant::now();
                self.status_cache[idx].response_time_ms = Some(response_time);
                self.status_cache[idx].consecutive_failures = 0;

                log::info!(
                    "✓ Server {} is healthy ({}ms)",
                    server.url,
                    response_time
                );
                true
            }
            Err(e) => {
                self.status_cache[idx].is_healthy = false;
                self.status_cache[idx].last_checked = Instant::now();
                self.status_cache[idx].consecutive_failures += 1;

                log::warn!(
                    "✗ Server {} health check failed: {} (failures: {})",
                    server.url,
                    e,
                    self.status_cache[idx].consecutive_failures
                );
                false
            }
        }
    }

    /// Check if server is responding
    async fn check_server_health(&self, url: &str) -> Result<(), String> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .map_err(|e| e.to_string())?;

        let health_url = format!("{}/health", url);

        let response = client
            .get(&health_url)
            .send()
            .await
            .map_err(|e| format!("Connection failed: {}", e))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Server returned status: {}", response.status()))
        }
    }

    /// Mark server as failed (called when API request fails)
    pub fn mark_failure(&mut self, url: &str) {
        if let Some(idx) = self.servers.iter().position(|s| s.url == url) {
            self.status_cache[idx].consecutive_failures += 1;
            self.status_cache[idx].is_healthy = false;
            log::warn!(
                "Marked server {} as failed (failures: {})",
                url,
                self.status_cache[idx].consecutive_failures
            );
        }
    }

    /// Reset failure count (called when request succeeds)
    pub fn mark_success(&mut self, url: &str) {
        if let Some(idx) = self.servers.iter().position(|s| s.url == url) {
            self.status_cache[idx].consecutive_failures = 0;
            self.status_cache[idx].is_healthy = true;
        }
    }

    /// Get server statistics
    pub fn get_stats(&self) -> Vec<(String, ServerStatus)> {
        self.servers
            .iter()
            .zip(self.status_cache.iter())
            .filter(|(s, _)| s.enabled)
            .map(|(s, status)| (s.url.clone(), status.clone()))
            .collect()
    }
}

/// Global function to get the best available API server
pub async fn get_api_server() -> Result<String, String> {
    let mut manager = SERVER_MANAGER.write().await;
    manager.get_server().await.ok_or_else(|| {
        "No API servers available. Please check your network connection or contact support."
            .to_string()
    })
}

/// Mark a server as failed (call this when request fails)
pub async fn mark_server_failed(url: &str) {
    let mut manager = SERVER_MANAGER.write().await;
    manager.mark_failure(url);
}

/// Mark a server as successful (call this when request succeeds)
pub async fn mark_server_success(url: &str) {
    let mut manager = SERVER_MANAGER.write().await;
    manager.mark_success(url);
}

/// Get statistics for all servers
pub async fn get_server_stats() -> Vec<(String, ServerStatus)> {
    let manager = SERVER_MANAGER.read().await;
    manager.get_stats()
}

/// Configure backup servers from environment variables or config file
pub async fn configure_servers(servers: Vec<ServerConfig>) {
    let mut manager = SERVER_MANAGER.write().await;
    manager.servers = servers;
    manager.status_cache = manager.servers
        .iter()
        .map(|s| ServerStatus {
            url: s.url.clone(),
            is_healthy: true,
            last_checked: Instant::now(),
            response_time_ms: None,
            consecutive_failures: 0,
        })
        .collect();

    log::info!("Configured {} API servers", manager.servers.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server_failover() {
        let mut manager = ServerManager::new();

        // Mark primary as failed
        manager.mark_failure("http://nas.haydenstudio.hk:21114");
        manager.mark_failure("http://nas.haydenstudio.hk:21114");
        manager.mark_failure("http://nas.haydenstudio.hk:21114");

        // Should try to use backup server
        let server = manager.get_server().await;
        assert!(server.is_some());

        if let Some(url) = server {
            println!("Failover server: {}", url);
            // Should not be the primary
            assert_ne!(url, "http://nas.haydenstudio.hk:21114");
        }
    }

    #[tokio::test]
    async fn test_get_stats() {
        let stats = get_server_stats().await;
        println!("Server stats:");
        for (url, status) in stats {
            println!(
                "  {} - Healthy: {}, Failures: {}",
                url, status.is_healthy, status.consecutive_failures
            );
        }
    }
}
