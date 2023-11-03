use std::sync::mpsc;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::widgets::ListState;

use crate::{
    app::{App, AppMode},
    event::Event,
};

pub async fn update(app: &mut App, key_event: KeyEvent) {
    match app.app_mode {
        AppMode::Normal => match key_event.code {
            KeyCode::Char('q') => app.quit(),
            KeyCode::Char('s') => {
                app.app_mode = AppMode::Search;
                app.should_redraw = true;
            }
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
        },
        AppMode::Search => match key_event.code {
            KeyCode::Char('q') => app.quit(), //this is duplicated code, we can refactor it later
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit()
                }
            }
            KeyCode::Enter => {
                app.select_searched_station().await;
                app.should_redraw = true;
            }
            KeyCode::Char(to_insert) => {
                app.search_scroll_state = ListState::default();
                app.enter_char(to_insert);
                app.should_redraw = true;
            }
            KeyCode::Backspace => {
                app.search_scroll_state = ListState::default();
                app.delete_char();
                app.should_redraw = true;
            }
            KeyCode::Down => {
                app.scroll_down();
                app.should_redraw = true;
            }
            KeyCode::Up => {
                app.scroll_up();
                app.should_redraw = true;
            }
            KeyCode::Left => {
                app.move_cursor_left();
                app.should_redraw = true;
            }
            KeyCode::Right => {
                app.move_cursor_right();
                app.should_redraw = true;
            }
            KeyCode::Esc => {
                app.app_mode = AppMode::Normal;
                app.should_redraw = true;
            }
            _ => {}
        },
    }
}

// this lets us mutate the app state without having to pass a mutable reference and blocking the main ui/event thread or having to use a mutex
// we simulate the refresh command by sending a key event to the event handler
// the event handler has a mutable reference to the app and can mutate it
pub fn initiate_auto_refresh(sender: mpsc::Sender<Event>) {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
            // println!("sending refresh event");
            let _ = sender.send(Event::Key(KeyEvent::from(KeyCode::Char('r'))));
        }
    });
}
