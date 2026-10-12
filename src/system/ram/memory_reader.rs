use super::ram::RAM;

use std::{fs, io};

fn invalid(msg: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, msg.into())
}

// Recibe la línea ya separada y devuelve el valor crudo en KiB.
fn parse_kib(parts: &[&str], name: &str) -> Result<u64, io::Error> {
    parts
        .get(1)
        .ok_or_else(|| invalid(format!("{name}: falta el valor")))?
        .parse::<u64>()
        .map_err(|e| invalid(format!("{name}: {e}")))
}

fn missing(name: &str) -> io::Error {
    invalid(format!("{name} no aparece en /proc/meminfo"))
}

pub fn read() -> Result<RAM, io::Error> {
    let contents: String = fs::read_to_string("/proc/meminfo")?;

    let mut total: Option<u64> = None;
    let mut available: Option<u64> = None;
    let mut cache: Option<u64> = None;
    let mut swap_total: Option<u64> = None;
    let mut swap_free: Option<u64> = None;

    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let Some(key) = parts.first() else {
            continue; // línea vacía
        };

        match *key {
            "MemTotal:" => total = Some(parse_kib(&parts, "MemTotal")?),
            "MemAvailable:" => available = Some(parse_kib(&parts, "MemAvailable")?),
            "Cached:" => cache = Some(parse_kib(&parts, "Cached")?),
            "SwapTotal:" => swap_total = Some(parse_kib(&parts, "SwapTotal")?),
            "SwapFree:" => swap_free = Some(parse_kib(&parts, "SwapFree")?),
            _ => {}
        }

        if [total, available, cache, swap_total, swap_free]
            .iter()
            .all(Option::is_some)
        {
            break;
        }
    }

    let total = total.ok_or_else(|| missing("MemTotal"))?;
    let available = available.ok_or_else(|| missing("MemAvailable"))?;
    let cache = cache.ok_or_else(|| missing("Cached"))?;
    let swap_total = swap_total.ok_or_else(|| missing("SwapTotal"))?;
    let swap_free = swap_free.ok_or_else(|| missing("SwapFree"))?;

    RAM::new(
        total,
        cache,
        available,
        swap_total,
        swap_total.saturating_sub(swap_free),
    )
    .map_err(invalid)
}

pub fn update(ram: &mut RAM) -> Result<(), io::Error> {
    *ram = read()?;
    Ok(())
}