//! Command-line options for podman.

/// Operating mode
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    /// Default mode
    Default,
    /// List containers mode
    ListContainers,
    /// List images mode
    ListImages,
    /// List pods mode
    ListPods,
    /// List volumes mode
    ListVolumes,
    /// List networks mode
    ListNetworks,
    /// Build mode
    Build(String),
    /// Stats mode
    Stats,
}

/// Command-line options
#[derive(Debug, Clone)]
pub struct Options {
    /// Operating mode
    pub mode: Mode,
    /// Config file path
    pub config_path: String,
    /// Storage path
    pub storage_path: String,
    /// Verbose output
    pub verbose: bool,
    /// Quiet mode
    pub quiet: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            mode: Mode::Default,
            config_path: String::new(),
            storage_path: String::new(),
            verbose: false,
            quiet: false,
        }
    }
}
