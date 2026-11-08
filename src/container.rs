//! Container definitions for Podman.

/// Container state
#[derive(Debug, Clone, PartialEq)]
pub enum ContainerState {
    /// Created container
    Created,
    /// Running container
    Running,
    /// Paused container
    Paused,
    /// Stopped container
    Stopped,
    /// Exited container
    Exited,
    /// Removing container
    Removing,
}

/// Container config
#[derive(Debug, Clone)]
pub struct ContainerConfig {
    /// Port mappings (host_port, container_port)
    pub ports: Vec<(u16, u16)>,
    /// Environment variables
    pub env_vars: Vec<(String, String)>,
    /// Volume mounts (host_path, container_path, read_only)
    pub volumes: Vec<(String, String, bool)>,
    /// Network name
    pub network: Option<String>,
    /// Memory limit in MB
    pub memory_limit_mb: u64,
    /// CPU limit (cores)
    pub cpu_limit: f64,
    /// Restart policy
    pub restart_policy: String,
    /// Working directory
    pub workdir: String,
    /// Command
    pub command: Vec<String>,
}

impl Default for ContainerConfig {
    fn default() -> Self {
        Self {
            ports: vec![],
            env_vars: vec![],
            volumes: vec![],
            network: None,
            memory_limit_mb: 0,
            cpu_limit: 0.0,
            restart_policy: "no".into(),
            workdir: "/".into(),
            command: vec![],
        }
    }
}

/// Container stats
#[derive(Debug, Clone)]
pub struct ContainerStats {
    /// CPU usage percent
    pub cpu_usage_percent: Option<f64>,
    /// Memory usage in MB
    pub memory_usage_mb: Option<u64>,
    /// Network RX bytes
    pub network_rx_bytes: u64,
    /// Network TX bytes
    pub network_tx_bytes: u64,
    /// Block read bytes
    pub block_read_bytes: u64,
    /// Block write bytes
    pub block_write_bytes: u64,
    /// Uptime in seconds
    pub uptime_seconds: u64,
    /// PIDs
    pub pids: u32,
}

impl Default for ContainerStats {
    fn default() -> Self {
        Self {
            cpu_usage_percent: None,
            memory_usage_mb: None,
            network_rx_bytes: 0,
            network_tx_bytes: 0,
            block_read_bytes: 0,
            block_write_bytes: 0,
            uptime_seconds: 0,
            pids: 0,
        }
    }
}

/// Container
#[derive(Debug, Clone)]
pub struct Container {
    /// Container ID
    pub id: String,
    /// Container name
    pub name: String,
    /// Container image
    pub image: Option<String>,
    /// Container state
    pub state: ContainerState,
    /// Container config
    pub config: ContainerConfig,
    /// Container stats
    pub stats: ContainerStats,
    /// Pod ID
    pub pod_id: Option<String>,
    /// Created timestamp
    pub created_at: u64,
    /// Started timestamp
    pub started_at: Option<u64>,
    /// Stopped timestamp
    pub stopped_at: Option<u64>,
    /// Exit code
    pub exit_code: Option<i32>,
}

impl Container {
    /// Create new container
    pub fn new(name: String) -> Self {
        Self {
            id: format!("container-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            name,
            image: None,
            state: ContainerState::Created,
            config: ContainerConfig::default(),
            stats: ContainerStats::default(),
            pod_id: None,
            created_at: Self::now(),
            started_at: None,
            stopped_at: None,
            exit_code: None,
        }
    }
    
    /// Create container with image
    pub fn with_image(name: String, image: String) -> Self {
        let mut container = Self::new(name);
        container.image = Some(image);
        container
    }
    
    /// Set config
    pub fn set_config(&mut self, config: ContainerConfig) {
        self.config = config;
    }
    
    /// Start container
    pub fn start(&mut self) {
        self.state = ContainerState::Running;
        self.started_at = Some(Self::now());
    }
    
    /// Stop container
    pub fn stop(&mut self) {
        self.state = ContainerState::Exited;
        self.stopped_at = Some(Self::now());
    }
    
    /// Pause container
    pub fn pause(&mut self) {
        if self.state == ContainerState::Running {
            self.state = ContainerState::Paused;
        }
    }
    
    /// Unpause container
    pub fn unpause(&mut self) {
        if self.state == ContainerState::Paused {
            self.state = ContainerState::Running;
        }
    }
    
    /// Set exit code
    pub fn set_exit_code(&mut self, code: i32) {
        self.exit_code = Some(code);
    }
    
    /// Update stats
    pub fn update_stats(&mut self, memory_mb: u64, network_rx: u64, network_tx: u64, cpu_percent: f64) {
        self.stats.memory_usage_mb = Some(memory_mb);
        self.stats.network_rx_bytes = network_rx;
        self.stats.network_tx_bytes = network_tx;
        self.stats.cpu_usage_percent = Some(cpu_percent);
        if let Some(started) = self.started_at {
            self.stats.uptime_seconds = Self::now() - started;
        }
    }
    
    /// Check if running
    pub fn is_running(&self) -> bool {
        self.state == ContainerState::Running
    }
    
    /// Check if paused
    pub fn is_paused(&self) -> bool {
        self.state == ContainerState::Paused
    }
    
    /// Check if exited
    pub fn is_exited(&self) -> bool {
        self.state == ContainerState::Exited
    }
    
    /// Check if created
    pub fn is_created(&self) -> bool {
        self.state == ContainerState::Created
    }
    
    /// Get uptime in seconds
    pub fn get_uptime(&self) -> Option<u64> {
        match (self.started_at, self.stopped_at) {
            (Some(start), Some(stop)) => Some(stop - start),
            (Some(start), None) => Some(Self::now() - start),
            _ => None,
        }
    }
    
    /// Get container age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
