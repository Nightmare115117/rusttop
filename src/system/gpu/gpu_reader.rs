use super::gpu::GPU;
use std::{
    fs,
    io::{self, ErrorKind},
    path::Path,
};

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(ErrorKind::InvalidData, message.into())
}

fn read_optional(path: &Path) -> io::Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(value) => Ok(Some(value.trim().to_string())),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

fn parse_hex_id(value: Option<String>, fallback: &str) -> io::Result<String> {
    let Some(value) = value else {
        return Ok(fallback.to_string());
    };
    let value = value.strip_prefix("0x").unwrap_or(&value);
    u16::from_str_radix(value, 16)
        .map(|id| format!("{id:04x}"))
        .map_err(|error| invalid(format!("identificador de GPU inválido: {error}")))
}

fn read_metric(device_path: &Path, filename: &str) -> io::Result<Option<u64>> {
    let Some(value) = read_optional(&device_path.join(filename))? else {
        return Ok(None);
    };
    value
        .parse::<u64>()
        .map(Some)
        .map_err(|error| invalid(format!("{filename}: {error}")))
}

fn vendor_name(vendor: &str) -> &'static str {
    match vendor {
        "8086" => "Intel",
        "10de" => "NVIDIA",
        "1002" => "AMD",
        _ => "GPU",
    }
}

fn parse_card_name(name: &str) -> bool {
    name.strip_prefix("card")
        .is_some_and(|number| !number.is_empty() && number.bytes().all(|b| b.is_ascii_digit()))
}

pub fn read() -> io::Result<Vec<GPU>> {
    let drm_path = Path::new("/sys/class/drm");
    let entries = match fs::read_dir(drm_path) {
        Ok(entries) => entries,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error),
    };

    let mut gpus = Vec::new();
    for entry in entries {
        let entry = entry?;
        let card = entry.file_name();
        let card = card.to_string_lossy();
        if !parse_card_name(&card) {
            continue;
        }

        let device_path = entry.path().join("device");
        let vendor = parse_hex_id(read_optional(&device_path.join("vendor"))?, "0000")?;
        let product = parse_hex_id(read_optional(&device_path.join("device"))?, "0000")?;
        let name = if vendor == "0000" {
            format!("GPU {product}")
        } else {
            format!("{} {product}", vendor_name(&vendor))
        };

        let utilization = read_metric(&device_path, "gpu_busy_percent")?
            .map(|value| value as f64);
        let memory_used = read_metric(&device_path, "mem_info_vram_used")?;
        let memory_total = read_metric(&device_path, "mem_info_vram_total")?;

        gpus.push(GPU::new(
            card.into_owned(),
            name,
            utilization,
            memory_used,
            memory_total,
        ));
    }
    gpus.sort_by(|left, right| left.get_device().cmp(right.get_device()));
    Ok(gpus)
}

pub fn update(gpus: &mut Vec<GPU>) -> io::Result<()> {
    *gpus = read()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_card_devices_but_not_connectors() {
        assert!(parse_card_name("card0"));
        assert!(!parse_card_name("card0-DP-1"));
        assert!(!parse_card_name("renderD128"));
    }

    #[test]
    fn maps_common_gpu_vendors() {
        assert_eq!(vendor_name("8086"), "Intel");
        assert_eq!(vendor_name("10de"), "NVIDIA");
        assert_eq!(vendor_name("1002"), "AMD");
        assert_eq!(vendor_name("ffff"), "GPU");
    }
}
