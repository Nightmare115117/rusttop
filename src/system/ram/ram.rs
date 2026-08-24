pub struct RAM {
    total: f32,
    cache: f32,
    available: f32,
    swap_total: f32,
    swap_used: f32
}

impl RAM {

    pub fn new(total: f32, cache: f32, available: f32, swap_total: f32, swap_used: f32) -> RAM {
        RAM {
            total: total,
            cache: cache,
            available: available,
            swap_total: swap_total,
            swap_used: swap_used
        }
    }

    pub fn get_total (&self) -> f32 {
        return self.total;
    }

    #[allow(dead_code)]
    pub fn get_used (&self) -> f32 {
        return self.total - self.cache - self.available;
    }

    #[allow(dead_code)]
    pub fn get_cache(&self) -> f32 {
        return self.cache;
    }

    #[allow(dead_code)]
    pub fn get_available(&self) -> f32 {
        return self.available;
    }

    pub fn get_swap_total (&self) -> f32 {
        return self.swap_total;
    }

    pub fn get_swap_used (&self) -> f32 {
        return self.swap_used;
    }

    pub fn set_cache (&mut self, cache: f32) {
        if cache >= 0.0 { 
            self.cache = cache;
        }
    }    

    pub fn set_available (&mut self, available: f32) {
        if available >= 0.0 {  
            self.available = available;
        }
    }

    pub fn set_swap_total (&mut self, swap_total: f32) {
        if (swap_total >= 0.0) && (swap_total >= self.swap_used) {
            self.swap_total = swap_total;
        }
    }

    pub fn set_swap_used (&mut self, swap_used: f32) {
        if (swap_used >= 0.0) && (swap_used <= self.swap_total) {
            self.swap_used = swap_used;
        }
    }
}