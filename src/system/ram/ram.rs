#[derive(Debug)]
pub struct RAM {
    total: u64,
    cache: u64,
    available: u64,
    swap_total: u64,
    swap_used: u64,
}

impl RAM {
    pub fn new(
        total: u64,
        cache: u64,
        available: u64,
        swap_total: u64,
        swap_used: u64,
    ) -> Result<RAM, String> {
        if cache > total {
            return Err("cache no puede ser mayor que total".to_string());
        }
        if available > total {
            return Err("available no puede ser mayor que total".to_string());
        }
        if swap_used > swap_total {
            return Err("swap_used no puede ser mayor que swap_total".to_string());
        }
        Ok(RAM { total, cache, available, swap_total, swap_used })
    }

    pub fn get_total(&self) -> u64 {
        self.total
    }

    #[allow(dead_code)]
    pub fn get_used(&self) -> u64 {
        self.total.saturating_sub(self.available)
    }

    #[allow(dead_code)]
    pub fn get_cache(&self) -> u64 {
        self.cache
    }

    #[allow(dead_code)]
    pub fn get_available(&self) -> u64 {
        self.available
    }

    pub fn get_swap_total(&self) -> u64 {
        self.swap_total
    }

    pub fn get_swap_used(&self) -> u64 {
        self.swap_used
    }

    pub fn set_cache(&mut self, cache: u64) -> Result<(), String> {
        if cache > self.total {
            return Err("cache no puede ser mayor que total".to_string());
        }
        self.cache = cache;
        Ok(())
    }

    pub fn set_available(&mut self, available: u64) -> Result<(), String> {
        if available > self.total {
            return Err("available no puede ser mayor que total".to_string());
        }
        self.available = available;
        Ok(())
    }

    pub fn set_swap_total(&mut self, swap_total: u64) -> Result<(), String> {
        if swap_total < self.swap_used {
            return Err("swap_total no puede ser menor que swap_used".to_string());
        }
        self.swap_total = swap_total;
        Ok(())
    }

    pub fn set_swap_used(&mut self, swap_used: u64) -> Result<(), String> {
        if swap_used > self.swap_total {
            return Err("swap_used no puede ser mayor que swap_total".to_string());
        }
        self.swap_used = swap_used;
        Ok(())
    }
}