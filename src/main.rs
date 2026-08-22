mod app;
mod ui;
mod events;
use std::io;
use app::App;
use crossterm::{
    execute, terminal::{LeaveAlternateScreen, disable_raw_mode}
};

fn main() -> io::Result<()> {
    let mut app = App::new("Hola desde Terminal".to_string(), true);
    let mut terminal = ui::inicializar_terminal()?;

    while app.is_running() {
        
        ui::ejecutar_terminal(&mut terminal, app.get_app_status())?;
        events::activar_eventos(&mut app)?;
    }

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;

    Ok(())
}