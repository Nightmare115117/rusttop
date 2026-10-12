#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Network {
    name: String,
    received_bytes: u64,
    received_packets: u64,
    receive_errors: u64,
    transmitted_bytes: u64,
    transmitted_packets: u64,
    transmit_errors: u64,
}

impl Network {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        received_bytes: u64,
        received_packets: u64,
        receive_errors: u64,
        transmitted_bytes: u64,
        transmitted_packets: u64,
        transmit_errors: u64,
    ) -> Network {
        Network {
            name,
            received_bytes,
            received_packets,
            receive_errors,
            transmitted_bytes,
            transmitted_packets,
            transmit_errors,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_received_bytes(&self) -> u64 {
        self.received_bytes
    }

    pub fn get_received_packets(&self) -> u64 {
        self.received_packets
    }

    pub fn get_receive_errors(&self) -> u64 {
        self.receive_errors
    }

    pub fn get_transmitted_bytes(&self) -> u64 {
        self.transmitted_bytes
    }

    pub fn get_transmitted_packets(&self) -> u64 {
        self.transmitted_packets
    }

    pub fn get_transmit_errors(&self) -> u64 {
        self.transmit_errors
    }
}
