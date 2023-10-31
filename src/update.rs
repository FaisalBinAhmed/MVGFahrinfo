use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub async fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('r') => {
            app.update_departures().await;
            app.should_redraw = true;
        }
        KeyCode::Down => {
            app.increment_station();
            app.should_redraw = true;
        }
        KeyCode::Up => {
            app.decrement_station();
            app.should_redraw = true;
        }
        KeyCode::Enter => {
            app.select_station().await;
            app.should_redraw = true;
        }
        KeyCode::Tab => {
            app.toggle_tabs();
            app.should_redraw = true;
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        _ => {
            // todo: pass the key event
        }
    }
}
