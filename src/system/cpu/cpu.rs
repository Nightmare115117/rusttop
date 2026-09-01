pub struct CPU {

    user: i64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
    steal: u64,
    guest: u64,
    guest_nice: u64,

    usage:f64
}

impl CPU {

    pub fn new() -> CPU {
        
    }

    pub fn get_user(&self) -> u64 {
        return self.user;
    }

    pub fn get_nice(&self) -> u64 {
        return self.nice;
    }

    pub fn get_system(&self) -> u64 {
        return self.system;
    }

    pub fn get_idle(&self) -> u64 {
        return self.idle;
    }

    pub fn get_iowait(&self) -> u64 {
        return self.iowait;
    }

    pub fn get_irq(&self) -> u64 {
        return self.irq;
    }

    pub fn get_softirq(&self) -> u64 {
        return self.softirq;
    }

    pub fn get_steal(&self) -> u64 {
        return self.steal;
    }

    pub fn get_guest(&self) -> u64 {
        return self.guest;
    }

    pub fn get_guest_nice(&self) -> u64 {
        return self.guest_nice;
    }

    fn get_total(&self) -> u64 {
        return self.user + self.nice + self.system + self.idle + self.iowait + self.irq + self.softirq + self.steal + self.guest + self.guest_nice;
    }

    
}