//! podman model — daemonless container engine.

#[derive(Debug, Clone)]
pub struct Container {
    pub id: String,
    pub image: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub containers: Vec<Container>,
}

impl Default for Model {
    fn default() -> Self { Self { containers: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn run(&mut self, image: &str) -> bool {
        let id = format!("{:x}", self.containers.len() + 1);
        self.containers.push(Container { id, image: image.into(), status: "running".into() });
        true
    }

    pub fn list(&self) -> &[Container] { &self.containers }

    pub fn stop(&mut self, id: &str) -> bool {
        if let Some(c) = self.containers.iter_mut().find(|c| c.id == id) { c.status = "exited".into(); true } else { false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_stop() {
        let mut p = Model::new();
        assert!(p.run("alpine"));
        assert!(p.stop("1"));
        assert_eq!(p.list()[0].status, "exited");
    }
}
