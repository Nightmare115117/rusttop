mod app;
mod ui;
mod events;
mod system;
use std::io;
use crate::system::ram::{
    ram::RAM, memory_reader
};
use app::App;
use crossterm::{
    execute, terminal::{LeaveAlternateScreen, disable_raw_mode}
};

fn main() -> io::Result<()> {
    let mut app = App::new("Hola desde Terminal".to_string(), true);
    let mut terminal = ui::inicializar_terminal()?;
    let mut ram: RAM = memory_reader::read()?;
    while app.is_running() {
        
        memory_reader::update(&mut ram)?;
        ui::ejecutar_terminal(&mut terminal, &ram)?;
        events::activar_eventos(&mut app)?;
    }

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;

    Ok(())
}