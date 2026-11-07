//! vagrant model — VM provisioning with Vagrantfiles.

#[derive(Debug, Clone)]
pub struct Box_ {
    pub name: String,
    pub provider: String,
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct Machine {
    pub name: String,
    pub box_: String,
    pub state: String,
    pub synced_folder: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub boxes: Vec<Box_>,
    pub machines: Vec<Machine>,
}

impl Default for Model {
    fn default() -> Self { Self {
        boxes: vec![
            Box_ { name: "ubuntu/focal64".into(), provider: "virtualbox".into(), version: "2021.01.01".into() },
            Box_ { name: "hashicorp/bionic64".into(), provider: "libvirt".into(), version: "2021.01.01".into() },
        ],
        machines: vec![],
    } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn up(&mut self, name: &str, box_name: &str) -> bool {
        if self.boxes.iter().any(|b| b.name == box_name) {
            self.machines.push(Machine { name: name.into(), box_: box_name.into(), state: "running".into(), synced_folder: "/vagrant".into() });
            true
        } else { false }
    }

    pub fn list(&self) -> &[Machine] { &self.machines }

    pub fn halt(&mut self, name: &str) -> bool {
        if let Some(m) = self.machines.iter_mut().find(|m| m.name == name) { m.state = "poweroff".into(); true } else { false }
    }

    pub fn destroy(&mut self, name: &str) -> bool {
        if let Some(pos) = self.machines.iter().position(|m| m.name == name) { self.machines.remove(pos); true } else { false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn up_halt_destroy() {
        let mut v = Model::new();
        assert!(v.up("default", "ubuntu/focal64"));
        assert!(v.halt("default"));
        assert!(v.destroy("default"));
        assert!(v.list().is_empty());
    }
}
