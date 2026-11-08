//! Pod definitions for Podman.

/// Pod state
#[derive(Debug, Clone, PartialEq)]
pub enum PodState {
    /// Created pod
    Created,
    /// Running pod
    Running,
    /// Degraded pod
    Degraded,
    /// Exited pod
    Exited,
}

/// Pod config
#[derive(Debug, Clone)]
pub struct PodConfig {
    /// Port mappings
    pub ports: Vec<(u16, u16)>,
    /// Network name
    pub network: Option<String>,
    /// Shared namespace
    pub share_process_namespace: bool,
    /// Infra container name
    pub infra_name: String,
    /// Pod labels
    pub labels: Vec<(String, String)>,
}

impl Default for PodConfig {
    fn default() -> Self {
        Self {
            ports: vec![],
            network: None,
            share_process_namespace: true,
            infra_name: "podman-infra".into(),
            labels: vec![],
        }
    }
}

/// Pod infra container
#[derive(Debug, Clone)]
pub struct PodInfraContainer {
    /// Container ID
    pub container_id: String,
    /// Container name
    pub name: String,
    /// Container image
    pub image: String,
}

/// Pod
#[derive(Debug, Clone)]
pub struct Pod {
    /// Pod ID
    pub id: String,
    /// Pod name
    pub name: String,
    /// Pod state
    pub state: PodState,
    /// Pod config
    pub config: PodConfig,
    /// Container names in pod
    pub containers: Vec<String>,
    /// Infra container
    pub infra_container: Option<PodInfraContainer>,
    /// Created timestamp
    pub created_at: u64,
    /// Started timestamp
    pub started_at: Option<u64>,
}

impl Pod {
    /// Create new pod
    pub fn new(name: String) -> Self {
        Self {
            id: format!("pod-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            name,
            state: PodState::Created,
            config: PodConfig::default(),
            containers: vec![],
            infra_container: None,
            created_at: Self::now(),
            started_at: None,
        }
    }
    
    /// Set config
    pub fn set_config(&mut self, config: PodConfig) {
        self.config = config;
    }
    
    /// Add container to pod
    pub fn add_container(&mut self, container_name: String) -> bool {
        if !self.containers.contains(&container_name) {
            self.containers.push(container_name);
            true
        } else {
            false
        }
    }
    
    /// Remove container from pod
    pub fn remove_container(&mut self, container_name: &str) -> bool {
        if let Some(pos) = self.containers.iter().position(|c| c == container_name) {
            self.containers.remove(pos);
            true
        } else {
            false
        }
    }
    
    /// Start pod
    pub fn start(&mut self) {
        self.state = PodState::Running;
        self.started_at = Some(Self::now());
    }
    
    /// Stop pod
    pub fn stop(&mut self) {
        self.state = PodState::Exited;
    }
    
    /// Set degraded
    pub fn set_degraded(&mut self) {
        if self.state == PodState::Running {
            self.state = PodState::Degraded;
        }
    }
    
    /// Check if running
    pub fn is_running(&self) -> bool {
        self.state == PodState::Running
    }
    
    /// Check if degraded
    pub fn is_degraded(&self) -> bool {
        self.state == PodState::Degraded
    }
    
    /// Get container count
    pub fn get_container_count(&self) -> usize {
        self.containers.len()
    }
    
    /// Get uptime in seconds
    pub fn get_uptime(&self) -> Option<u64> {
        match (self.started_at, self.state.clone()) {
            (Some(start), PodState::Running) => Some(Self::now() - start),
            (Some(start), PodState::Exited) => Some(Self::now() - start),
            _ => None,
        }
    }
    
    /// Get pod age in seconds
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
