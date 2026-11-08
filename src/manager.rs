//! Podman manager.

use std::collections::HashMap;
use crate::container::{Container, ContainerState, ContainerConfig};
use crate::image::{Image, ImageState, ImageConfig};
use crate::pod::{Pod, PodState, PodConfig};
use crate::volume::{Volume, VolumeState, VolumeConfig};
use crate::network::{Network, NetworkState, NetworkConfig};
use crate::build::{BuildContext, BuildState};

/// Manager options
#[derive(Debug, Clone)]
pub struct ManagerOptions {
    /// Max containers
    pub max_containers: usize,
    /// Max images
    pub max_images: usize,
    /// Max pods
    pub max_pods: usize,
    /// Max volumes
    pub max_volumes: usize,
    /// Max networks
    pub max_networks: usize,
    /// Default network
    pub default_network: String,
    /// Storage driver
    pub storage_driver: String,
}

impl Default for ManagerOptions {
    fn default() -> Self {
        Self {
            max_containers: 1000,
            max_images: 5000,
            max_pods: 100,
            max_volumes: 1000,
            max_networks: 100,
            default_network: "podman".into(),
            storage_driver: "overlay".into(),
        }
    }
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total containers
    pub total_containers: usize,
    /// Running containers
    pub running_containers: usize,
    /// Total images
    pub total_images: usize,
    /// Total pods
    pub total_pods: usize,
    /// Total volumes
    pub total_volumes: usize,
    /// Total networks
    pub total_networks: usize,
    /// Total builds
    pub total_builds: usize,
    /// Manager uptime in seconds
    pub uptime: u64,
}

/// Podman manager
#[derive(Debug, Clone)]
pub struct PodmanManager {
    /// Containers
    containers: HashMap<String, Container>,
    /// Images
    images: HashMap<String, Image>,
    /// Pods
    pods: HashMap<String, Pod>,
    /// Volumes
    volumes: HashMap<String, Volume>,
    /// Networks
    networks: HashMap<String, Network>,
    /// Build contexts
    builds: HashMap<String, BuildContext>,
    /// Options
    options: ManagerOptions,
    /// Statistics
    statistics: ManagerStatistics,
    /// Manager created timestamp
    pub created_at: u64,
}

impl Default for PodmanManager {
    fn default() -> Self {
        Self {
            containers: HashMap::new(),
            images: HashMap::new(),
            pods: HashMap::new(),
            volumes: HashMap::new(),
            networks: HashMap::new(),
            builds: HashMap::new(),
            options: ManagerOptions::default(),
            statistics: ManagerStatistics {
                total_containers: 0,
                running_containers: 0,
                total_images: 0,
                total_pods: 0,
                total_volumes: 0,
                total_networks: 0,
                total_builds: 0,
                uptime: 0,
            },
            created_at: Self::now(),
        }
    }
}

impl PodmanManager {
    /// Create new podman manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create manager with options
    pub fn with_options(options: ManagerOptions) -> Self {
        Self {
            containers: HashMap::new(),
            images: HashMap::new(),
            pods: HashMap::new(),
            volumes: HashMap::new(),
            networks: HashMap::new(),
            builds: HashMap::new(),
            options,
            statistics: ManagerStatistics {
                total_containers: 0,
                running_containers: 0,
                total_images: 0,
                total_pods: 0,
                total_volumes: 0,
                total_networks: 0,
                total_builds: 0,
                uptime: 0,
            },
            created_at: Self::now(),
        }
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        // Create default network
        let network = Network::new(self.options.default_network.clone());
        self.networks.insert(network.id.clone(), network);
        self.update_statistics();
    }
    
    // Container methods
    
    /// Create container
    pub fn create_container(&mut self, name: String, image: String) -> String {
        let container = Container::with_image(name, image);
        let id = container.id.clone();
        
        self.containers.insert(id.clone(), container);
        self.update_statistics();
        id
    }
    
    /// Create container with config
    pub fn create_container_with_config(&mut self, name: String, image: String, config: ContainerConfig) -> String {
        let mut container = Container::with_image(name, image);
        container.set_config(config);
        let id = container.id.clone();
        
        self.containers.insert(id.clone(), container);
        self.update_statistics();
        id
    }
    
    /// Get container
    pub fn get_container(&self, container_id: &str) -> Option<&Container> {
        self.containers.get(container_id)
    }
    
    /// Get container mutably
    pub fn get_container_mut(&mut self, container_id: &str) -> Option<&mut Container> {
        self.containers.get_mut(container_id)
    }
    
    /// Get all containers
    pub fn list_containers(&self) -> Vec<&Container> {
        self.containers.values().collect()
    }
    
    /// Remove container
    pub fn remove_container(&mut self, container_id: &str) -> bool {
        if let Some(container) = self.containers.get(container_id) {
            // Remove from pod if in one
            if let Some(pod_id) = &container.pod_id {
                if let Some(pod) = self.pods.get_mut(pod_id) {
                    pod.remove_container(&container.name);
                }
            }
            // Remove from networks
            for network in self.networks.values_mut() {
                network.remove_container(container_id);
            }
        }
        
        if self.containers.remove(container_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Start container
    pub fn start_container(&mut self, container_id: &str) -> bool {
        if let Some(container) = self.containers.get_mut(container_id) {
            container.start();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Stop container
    pub fn stop_container(&mut self, container_id: &str) -> bool {
        if let Some(container) = self.containers.get_mut(container_id) {
            container.stop();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Pause container
    pub fn pause_container(&mut self, container_id: &str) -> bool {
        if let Some(container) = self.containers.get_mut(container_id) {
            container.pause();
            return true;
        }
        false
    }
    
    /// Unpause container
    pub fn unpause_container(&mut self, container_id: &str) -> bool {
        if let Some(container) = self.containers.get_mut(container_id) {
            container.unpause();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get containers by state
    pub fn get_containers_by_state(&self, state: &ContainerState) -> Vec<&Container> {
        self.containers.values()
            .filter(|c| c.state == *state)
            .collect()
    }
    
    /// Get running containers
    pub fn get_running_containers(&self) -> Vec<&Container> {
        self.get_containers_by_state(&ContainerState::Running)
    }
    
    /// Search containers
    pub fn search_containers(&self, query: &str) -> Vec<&Container> {
        let query_lower = query.to_lowercase();
        self.containers.values()
            .filter(|c| c.name.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Image methods
    
    /// Pull image
    pub fn pull_image(&mut self, name: String, tag: String) -> String {
        let full_name = format!("{}:{}", name, tag);
        let image = Image::new(full_name);
        let id = image.id.clone();
        
        self.images.insert(id.clone(), image);
        self.update_statistics();
        id
    }
    
    /// Get image
    pub fn get_image(&self, image_id: &str) -> Option<&Image> {
        self.images.get(image_id)
    }
    
    /// Get image mutably
    pub fn get_image_mut(&mut self, image_id: &str) -> Option<&mut Image> {
        self.images.get_mut(image_id)
    }
    
    /// Get all images
    pub fn list_images(&self) -> Vec<&Image> {
        self.images.values().collect()
    }
    
    /// Remove image
    pub fn remove_image(&mut self, image_id: &str) -> bool {
        if self.images.remove(image_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get images by state
    pub fn get_images_by_state(&self, state: &ImageState) -> Vec<&Image> {
        self.images.values()
            .filter(|i| i.state == *state)
            .collect()
    }
    
    /// Search images
    pub fn search_images(&self, query: &str) -> Vec<&Image> {
        let query_lower = query.to_lowercase();
        self.images.values()
            .filter(|i| i.name.to_lowercase().contains(&query_lower) ||
                       i.tag.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Pod methods
    
    /// Create pod
    pub fn create_pod(&mut self, name: String) -> String {
        let pod = Pod::new(name);
        let id = pod.id.clone();
        
        self.pods.insert(id.clone(), pod);
        self.update_statistics();
        id
    }
    
    /// Get pod
    pub fn get_pod(&self, pod_id: &str) -> Option<&Pod> {
        self.pods.get(pod_id)
    }
    
    /// Get pod mutably
    pub fn get_pod_mut(&mut self, pod_id: &str) -> Option<&mut Pod> {
        self.pods.get_mut(pod_id)
    }
    
    /// Get all pods
    pub fn list_pods(&self) -> Vec<&Pod> {
        self.pods.values().collect()
    }
    
    /// Remove pod
    pub fn remove_pod(&mut self, pod_id: &str) -> bool {
        if let Some(pod) = self.pods.get(pod_id) {
            // Remove containers from pod
            for container_name in &pod.containers {
                // Find and remove container
                let container_id = self.containers.values()
                    .find(|c| c.name == *container_name)
                    .map(|c| c.id.clone());
                if let Some(cid) = container_id {
                    self.containers.remove(&cid);
                }
            }
        }
        
        if self.pods.remove(pod_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Add container to pod
    pub fn add_container_to_pod(&mut self, pod_id: &str, container_id: &str) -> bool {
        if let (Some(pod), Some(container)) = (self.pods.get_mut(pod_id), self.containers.get_mut(container_id)) {
            if pod.add_container(container.name.clone()) {
                container.pod_id = Some(pod_id.to_string());
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Start pod
    pub fn start_pod(&mut self, pod_id: &str) -> bool {
        if let Some(pod) = self.pods.get_mut(pod_id) {
            pod.start();
            // Start all containers in pod
            let container_ids: Vec<String> = self.containers.values()
                .filter(|c| c.pod_id.as_ref() == Some(&pod_id.to_string()))
                .map(|c| c.id.clone())
                .collect();
            for cid in container_ids {
                if let Some(container) = self.containers.get_mut(&cid) {
                    container.start();
                }
            }
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Stop pod
    pub fn stop_pod(&mut self, pod_id: &str) -> bool {
        if let Some(pod) = self.pods.get_mut(pod_id) {
            pod.stop();
            // Stop all containers in pod
            let container_ids: Vec<String> = self.containers.values()
                .filter(|c| c.pod_id.as_ref() == Some(&pod_id.to_string()))
                .map(|c| c.id.clone())
                .collect();
            for cid in container_ids {
                if let Some(container) = self.containers.get_mut(&cid) {
                    container.stop();
                }
            }
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get pods by state
    pub fn get_pods_by_state(&self, state: &PodState) -> Vec<&Pod> {
        self.pods.values()
            .filter(|p| p.state == *state)
            .collect()
    }
    
    /// Get running pods
    pub fn get_running_pods(&self) -> Vec<&Pod> {
        self.get_pods_by_state(&PodState::Running)
    }
    
    /// Search pods
    pub fn search_pods(&self, query: &str) -> Vec<&Pod> {
        let query_lower = query.to_lowercase();
        self.pods.values()
            .filter(|p| p.name.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Volume methods
    
    /// Create volume
    pub fn create_volume(&mut self, name: String) -> String {
        let volume = Volume::new(name);
        let id = volume.id.clone();
        
        self.volumes.insert(id.clone(), volume);
        self.update_statistics();
        id
    }
    
    /// Get volume
    pub fn get_volume(&self, volume_id: &str) -> Option<&Volume> {
        self.volumes.get(volume_id)
    }
    
    /// Get volume mutably
    pub fn get_volume_mut(&mut self, volume_id: &str) -> Option<&mut Volume> {
        self.volumes.get_mut(volume_id)
    }
    
    /// Get all volumes
    pub fn list_volumes(&self) -> Vec<&Volume> {
        self.volumes.values().collect()
    }
    
    /// Remove volume
    pub fn remove_volume(&mut self, volume_id: &str) -> bool {
        if let Some(volume) = self.volumes.get(volume_id) {
            if volume.is_in_use() {
                return false; // Cannot remove in-use volume
            }
        }
        
        if self.volumes.remove(volume_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get volumes by state
    pub fn get_volumes_by_state(&self, state: &VolumeState) -> Vec<&Volume> {
        self.volumes.values()
            .filter(|v| v.state == *state)
            .collect()
    }
    
    /// Search volumes
    pub fn search_volumes(&self, query: &str) -> Vec<&Volume> {
        let query_lower = query.to_lowercase();
        self.volumes.values()
            .filter(|v| v.name.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Network methods
    
    /// Create network
    pub fn create_network(&mut self, name: String) -> String {
        let network = Network::new(name);
        let id = network.id.clone();
        
        self.networks.insert(id.clone(), network);
        self.update_statistics();
        id
    }
    
    /// Get network
    pub fn get_network(&self, network_id: &str) -> Option<&Network> {
        self.networks.get(network_id)
    }
    
    /// Get network mutably
    pub fn get_network_mut(&mut self, network_id: &str) -> Option<&mut Network> {
        self.networks.get_mut(network_id)
    }
    
    /// Get all networks
    pub fn list_networks(&self) -> Vec<&Network> {
        self.networks.values().collect()
    }
    
    /// Remove network
    pub fn remove_network(&mut self, network_id: &str) -> bool {
        if self.networks.remove(network_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Connect container to network
    pub fn connect_container_to_network(&mut self, container_id: &str, network_id: &str) -> bool {
        if let (Some(container), Some(network)) = (self.containers.get_mut(container_id), self.networks.get_mut(network_id)) {
            network.add_container(container_id.to_string());
            container.config.network = Some(network_id.to_string());
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Disconnect container from network
    pub fn disconnect_container_from_network(&mut self, container_id: &str, network_id: &str) -> bool {
        if let Some(network) = self.networks.get_mut(network_id) {
            if network.remove_container(container_id) {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Get networks by state
    pub fn get_networks_by_state(&self, state: &NetworkState) -> Vec<&Network> {
        self.networks.values()
            .filter(|n| n.state == *state)
            .collect()
    }
    
    /// Get active networks
    pub fn get_active_networks(&self) -> Vec<&Network> {
        self.get_networks_by_state(&NetworkState::Active)
    }
    
    /// Search networks
    pub fn search_networks(&self, query: &str) -> Vec<&Network> {
        let query_lower = query.to_lowercase();
        self.networks.values()
            .filter(|n| n.name.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Build methods
    
    /// Create build context
    pub fn create_build(&mut self, path: String) -> String {
        let build = BuildContext::new(path);
        let id = format!("build-{}", self.builds.len() + 1);
        
        self.builds.insert(id.clone(), build);
        self.update_statistics();
        id
    }
    
    /// Get build
    pub fn get_build(&self, build_id: &str) -> Option<&BuildContext> {
        self.builds.get(build_id)
    }
    
    /// Get build mutably
    pub fn get_build_mut(&mut self, build_id: &str) -> Option<&mut BuildContext> {
        self.builds.get_mut(build_id)
    }
    
    /// Get all builds
    pub fn list_builds(&self) -> Vec<&BuildContext> {
        self.builds.values().collect()
    }
    
    /// Remove build
    pub fn remove_build(&mut self, build_id: &str) -> bool {
        if self.builds.remove(build_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Start build
    pub fn start_build(&mut self, build_id: &str) -> bool {
        if let Some(build) = self.builds.get_mut(build_id) {
            build.start_build();
            return true;
        }
        false
    }
    
    /// Complete build
    pub fn complete_build(&mut self, build_id: &str) -> bool {
        if let Some(build) = self.builds.get_mut(build_id) {
            build.complete_build();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Fail build
    pub fn fail_build(&mut self, build_id: &str) -> bool {
        if let Some(build) = self.builds.get_mut(build_id) {
            build.fail_build();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get builds by state
    pub fn get_builds_by_state(&self, state: &BuildState) -> Vec<&BuildContext> {
        self.builds.values()
            .filter(|b| b.state == *state)
            .collect()
    }
    
    /// Search builds
    pub fn search_builds(&self, query: &str) -> Vec<&BuildContext> {
        let query_lower = query.to_lowercase();
        self.builds.values()
            .filter(|b| b.path.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Statistics methods
    
    /// Get manager uptime
    pub fn get_uptime(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_containers = self.containers.len();
        self.statistics.running_containers = self.containers.values()
            .filter(|c| c.is_running())
            .count();
        self.statistics.total_images = self.images.len();
        self.statistics.total_pods = self.pods.len();
        self.statistics.total_volumes = self.volumes.len();
        self.statistics.total_networks = self.networks.len();
        self.statistics.total_builds = self.builds.len();
        self.statistics.uptime = self.get_uptime();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
    
    /// Export manager data
    pub fn export_data(&self) -> ManagerExport {
        ManagerExport {
            containers: self.containers.values().cloned().collect(),
            images: self.images.values().cloned().collect(),
            pods: self.pods.values().cloned().collect(),
            volumes: self.volumes.values().cloned().collect(),
            networks: self.networks.values().cloned().collect(),
            builds: self.builds.values().cloned().collect(),
        }
    }
    
    /// Import manager data
    pub fn import_data(&mut self, export: ManagerExport) {
        self.containers.clear();
        for container in export.containers {
            self.containers.insert(container.id.clone(), container);
        }
        
        self.images.clear();
        for image in export.images {
            self.images.insert(image.id.clone(), image);
        }
        
        self.pods.clear();
        for pod in export.pods {
            self.pods.insert(pod.id.clone(), pod);
        }
        
        self.volumes.clear();
        for volume in export.volumes {
            self.volumes.insert(volume.id.clone(), volume);
        }
        
        self.networks.clear();
        for network in export.networks {
            self.networks.insert(network.id.clone(), network);
        }
        
        self.builds.clear();
        for build in export.builds {
            self.builds.insert(build.path.clone(), build);
        }
        
        self.update_statistics();
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// Manager export data
#[derive(Debug, Clone)]
pub struct ManagerExport {
    /// Containers
    pub containers: Vec<Container>,
    /// Images
    pub images: Vec<Image>,
    /// Pods
    pub pods: Vec<Pod>,
    /// Volumes
    pub volumes: Vec<Volume>,
    /// Networks
    pub networks: Vec<Network>,
    /// Builds
    pub builds: Vec<BuildContext>,
}
