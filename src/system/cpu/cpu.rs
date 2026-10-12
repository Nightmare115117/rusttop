#[derive(Debug, Clone, PartialEq)]
pub struct CPU {
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
    steal: u64,
    guest: u64,
    guest_nice: u64,
    usage: f64,
}

impl CPU {
    pub fn new(
        user: u64,
        nice: u64,
        system: u64,
        idle: u64,
        iowait: u64,
        irq: u64,
        softirq: u64,
        steal: u64,
        guest: u64,
        guest_nice: u64,
    ) -> CPU {
        CPU {
            user,
            nice,
            system,
            idle,
            iowait,
            irq,
            softirq,
            steal,
            guest,
            guest_nice,
            usage: 0.0,
        }
    }

    pub fn get_user(&self) -> u64 {
        self.user
    }

    pub fn get_nice(&self) -> u64 {
        self.nice
    }

    pub fn get_system(&self) -> u64 {
        self.system
    }

    pub fn get_idle(&self) -> u64 {
        self.idle
    }

    pub fn get_iowait(&self) -> u64 {
        self.iowait
    }

    pub fn get_irq(&self) -> u64 {
        self.irq
    }

    pub fn get_softirq(&self) -> u64 {
        self.softirq
    }

    pub fn get_steal(&self) -> u64 {
        self.steal
    }

    pub fn get_guest(&self) -> u64 {
        self.guest
    }

    pub fn get_guest_nice(&self) -> u64 {
        self.guest_nice
    }

    pub fn get_usage(&self) -> f64 {
        self.usage
    }

    pub fn get_total(&self) -> u64 {
        self.user
            .saturating_add(self.nice)
            .saturating_add(self.system)
            .saturating_add(self.idle)
            .saturating_add(self.iowait)
            .saturating_add(self.irq)
            .saturating_add(self.softirq)
            .saturating_add(self.steal)
    }

    pub(crate) fn set_usage(&mut self, usage: f64) {
        self.usage = usage.clamp(0.0, 100.0);
    }
}