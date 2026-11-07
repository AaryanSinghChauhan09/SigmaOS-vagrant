//! vagrant model — development environment automation.

#[derive(Debug, Clone)]
pub struct Machine {
    pub name: String,
    pub box_: String,
    pub status: String,
    pub running: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub machines: Vec<Machine>,
}

impl Default for Model {
    fn default() -> Self { Self { machines: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_machine(&mut self, name: &str, box_: &str) {
        self.machines.push(Machine { name: name.into(), box_: box_.into(), status: "stopped".into(), running: false });
    }

    pub fn up(&mut self, name: &str) -> bool {
        if let Some(m) = self.machines.iter_mut().find(|m| m.name == name) {
            m.running = true;
            m.status = "running".into();
            true
        } else { false }
    }

    pub fn halt(&mut self, name: &str) -> bool {
        if let Some(m) = self.machines.iter_mut().find(|m| m.name == name) {
            m.running = false;
            m.status = "halted".into();
            true
        } else { false }
    }

    pub fn destroy(&mut self, name: &str) -> bool {
        let before = self.machines.len();
        self.machines.retain(|m| m.name != name);
        self.machines.len() < before
    }

    pub fn list(&self) -> &[Machine] { &self.machines }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn up_halt_destroy() {
        let mut v = Model::new();
        v.add_machine("web", "ubuntu/focal64");
        assert!(v.up("web"));
        assert!(v.halt("web"));
        assert!(v.destroy("web"));
    }
}
