use std::io;
use std::io::Stdout;
use crossterm::{execute, terminal::{EnterAlternateScreen, enable_raw_mode}};
use ratatui::{layout::Rect, widgets::Gauge};
use ratatui::{
    Terminal, backend::CrosstermBackend, Frame, layout::{
        self, Constraint, Layout
    }, text::Line, widgets::{Block, Borders, Paragraph},
};

use crate::system::ram::ram::RAM;

pub fn inicializar_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    return Ok(terminal);
}

pub fn ejecutar_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>, ram: &RAM) -> io::Result<()> {

    terminal.draw(|frame| {
        let area = frame.area();
        let block = Block::default()
            .title("RUSTOP")
            .borders(Borders::ALL);
        let inner_central = block.inner(area);

        frame.render_widget(block, area);

        let layout_vertical = Layout::default().direction(layout::Direction::Vertical)
        .constraints(Constraint::from_percentages([35, 65])).split(inner_central);
        
        let block_vertical0 = Block::default().borders(Borders::BOTTOM);
        let inner_horizontal = layout_vertical[1];

        frame.render_widget(block_vertical0, layout_vertical[0]);

        let layout_horizontal = Layout::default().direction(layout::Direction::Horizontal)
        .constraints(Constraint::from_percentages([32, 36,32])).split(inner_horizontal);

        let block_horirizontal0 = Block::default().borders(Borders::RIGHT);
        let block_horirizontal1 = Block::default().borders(Borders::RIGHT);
        let inner_recuadrofinal = layout_horizontal[2];

        frame.render_widget(block_horirizontal0, layout_horizontal[0]);
        ram_widget(frame, ram, &layout_horizontal[0]);
        frame.render_widget(block_horirizontal1, layout_horizontal[1]);

        let layout_disk_network = Layout::default().direction(layout::Direction::Vertical)
        .constraints(Constraint::from_percentages([50, 50])).split(inner_recuadrofinal);

        let block_disk = Block::default();
        let block_network = Block::default();

        frame.render_widget(block_disk, layout_disk_network[0]);
        frame.render_widget(block_network, layout_disk_network[1]);
    })?;

    return Ok(());
}

fn ram_widget(frame: &mut Frame, ram: &RAM, inner: &Rect) {

    let layout = Layout::default().direction(layout::Direction::Vertical)
    .constraints(Constraint::from_percentages([33, 31 ,33])). split(*inner);

    let block_ram = Block::default().title("RAM").borders(Borders::BOTTOM);
    let inner_ram = block_ram.inner(layout[0]);

    frame.render_widget(block_ram, layout[0]);

    let layout_ram = Layout::default().direction(layout::Direction::Horizontal)
    .constraints(Constraint::from_percentages([75, 25])).split(inner_ram);

    let block_gauge_ram = Block::default();
    let block_text_ram = Block::default();

    let ratio_ram = ram.get_used() as f64 / ram.get_total() as f64;

    let gauge_ram = Gauge::default().ratio(ratio_ram).block(block_gauge_ram);
    let paragraph_ram = Paragraph::new(
        Line::from(format!("{:.2}/{:.2} GB", ram.get_used(), ram.get_total()))
    ).block(block_text_ram);

    frame.render_widget(gauge_ram, layout_ram[0]);
    frame.render_widget(paragraph_ram, layout_ram[1]);

    let block_swap = Block::default().title("SWAP").borders(Borders::BOTTOM);
    let inner_swap = block_swap.inner(layout[1]);

    frame.render_widget(block_swap, layout[1]);

    let layout_swap = Layout::default().direction(layout::Direction::Horizontal)
    .constraints(Constraint::from_percentages([75, 25])).split(inner_swap);

    let block_gauge_swap = Block::default();
    let block_text_swap = Block::default();

    let ratio_swap = ram.get_swap_used() as f64 / ram.get_swap_total() as f64;

    let gauge_swap = Gauge::default().ratio(ratio_swap).block(block_gauge_swap);
    let paragraph_swap = Paragraph::new(
        Line::from(format!("{:.2}/{:.2} GB", ram.get_swap_used(), ram.get_swap_total()))
    ).block(block_text_swap);

    frame.render_widget(gauge_swap, layout_swap[0]);
    frame.render_widget(paragraph_swap, layout_swap[1]);

    let block_paragraph_info = Block::default().title("Memory Info");

    let paragraph_info = Paragraph::new(vec![
        Line::from(format!("Total: {:.2} GB", ram.get_total())),
        Line::from(format!("Available: {:.2} GB", ram.get_available())),
        Line::from(format!("Used: {:.2} GB", ram.get_used())),
        Line::from(format!("Cache: {:.2} GB", ram.get_cache()))
    ]).block(block_paragraph_info);

    frame.render_widget(paragraph_info, layout[2]);
}

