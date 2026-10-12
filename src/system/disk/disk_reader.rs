use super::disk::Disk;
use std::{fs, io};

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

fn parse_disk_line(line: &str) -> io::Result<Disk> {
    let fields: Vec<&str> = line.split_whitespace().collect();
    if fields.len() < 14 {
        return Err(invalid("línea incompleta en /proc/diskstats"));
    }

    let number = |index: usize| {
        fields[index]
            .parse::<u64>()
            .map_err(|error| invalid(format!("contador de disco inválido: {error}")))
    };

    Ok(Disk::new(
        fields[2].to_string(),
        number(3)?,
        number(5)?,
        number(6)?,
        number(7)?,
        number(9)?,
        number(10)?,
        number(12)?,
    ))
}

pub fn read() -> io::Result<Vec<Disk>> {
    let contents = fs::read_to_string("/proc/diskstats")?;
    contents
        .lines()
        .map(parse_disk_line)
        .collect::<io::Result<Vec<_>>>()
}

pub fn update(disks: &mut Vec<Disk>) -> io::Result<()> {
    *disks = read()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_disk_counters() {
        let disk = parse_disk_line("8 0 sda 10 0 20 30 40 0 50 60 0 70 80").unwrap();

        assert_eq!(disk.get_name(), "sda");
        assert_eq!(disk.get_reads_completed(), 10);
        assert_eq!(disk.get_read_bytes(), 20 * 512);
        assert_eq!(disk.get_writes_completed(), 40);
        assert_eq!(disk.get_written_bytes(), 50 * 512);
        assert_eq!(disk.get_io_time_ms(), 70);
    }
}
