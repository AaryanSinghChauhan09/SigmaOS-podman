//! Image definitions for Podman.

/// Image state
#[derive(Debug, Clone, PartialEq)]
pub enum ImageState {
    /// Available image
    Available,
    /// Pulling image
    Pulling,
    /// Building image
    Building,
    /// Removing image
    Removing,
}

/// Image layer
#[derive(Debug, Clone)]
pub struct ImageLayer {
    /// Layer digest
    pub digest: String,
    /// Layer size in bytes
    pub size_bytes: u64,
    /// Layer created timestamp
    pub created_at: u64,
    /// Layer media type
    pub media_type: String,
}

impl ImageLayer {
    /// Create new image layer
    pub fn new(digest: String, size_bytes: u64) -> Self {
        Self {
            digest,
            size_bytes,
            created_at: Self::now(),
            media_type: "application/vnd.docker.image.rootfs.diff.tar.gzip".into(),
        }
    }
    
    /// Get layer size in MB
    pub fn get_size_mb(&self) -> f64 {
        self.size_bytes as f64 / (1024.0 * 1024.0)
    }
    
    /// Get layer age in seconds
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

/// Image config
#[derive(Debug, Clone)]
pub struct ImageConfig {
    /// Entrypoint
    pub entrypoint: Vec<String>,
    /// Cmd
    pub cmd: Vec<String>,
    /// Working dir
    pub workdir: String,
    /// Env vars
    pub env: Vec<(String, String)>,
    /// Exposed ports
    pub exposed_ports: Vec<u16>,
    /// Volumes
    pub volumes: Vec<String>,
    /// Labels
    pub labels: Vec<(String, String)>,
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self {
            entrypoint: vec![],
            cmd: vec![],
            workdir: "/".into(),
            env: vec![],
            exposed_ports: vec![],
            volumes: vec![],
            labels: vec![],
        }
    }
}

/// Image
#[derive(Debug, Clone)]
pub struct Image {
    /// Image ID
    pub id: String,
    /// Image name
    pub name: String,
    /// Image tag
    pub tag: String,
    /// Image state
    pub state: ImageState,
    /// Image layers
    pub layers: Vec<ImageLayer>,
    /// Image config
    pub config: ImageConfig,
    /// Image size in bytes
    pub size_bytes: u64,
    /// Created timestamp
    pub created_at: u64,
    /// Pulled timestamp
    pub pulled_at: Option<u64>,
}

impl Image {
    /// Create new image
    pub fn new(name: String) -> Self {
        let (name_part, tag_part) = if let Some(pos) = name.rfind(':') {
            (name[..pos].to_string(), name[pos+1..].to_string())
        } else {
            (name, "latest".into())
        };
        
        Self {
            id: format!("image-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            name: name_part,
            tag: tag_part,
            state: ImageState::Available,
            layers: vec![],
            config: ImageConfig::default(),
            size_bytes: 0,
            created_at: Self::now(),
            pulled_at: None,
        }
    }
    
    /// Add layer
    pub fn add_layer(&mut self, layer: ImageLayer) {
        self.size_bytes += layer.size_bytes;
        self.layers.push(layer);
    }
    
    /// Set config
    pub fn set_config(&mut self, config: ImageConfig) {
        self.config = config;
    }
    
    /// Mark as pulled
    pub fn mark_pulled(&mut self) {
        self.pulled_at = Some(Self::now());
    }
    
    /// Start pulling
    pub fn start_pull(&mut self) {
        self.state = ImageState::Pulling;
    }
    
    /// Finish pulling
    pub fn finish_pull(&mut self) {
        self.state = ImageState::Available;
        self.mark_pulled();
    }
    
    /// Remove image
    pub fn remove(&mut self) {
        self.state = ImageState::Removing;
    }
    
    /// Get full name
    pub fn get_full_name(&self) -> String {
        format!("{}:{}", self.name, self.tag)
    }
    
    /// Get size in MB
    pub fn get_size_mb(&self) -> f64 {
        self.size_bytes as f64 / (1024.0 * 1024.0)
    }
    
    /// Get layer count
    pub fn get_layer_count(&self) -> usize {
        self.layers.len()
    }
    
    /// Check if available
    pub fn is_available(&self) -> bool {
        self.state == ImageState::Available
    }
    
    /// Check if pulling
    pub fn is_pulling(&self) -> bool {
        self.state == ImageState::Pulling
    }
    
    /// Get image age in seconds
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
