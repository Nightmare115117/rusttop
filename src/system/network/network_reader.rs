use super::network::Network;
use std::{fs, io};

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

fn parse_network_line(line: &str) -> io::Result<Network> {
    let (name, counters) = line
        .rsplit_once(':')
        .ok_or_else(|| invalid("línea inválida en /proc/net/dev"))?;
    let fields: Vec<&str> = counters.split_whitespace().collect();
    if fields.len() < 16 {
        return Err(invalid("línea incompleta en /proc/net/dev"));
    }

    let number = |index: usize| {
        fields[index]
            .parse::<u64>()
            .map_err(|error| invalid(format!("contador de red inválido: {error}")))
    };

    Ok(Network::new(
        name.trim().to_string(),
        number(0)?,
        number(1)?,
        number(2)?,
        number(8)?,
        number(9)?,
        number(10)?,
    ))
}

pub fn read() -> io::Result<Vec<Network>> {
    let contents = fs::read_to_string("/proc/net/dev")?;
    contents
        .lines()
        .skip(2)
        .filter(|line| !line.trim().is_empty())
        .map(parse_network_line)
        .collect::<io::Result<Vec<_>>>()
}

pub fn update(networks: &mut Vec<Network>) -> io::Result<()> {
    *networks = read()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_interface_counters() {
        let network =
            parse_network_line("  eth0: 100 2 3 0 0 0 0 0 200 4 5 0 0 0 0 0").unwrap();

        assert_eq!(network.get_name(), "eth0");
        assert_eq!(network.get_received_bytes(), 100);
        assert_eq!(network.get_received_packets(), 2);
        assert_eq!(network.get_receive_errors(), 3);
        assert_eq!(network.get_transmitted_bytes(), 200);
        assert_eq!(network.get_transmitted_packets(), 4);
        assert_eq!(network.get_transmit_errors(), 5);
    }
}
