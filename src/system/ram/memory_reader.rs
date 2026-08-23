use super::ram::RAM;

use std::{
    fs, io
};

pub fn read() -> Result<RAM, io::Error> {

    let readed: String = fs::read_to_string("/proc/meminfo")?;

    let mut total: f32 = 0.0;
    let mut available: f32 = 0.0;
    let mut cache: f32 = 0.0;
    let mut swap_total: f32 = 0.0;
    let mut swap_free: f32 = 0.0;

    let mut found = 0;

    for line in readed.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts[0] {
            "MemTotal:" => {
                total = parts[1].parse::<f32>().unwrap() * 1024.0 / 1_000_000_000.0;
                found += 1;
            }
            "MemAvailable:" => {
                available = parts[1].parse::<f32>().unwrap() * 1024.0 / 1_000_000_000.0;
                found += 1;
            }
            "Cached:" => {
                cache = parts[1].parse::<f32>().unwrap() * 1024.0 / 1_000_000_000.0;
                found += 1;
            }
            "SwapTotal:" => {
                swap_total = parts[1].parse::<f32>().unwrap() * 1024.0 / 1_000_000_000.0;
                found += 1;
            }
            "SwapFree:" => {
                swap_free = parts[1].parse::<f32>().unwrap() * 1024.0 / 1_000_000_000.0;
                found += 1;
            }
            _ => {}
        }

        if found == 5 {
            break;
        }
    }

    Ok(RAM::new(
        total,
        cache,
        available,
        swap_total,
        swap_total - swap_free
    ))
}

pub fn update(ram: &mut RAM) -> Result<(), io::Error> {

    let readed: String = fs::read_to_string("/proc/meminfo")?;

    let mut found = 0;

    for line in readed.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts[0] {
            "MemAvailable:" => {
                ram.set_available(
                    parts[1].parse::<f32>().unwrap() * 1024.0 / 1_000_000_000.0
                );
                found += 1;
            }
            "Cached:" => {
                ram.set_cache(
                    parts[1].parse::<f32>().unwrap() * 1024.0 / 1_000_000_000.0
                );
                found += 1;
            }
            "SwapTotal:" => {
                ram.set_swap_total(
                    parts[1].parse::<f32>().unwrap() * 1024.0 / 1_000_000_000.0
                );
                found += 1;
            }
            "SwapFree:" => {
                let swap_free =
                    parts[1].parse::<f32>().unwrap() * 1024.0 / 1_000_000_000.0;

                ram.set_swap_used(ram.get_swap_total() - swap_free);
                found += 1;
            }
            _ => {}
        }

        if found == 4 {
            break;
        }
    }

    Ok(())
}