//! Build definitions for Podman.

/// Build state
#[derive(Debug, Clone, PartialEq)]
pub enum BuildState {
    /// Pending build
    Pending,
    /// Building
    Building,
    /// Completed
    Completed,
    /// Failed
    Failed,
    /// Cancelled
    Cancelled,
}

/// Build step
#[derive(Debug, Clone)]
pub struct BuildStep {
    /// Step instruction
    pub instruction: String,
    /// Step status
    pub status: StepStatus,
    /// Step output
    pub output: String,
    /// Step duration in seconds
    pub duration_seconds: u64,
}

/// Step status
#[derive(Debug, Clone, PartialEq)]
pub enum StepStatus {
    /// Pending step
    Pending,
    /// Running step
    Running,
    /// Completed step
    Completed,
    /// Failed step
    Failed,
}

impl BuildStep {
    /// Create new build step
    pub fn new(instruction: String) -> Self {
        Self {
            instruction,
            status: StepStatus::Pending,
            output: String::new(),
            duration_seconds: 0,
        }
    }
    
    /// Start step
    pub fn start(&mut self) {
        self.status = StepStatus::Running;
    }
    
    /// Complete step
    pub fn complete(&mut self, output: String, duration: u64) {
        self.status = StepStatus::Completed;
        self.output = output;
        self.duration_seconds = duration;
    }
    
    /// Fail step
    pub fn fail(&mut self, error: String) {
        self.status = StepStatus::Failed;
        self.output = error;
    }
}

/// Build context
#[derive(Debug, Clone)]
pub struct BuildContext {
    /// Context path
    pub path: String,
    /// Dockerfile path
    pub dockerfile: String,
    /// Build steps
    pub steps: Vec<BuildStep>,
    /// Build state
    pub state: BuildState,
    /// Base image
    pub base_image: Option<String>,
    /// Target image name
    pub target_image: Option<String>,
    /// Build args
    pub build_args: Vec<(String, String)>,
    /// Created timestamp
    pub created_at: u64,
    /// Completed timestamp
    pub completed_at: Option<u64>,
}

impl BuildContext {
    /// Create new build context
    pub fn new(path: String) -> Self {
        Self {
            path,
            dockerfile: "Dockerfile".into(),
            steps: vec![],
            state: BuildState::Pending,
            base_image: None,
            target_image: None,
            build_args: vec![],
            created_at: Self::now(),
            completed_at: None,
        }
    }
    
    /// Add build step
    pub fn add_step(&mut self, step: BuildStep) {
        self.steps.push(step);
    }
    
    /// Set base image
    pub fn set_base_image(&mut self, image: String) {
        self.base_image = Some(image);
    }
    
    /// Set target image
    pub fn set_target_image(&mut self, image: String) {
        self.target_image = Some(image);
    }
    
    /// Add build arg
    pub fn add_build_arg(&mut self, key: String, value: String) {
        self.build_args.push((key, value));
    }
    
    /// Start build
    pub fn start_build(&mut self) {
        self.state = BuildState::Building;
    }
    
    /// Complete build
    pub fn complete_build(&mut self) {
        self.state = BuildState::Completed;
        self.completed_at = Some(Self::now());
    }
    
    /// Fail build
    pub fn fail_build(&mut self) {
        self.state = BuildState::Failed;
        self.completed_at = Some(Self::now());
    }
    
    /// Cancel build
    pub fn cancel_build(&mut self) {
        self.state = BuildState::Cancelled;
        self.completed_at = Some(Self::now());
    }
    
    /// Get step count
    pub fn get_step_count(&self) -> usize {
        self.steps.len()
    }
    
    /// Get completed step count
    pub fn get_completed_step_count(&self) -> usize {
        self.steps.iter()
            .filter(|s| s.status == StepStatus::Completed)
            .count()
    }
    
    /// Get build duration in seconds
    pub fn get_build_duration(&self) -> Option<u64> {
        self.completed_at.map(|end| end - self.created_at)
    }
    
    /// Check if building
    pub fn is_building(&self) -> bool {
        self.state == BuildState::Building
    }
    
    /// Check if completed
    pub fn is_completed(&self) -> bool {
        self.state == BuildState::Completed
    }
    
    /// Check if failed
    pub fn is_failed(&self) -> bool {
        self.state == BuildState::Failed
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// Build result
#[derive(Debug, Clone)]
pub struct BuildResult {
    /// Build success
    pub success: bool,
    /// Build output
    pub output: String,
    /// Build duration in seconds
    pub duration_seconds: u64,
    /// Image ID if successful
    pub image_id: Option<String>,
    /// Error message if failed
    pub error: Option<String>,
}
