use std::io::{self, Stdout};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame, Terminal,
};

use rusttop::system::ram::ram::RAM;

/// Convierte KiB (lo que reporta /proc/meminfo) a GiB solo para mostrar.
fn kib_a_gib(kib: u64) -> f64 {
    kib as f64 / 1024.0 / 1024.0
}

/// Proporción segura para Gauge: nunca divide entre 0 y siempre queda en 0.0..=1.0.
fn proporcion(usado: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        (usado as f64 / total as f64).clamp(0.0, 1.0)
    }
}

pub fn inicializar_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    Terminal::new(CrosstermBackend::new(stdout))
}

pub fn restaurar_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()
}

pub fn ejecutar_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ram: &RAM,
) -> io::Result<()> {
    terminal.draw(|frame| {
        let area = frame.area();
        let bloque = Block::default().title("RUSTTOP").borders(Borders::ALL);
        let interior = bloque.inner(area);
        frame.render_widget(bloque, area);

        let vertical = Layout::vertical([Constraint::Percentage(35), Constraint::Percentage(65)])
            .split(interior);
        frame.render_widget(Block::default().borders(Borders::BOTTOM), vertical[0]);

        let horizontal = Layout::horizontal([
            Constraint::Percentage(32),
            Constraint::Percentage(36),
            Constraint::Percentage(32),
        ])
        .split(vertical[1]);

        // Primero se dibuja el borde y luego el contenido dentro de su área interior,
        // así el widget ya no pisa el borde derecho.
        let bloque_ram = Block::default().borders(Borders::RIGHT);
        let interior_ram = bloque_ram.inner(horizontal[0]);
        frame.render_widget(bloque_ram, horizontal[0]);
        ram_widget(frame, ram, interior_ram);

        frame.render_widget(Block::default().borders(Borders::RIGHT), horizontal[1]);

        // Pendiente: disco y red.
        let _disco_y_red = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(horizontal[2]);
    })?;

    Ok(())
}

fn ram_widget(frame: &mut Frame, ram: &RAM, area: Rect) {
    let filas = Layout::vertical([Constraint::Fill(1); 3]).split(area);

    fila_uso(frame, "RAM", ram.get_used(), ram.get_total(), filas[0]);
    fila_uso(frame, "SWAP", ram.get_swap_used(), ram.get_swap_total(), filas[1]);

    let info = Paragraph::new(vec![
        Line::from(format!("Total: {:.2} GiB", kib_a_gib(ram.get_total()))),
        Line::from(format!("Available: {:.2} GiB", kib_a_gib(ram.get_available()))),
        Line::from(format!("Used: {:.2} GiB", kib_a_gib(ram.get_used()))),
        Line::from(format!("Cache: {:.2} GiB", kib_a_gib(ram.get_cache()))),
    ])
    .block(Block::default().title("Memory Info"));

    frame.render_widget(info, filas[2]);
}

/// Fila con título, gauge (75%) y texto "usado/total" (25%).
fn fila_uso(frame: &mut Frame, titulo: &str, usado: u64, total: u64, area: Rect) {
    let bloque = Block::default().title(titulo).borders(Borders::BOTTOM);
    let interior = bloque.inner(area);
    frame.render_widget(bloque, area);

    let columnas = Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(interior);

    let gauge = Gauge::default().ratio(proporcion(usado, total));
    let texto = Paragraph::new(Line::from(format!(
        "{:.2}/{:.2} GiB",
        kib_a_gib(usado),
        kib_a_gib(total)
    )));

    frame.render_widget(gauge, columnas[0]);
    frame.render_widget(texto, columnas[1]);
}