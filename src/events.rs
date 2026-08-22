use crate::app::App;
use std::io;
use crossterm::{
    event::{self, Event, KeyCode}
};

pub fn activar_eventos(app: &mut App) -> io::Result<()>{
    if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    app.set_running(false);
                } else if key.code == KeyCode::Char('h') {
                    app.set_app_status("Hola con H".to_string());
            }
        }
    }

    return Ok(());
}