# rusttop

## Español

`rusttop` es un monitor de recursos del sistema para terminal, escrito en Rust.
La interfaz de terminal actual muestra el uso de RAM y SWAP; los módulos de
CPU, discos, interfaces de red y GPU también incluyen modelos y readers para
consultar sus contadores.

### Requisitos

- Linux, para las fuentes de datos del sistema (`/proc` y `/sys`).
- Rust y Cargo.

### Ejecución

```sh
cargo run
```

Pulsa `q` para salir.

### Módulos de lectura

Cada recurso está organizado en `src/system/<recurso>/` con un modelo, un
reader y un `mod.rs`:

- `ram`: total, disponible, caché y SWAP, en KiB.
- `cpu`: contadores agregados de `/proc/stat`; `usage` se calcula entre dos
  muestras consecutivas mediante `cpu_reader::update`.
- `disk`: contadores de `/proc/diskstats`; los sectores reportados se
  convierten a bytes suponiendo sectores de 512 bytes.
- `network`: bytes, paquetes y errores de cada interfaz desde `/proc/net/dev`.
- `gpu`: dispositivos DRM de `/sys/class/drm`. La utilización y memoria VRAM
  son `Option` porque dependen del driver y pueden no estar disponibles.

Ejemplo de uso de un reader:

```rust
use rusttop::system::cpu::cpu_reader;

fn sample() -> std::io::Result<()> {
    let mut cpu = cpu_reader::read()?;
    cpu_reader::update(&mut cpu)?;
    Ok(())
}
```

El binario importa los módulos desde la librería `rusttop`; pueden reutilizarse
desde otros crates mediante `rusttop::system`.

## English

`rusttop` is a terminal-based system resource monitor written in Rust. Its
current terminal interface displays RAM and swap usage; CPU, disk, network
interface, and GPU modules also provide models and readers for querying their
counters.

### Requirements

- Linux, for system data sources (`/proc` and `/sys`).
- Rust and Cargo.

### Run

```sh
cargo run
```

Press `q` to exit.

### Reader modules

Each resource is organized under `src/system/<resource>/` with a model, a
reader, and a `mod.rs`:

- `ram`: total, available, cached, and swap memory, in KiB.
- `cpu`: aggregate `/proc/stat` counters; `usage` is calculated between two
  consecutive samples by `cpu_reader::update`.
- `disk`: `/proc/diskstats` counters; reported sectors are converted to bytes
  assuming 512-byte sectors.
- `network`: per-interface bytes, packets, and errors from `/proc/net/dev`.
- `gpu`: DRM devices from `/sys/class/drm`. Utilization and VRAM values are
  `Option`s because availability depends on the driver.

Reader usage example:

```rust
use rusttop::system::cpu::cpu_reader;

fn sample() -> std::io::Result<()> {
    let mut cpu = cpu_reader::read()?;
    cpu_reader::update(&mut cpu)?;
    Ok(())
}
```

The binary imports modules through the `rusttop` library; other crates can
reuse them through `rusttop::system`.
