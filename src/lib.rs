//! SigmaOS-podman — Podman container runtime component.
//!
//! Models Podman's daemonless container engine: containers, images,
//! pods, volumes, networks, and the build system for SigmaOS.

pub mod container;
pub mod image;
pub mod pod;
pub mod volume;
pub mod network;
pub mod build;
pub mod manager;
pub mod options;

pub use container::{Container, ContainerState, ContainerConfig, ContainerStats};
pub use image::{Image, ImageConfig, ImageLayer, ImageState};
pub use pod::{Pod, PodConfig, PodInfraContainer, PodState};
pub use volume::{Volume, VolumeConfig, VolumeState};
pub use network::{Network, NetworkConfig, NetworkDriver, NetworkState};
pub use build::{BuildContext, BuildStep, BuildResult, BuildState};
pub use manager::{PodmanManager, ManagerOptions, ManagerStatistics};
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_container_state() {
        let states = vec![
            ContainerState::Created,
            ContainerState::Running,
            ContainerState::Paused,
            ContainerState::Stopped,
            ContainerState::Exited,
            ContainerState::Removing,
        ];
        assert_eq!(states.len(), 6);
    }
    
    #[test]
    fn test_image_state() {
        let states = vec![
            ImageState::Available,
            ImageState::Pulling,
            ImageState::Building,
            ImageState::Removing,
        ];
        assert_eq!(states.len(), 4);
    }
    
    #[test]
    fn test_pod_state() {
        let states = vec![
            PodState::Created,
            PodState::Running,
            PodState::Degraded,
            PodState::Exited,
        ];
        assert_eq!(states.len(), 4);
    }
    
    #[test]
    fn test_volume_state() {
        let states = vec![
            VolumeState::Available,
            VolumeState::InUse,
            VolumeState::Removing,
        ];
        assert_eq!(states.len(), 3);
    }
    
    #[test]
    fn test_network_state() {
        let states = vec![
            NetworkState::Active,
            NetworkState::Inactive,
            NetworkState::Removing,
        ];
        assert_eq!(states.len(), 3);
    }
    
    #[test]
    fn test_build_state() {
        let states = vec![
            BuildState::Pending,
            BuildState::Building,
            BuildState::Completed,
            BuildState::Failed,
            BuildState::Cancelled,
        ];
        assert_eq!(states.len(), 5);
    }
    
    #[test]
    fn test_container_creation() {
        let container = Container::new("nginx".into());
        assert_eq!(container.name, "nginx");
        assert_eq!(container.state, ContainerState::Created);
        assert!(container.image.is_none());
    }
    
    #[test]
    fn test_container_with_image() {
        let container = Container::with_image("web".into(), "nginx:latest".into());
        assert_eq!(container.image, Some("nginx:latest".into()));
    }
    
    #[test]
    fn test_container_config() {
        let mut config = ContainerConfig::default();
        config.ports = vec![(8080, 80)];
        config.env_vars = vec![("DEBUG".into(), "true".into())];
        config.memory_limit_mb = 512;
        config.cpu_limit = 0.5;
        
        assert_eq!(config.ports.len(), 1);
        assert_eq!(config.memory_limit_mb, 512);
    }
    
    #[test]
    fn test_container_lifecycle() {
        let mut container = Container::new("app".into());
        container.start();
        assert_eq!(container.state, ContainerState::Running);
        container.pause();
        assert_eq!(container.state, ContainerState::Paused);
        container.unpause();
        assert_eq!(container.state, ContainerState::Running);
        container.stop();
        assert_eq!(container.state, ContainerState::Exited);
    }
    
    #[test]
    fn test_container_stats() {
        let mut container = Container::new("app".into());
        container.start();
        container.update_stats(1024, 2048, 10, 10.0);
        
        assert_eq!(container.stats.memory_usage_mb, Some(1024));
        assert_eq!(container.stats.cpu_usage_percent, Some(10.0));
    }
    
    #[test]
    fn test_image_creation() {
        let image = Image::new("nginx:latest".into());
        assert_eq!(image.get_full_name(), "nginx:latest");
        assert_eq!(image.state, ImageState::Available);
        assert!(image.layers.is_empty());
    }
    
    #[test]
    fn test_image_layer() {
        let layer = ImageLayer::new("sha256:abc123".into(), 1024 * 1024);
        assert_eq!(layer.digest, "sha256:abc123");
        assert_eq!(layer.size_bytes, 1024 * 1024);
    }
    
    #[test]
    fn test_pod_creation() {
        let pod = Pod::new("web-pod".into());
        assert_eq!(pod.name, "web-pod");
        assert_eq!(pod.state, PodState::Created);
        assert!(pod.containers.is_empty());
    }
    
    #[test]
    fn test_pod_add_container() {
        let mut pod = Pod::new("web-pod".into());
        pod.add_container("nginx".into());
        pod.add_container("redis".into());
        
        assert_eq!(pod.containers.len(), 2);
    }
    
    #[test]
    fn test_volume_creation() {
        let volume = Volume::new("data".into());
        assert_eq!(volume.name, "data");
        assert_eq!(volume.state, VolumeState::Available);
    }
    
    #[test]
    fn test_network_creation() {
        let network = Network::new("mynet".into());
        assert_eq!(network.name, "mynet");
        assert_eq!(network.state, NetworkState::Active);
        assert_eq!(network.config.driver, NetworkDriver::Bridge);
    }
    
    #[test]
    fn test_build_context() {
        let mut context = BuildContext::new(".".into());
        context.dockerfile = "Dockerfile".into();
        context.add_step(BuildStep::new("FROM alpine".into()));
        context.add_step(BuildStep::new("RUN apk add nginx".into()));
        
        assert_eq!(context.steps.len(), 2);
        assert_eq!(context.dockerfile, "Dockerfile");
    }
    
    #[test]
    fn test_manager_creation() {
        let mut manager = PodmanManager::new();
        assert!(manager.list_containers().is_empty());
        assert!(manager.list_images().is_empty());
        assert!(manager.list_pods().is_empty());
    }
    
    #[test]
    fn test_create_container() {
        let mut manager = PodmanManager::new();
        let container_id = manager.create_container("nginx".into(), "nginx:latest".into());
        
        assert!(manager.get_container(&container_id).is_some());
        assert_eq!(manager.list_containers().len(), 1);
    }
    
    #[test]
    fn test_manager_container_lifecycle() {
        let mut manager = PodmanManager::new();
        let container_id = manager.create_container("app".into(), "alpine".into());
        
        assert!(manager.start_container(&container_id));
        assert!(manager.stop_container(&container_id));
    }
    
    #[test]
    fn test_remove_container() {
        let mut manager = PodmanManager::new();
        let container_id = manager.create_container("app".into(), "alpine".into());
        manager.start_container(&container_id);
        manager.stop_container(&container_id);
        
        assert!(manager.remove_container(&container_id));
        assert!(manager.get_container(&container_id).is_none());
    }
    
    #[test]
    fn test_pull_image() {
        let mut manager = PodmanManager::new();
        let image_id = manager.pull_image("nginx".into(), "latest".into());
        
        assert!(manager.get_image(&image_id).is_some());
        assert_eq!(manager.list_images().len(), 1);
    }
    
    #[test]
    fn test_create_pod() {
        let mut manager = PodmanManager::new();
        let pod_id = manager.create_pod("web-pod".into());
        
        assert!(manager.get_pod(&pod_id).is_some());
        assert_eq!(manager.list_pods().len(), 1);
    }
    
    #[test]
    fn test_manager_pod_add_container() {
        let mut manager = PodmanManager::new();
        let pod_id = manager.create_pod("web-pod".into());
        let container_id = manager.create_container("nginx".into(), "nginx:latest".into());
        
        assert!(manager.add_container_to_pod(&pod_id, &container_id));
    }
    
    #[test]
    fn test_create_volume() {
        let mut manager = PodmanManager::new();
        let volume_id = manager.create_volume("data".into());
        
        assert!(manager.get_volume(&volume_id).is_some());
        assert_eq!(manager.list_volumes().len(), 1);
    }
    
    #[test]
    fn test_create_network() {
        let mut manager = PodmanManager::new();
        let network_id = manager.create_network("mynet".into());
        
        assert!(manager.get_network(&network_id).is_some());
        assert_eq!(manager.list_networks().len(), 1);
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = PodmanManager::new();
        manager.create_container("app".into(), "alpine".into());
        manager.pull_image("nginx".into(), "latest".into());
        manager.create_pod("pod1".into());
        
        let export = manager.export_data();
        
        let mut manager2 = PodmanManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_containers().len(), 1);
        assert_eq!(manager2.list_images().len(), 1);
        assert_eq!(manager2.list_pods().len(), 1);
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = PodmanManager::new();
        manager.create_container("app1".into(), "alpine".into());
        manager.create_container("app2".into(), "nginx".into());
        manager.pull_image("redis".into(), "latest".into());
        manager.create_pod("pod1".into());
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_containers, 2);
        assert_eq!(stats.total_images, 1);
        assert_eq!(stats.total_pods, 1);
    }
}
