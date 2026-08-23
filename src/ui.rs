use std::io;
use std::io::Stdout;
use crossterm::{execute, terminal::{EnterAlternateScreen, enable_raw_mode}};
use ratatui:: {
    Terminal, backend::CrosstermBackend, text::{Line}, widgets::{Block, Borders, Paragraph},
};

pub fn inicializar_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    return Ok(terminal);
}

pub fn ejecutar_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &str) -> io::Result<()> {

    terminal.draw(|frame| {
        let area = frame.area();
        let block = Block::default()
            .title("RUSTOP")
            .borders(Borders::ALL);

        let paragraph = Paragraph::new(
                vec![
                    Line::from(app),
                    Line::from("\n"),
                    Line::from("Press q to quit."),
                ]
            )
            .block(block);

            frame.render_widget(paragraph, area);
    })?;

    return Ok(());
}//