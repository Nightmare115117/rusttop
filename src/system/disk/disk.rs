#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Disk {
    name: String,
    reads_completed: u64,
    sectors_read: u64,
    read_time_ms: u64,
    writes_completed: u64,
    sectors_written: u64,
    write_time_ms: u64,
    io_time_ms: u64,
}

impl Disk {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        reads_completed: u64,
        sectors_read: u64,
        read_time_ms: u64,
        writes_completed: u64,
        sectors_written: u64,
        write_time_ms: u64,
        io_time_ms: u64,
    ) -> Disk {
        Disk {
            name,
            reads_completed,
            sectors_read,
            read_time_ms,
            writes_completed,
            sectors_written,
            write_time_ms,
            io_time_ms,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_reads_completed(&self) -> u64 {
        self.reads_completed
    }

    pub fn get_sectors_read(&self) -> u64 {
        self.sectors_read
    }

    pub fn get_read_bytes(&self) -> u64 {
        self.sectors_read.saturating_mul(512)
    }

    pub fn get_read_time_ms(&self) -> u64 {
        self.read_time_ms
    }

    pub fn get_writes_completed(&self) -> u64 {
        self.writes_completed
    }

    pub fn get_sectors_written(&self) -> u64 {
        self.sectors_written
    }

    pub fn get_written_bytes(&self) -> u64 {
        self.sectors_written.saturating_mul(512)
    }

    pub fn get_write_time_ms(&self) -> u64 {
        self.write_time_ms
    }

    pub fn get_io_time_ms(&self) -> u64 {
        self.io_time_ms
    }
}
