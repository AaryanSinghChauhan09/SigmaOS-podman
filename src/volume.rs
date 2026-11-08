//! Volume definitions for Podman.

/// Volume state
#[derive(Debug, Clone, PartialEq)]
pub enum VolumeState {
    /// Available volume
    Available,
    /// In use volume
    InUse,
    /// Removing volume
    Removing,
}

/// Volume config
#[derive(Debug, Clone)]
pub struct VolumeConfig {
    /// Driver name
    pub driver: String,
    /// Mount point
    pub mount_point: String,
    /// Size limit in bytes
    pub size_limit: Option<u64>,
    /// Read-only
    pub read_only: bool,
    /// Labels
    pub labels: Vec<(String, String)>,
}

impl Default for VolumeConfig {
    fn default() -> Self {
        Self {
            driver: "local".into(),
            mount_point: "/var/lib/containers/storage/volumes".into(),
            size_limit: None,
            read_only: false,
            labels: vec![],
        }
    }
}

/// Volume
#[derive(Debug, Clone)]
pub struct Volume {
    /// Volume ID
    pub id: String,
    /// Volume name
    pub name: String,
    /// Volume state
    pub state: VolumeState,
    /// Volume config
    pub config: VolumeConfig,
    /// Containers using volume
    pub containers: Vec<String>,
    /// Created timestamp
    pub created_at: u64,
}

impl Volume {
    /// Create new volume
    pub fn new(name: String) -> Self {
        Self {
            id: format!("volume-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            name,
            state: VolumeState::Available,
            config: VolumeConfig::default(),
            containers: vec![],
            created_at: Self::now(),
        }
    }
    
    /// Set config
    pub fn set_config(&mut self, config: VolumeConfig) {
        self.config = config;
    }
    
    /// Add container using volume
    pub fn add_container(&mut self, container_id: String) {
        if !self.containers.contains(&container_id) {
            self.containers.push(container_id);
            self.state = VolumeState::InUse;
        }
    }
    
    /// Remove container from volume
    pub fn remove_container(&mut self, container_id: &str) -> bool {
        if let Some(pos) = self.containers.iter().position(|c| c == container_id) {
            self.containers.remove(pos);
            if self.containers.is_empty() {
                self.state = VolumeState::Available;
            }
            return true;
        }
        false
    }
    
    /// Remove volume
    pub fn remove(&mut self) {
        self.state = VolumeState::Removing;
    }
    
    /// Check if available
    pub fn is_available(&self) -> bool {
        self.state == VolumeState::Available
    }
    
    /// Check if in use
    pub fn is_in_use(&self) -> bool {
        self.state == VolumeState::InUse
    }
    
    /// Get container count
    pub fn get_container_count(&self) -> usize {
        self.containers.len()
    }
    
    /// Get volume age in seconds
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
