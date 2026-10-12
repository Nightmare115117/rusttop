#[derive(Debug, Clone, PartialEq)]
pub struct GPU {
    device: String,
    name: String,
    utilization_percent: Option<f64>,
    memory_used_bytes: Option<u64>,
    memory_total_bytes: Option<u64>,
}

impl GPU {
    pub fn new(
        device: String,
        name: String,
        utilization_percent: Option<f64>,
        memory_used_bytes: Option<u64>,
        memory_total_bytes: Option<u64>,
    ) -> GPU {
        GPU {
            device,
            name,
            utilization_percent: utilization_percent.map(|value| value.clamp(0.0, 100.0)),
            memory_used_bytes,
            memory_total_bytes,
        }
    }

    pub fn get_device(&self) -> &str {
        &self.device
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_utilization_percent(&self) -> Option<f64> {
        self.utilization_percent
    }

    pub fn get_memory_used_bytes(&self) -> Option<u64> {
        self.memory_used_bytes
    }

    pub fn get_memory_total_bytes(&self) -> Option<u64> {
        self.memory_total_bytes
    }
}
