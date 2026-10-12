use super::cpu::CPU;
use std::{fs, io};

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

fn parse_cpu_line(line: &str) -> io::Result<CPU> {
    let mut fields = line.split_whitespace();
    if fields.next() != Some("cpu") {
        return Err(invalid("la primera línea de CPU no es el agregado `cpu`"));
    }

    let values: Vec<u64> = fields
        .map(|field| {
            field
                .parse::<u64>()
                .map_err(|error| invalid(format!("contador de CPU inválido: {error}")))
        })
        .collect::<io::Result<_>>()?;

    if values.len() < 4 {
        return Err(invalid("la línea agregada de CPU debe incluir al menos 4 contadores"));
    }

    let value = |index| values.get(index).copied().unwrap_or(0);
    Ok(CPU::new(
        value(0),
        value(1),
        value(2),
        value(3),
        value(4),
        value(5),
        value(6),
        value(7),
        value(8),
        value(9),
    ))
}

pub fn read() -> io::Result<CPU> {
    let contents = fs::read_to_string("/proc/stat")?;
    let line = contents
        .lines()
        .find(|line| line.starts_with("cpu "))
        .ok_or_else(|| invalid("no se encontró el agregado de CPU en /proc/stat"))?;
    parse_cpu_line(line)
}

fn calculate_usage(previous: &CPU, current: &CPU) -> f64 {
    let total_delta = current.get_total().saturating_sub(previous.get_total());
    let previous_idle = previous.get_idle().saturating_add(previous.get_iowait());
    let current_idle = current.get_idle().saturating_add(current.get_iowait());
    let idle_delta = current_idle.saturating_sub(previous_idle);
    if total_delta == 0 {
        0.0
    } else {
        (total_delta.saturating_sub(idle_delta) as f64 / total_delta as f64) * 100.0
    }
}

pub fn update(cpu: &mut CPU) -> io::Result<()> {
    let mut current = read()?;
    current.set_usage(calculate_usage(cpu, &current));
    *cpu = current;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_linux_cpu_counters_and_omits_optional_fields_as_zero() {
        let cpu = parse_cpu_line("cpu  10 2 3 20").unwrap();

        assert_eq!(cpu.get_user(), 10);
        assert_eq!(cpu.get_nice(), 2);
        assert_eq!(cpu.get_system(), 3);
        assert_eq!(cpu.get_idle(), 20);
        assert_eq!(cpu.get_guest(), 0);
    }

    #[test]
    fn rejects_incomplete_cpu_line() {
        assert_eq!(
            parse_cpu_line("cpu 10 2 3").unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn calculates_usage_from_two_samples() {
        let previous = CPU::new(100, 0, 0, 100, 0, 0, 0, 0, 0, 0);
        let current = CPU::new(120, 0, 0, 110, 0, 0, 0, 0, 0, 0);

        assert!((calculate_usage(&previous, &current) - 66.666_666_666_7).abs() < 0.0001);
    }
}