//! Network definitions for Podman.

/// Network driver
#[derive(Debug, Clone, PartialEq)]
pub enum NetworkDriver {
    /// Bridge driver
    Bridge,
    /// Host driver
    Host,
    /// None driver
    None,
    /// Overlay driver
    Overlay,
    /// Macvlan driver
    Macvlan,
    /// Ipvlan driver
    Ipvlan,
}

/// Network state
#[derive(Debug, Clone, PartialEq)]
pub enum NetworkState {
    /// Active network
    Active,
    /// Inactive network
    Inactive,
    /// Removing network
    Removing,
}

/// Network config
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    /// Driver
    pub driver: NetworkDriver,
    /// Subnet
    pub subnet: Option<String>,
    /// Gateway
    pub gateway: Option<String>,
    /// DNS servers
    pub dns_servers: Vec<String>,
    /// Internal network
    pub internal: bool,
    /// Labels
    pub labels: Vec<(String, String)>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            driver: NetworkDriver::Bridge,
            subnet: None,
            gateway: None,
            dns_servers: vec![],
            internal: false,
            labels: vec![],
        }
    }
}

/// Network
#[derive(Debug, Clone)]
pub struct Network {
    /// Network ID
    pub id: String,
    /// Network name
    pub name: String,
    /// Network state
    pub state: NetworkState,
    /// Network config
    pub config: NetworkConfig,
    /// Containers on network
    pub containers: Vec<String>,
    /// Created timestamp
    pub created_at: u64,
}

impl Network {
    /// Create new network
    pub fn new(name: String) -> Self {
        Self {
            id: format!("network-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            name,
            state: NetworkState::Active,
            config: NetworkConfig::default(),
            containers: vec![],
            created_at: Self::now(),
        }
    }
    
    /// Set config
    pub fn set_config(&mut self, config: NetworkConfig) {
        self.config = config;
    }
    
    /// Add container to network
    pub fn add_container(&mut self, container_id: String) {
        if !self.containers.contains(&container_id) {
            self.containers.push(container_id);
        }
    }
    
    /// Remove container from network
    pub fn remove_container(&mut self, container_id: &str) -> bool {
        if let Some(pos) = self.containers.iter().position(|c| c == container_id) {
            self.containers.remove(pos);
            return true;
        }
        false
    }
    
    /// Deactivate network
    pub fn deactivate(&mut self) {
        self.state = NetworkState::Inactive;
    }
    
    /// Activate network
    pub fn activate(&mut self) {
        self.state = NetworkState::Active;
    }
    
    /// Remove network
    pub fn remove(&mut self) {
        self.state = NetworkState::Removing;
    }
    
    /// Check if active
    pub fn is_active(&self) -> bool {
        self.state == NetworkState::Active
    }
    
    /// Check if inactive
    pub fn is_inactive(&self) -> bool {
        self.state == NetworkState::Inactive
    }
    
    /// Get container count
    pub fn get_container_count(&self) -> usize {
        self.containers.len()
    }
    
    /// Get network age in seconds
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
