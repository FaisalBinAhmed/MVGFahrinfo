#[allow(unused, dead_code)]
use anyhow::Result; //to avoid writing the error type

use api::get_departures;
use crossterm::{
    event::{
        self,
        Event::Key,
        KeyCode::{self, Char},
        KeyEventKind,
    },
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{
        Alignment, Constraint, CrosstermBackend, Direction, Layout, Rect, Stylize, Terminal,
    },
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph},
};
// use tokio::{runtime::Handle, task};

use std::io::stderr;

mod components; //to import the components module
use components::static_widgets; // to avoid typing components::static_widgets:: every time
mod api;

pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>; // alias for the frame type

struct StatefulList {
    state: ListState,
    items: Vec<api::Station>,
}

impl StatefulList {
    async fn new() -> Self {
        Self {
            state: ListState::default(),
            items: api::get_stations().await.unwrap_or_else(|_| vec![]),
        }
    }
    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub struct App {
    counter: i64,
    should_quit: bool,
    stations: Vec<api::Station>,
    // station_list: StatefulList,
    show_popup: bool,
    progress: u16,
    fetching: bool,
    selected_station: Option<api::Station>,
    departures: Vec<api::DepartureInfo>,
}

impl App {
    async fn new() -> Self {
        Self {
            counter: 0,
            should_quit: false,
            stations: api::get_stations().await.unwrap_or_else(|_| vec![]),
            // station_list: StatefulList::new().await,
            show_popup: false,
            progress: 0,
            fetching: true,
            selected_station: None,
            departures: vec![],
        }
    }
    fn quit(&mut self) {
        self.should_quit = true;
    }
    fn increment_station(&mut self) {
        self.counter += 1;
    }

    fn decrement_station(&mut self) {
        if self.counter > 0 {
            self.counter -= 1;
        }
    }

    async fn update_departures(&mut self) {
        if let Some(station) = &self.selected_station {
            self.departures = get_departures(&station.id).await.unwrap_or_else(|_| vec![]);
        }
    }

    async fn select_station(&mut self) {
        self.show_popup = !self.show_popup; //temp
                                            // self.selected_station = Some(self.stations[self.counter as usize].clone());

        self.update_departures().await;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // initialize_panic_handler();
    startup()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    terminal.clear()?;

    let mut app = App::new().await;

    // if let stations = app.stations? {
    //     println!("Stations: {:#?}", stations[0]);
    // }

    // get_departures("de:09162:6").await?;

    // tokio::spawn(async {
    //     update_stations(&mut app).await;
    // });
    // update_stations(&mut app).await;

    loop {
        // application render
        terminal.draw(|f| {
            ui(&app, f);
        })?;
        // application update
        update(&mut app).await?;

        // application exit
        if app.should_quit {
            break;
        }
    }
    shutdown()?;

    return Ok(());
}

fn startup() -> Result<()> {
    stderr().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    return Ok(());
}

fn shutdown() -> Result<()> {
    stderr().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    return Ok(());
}

fn ui(app: &App, f: &mut Frame<'_>) {
    let paragraph = Paragraph::new(format!("Counter: {}", app.counter))
        .block(static_widgets::get_app_border())
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    f.render_widget(paragraph, chunks[0]);

    let itemlist = components::station_list::get_station_list_widget(app);

    f.render_widget(itemlist, chunks[1]);

    f.render_widget(
        Paragraph::new(format!("Press p to select station, q to quit app"))
            .light_red()
            .block(Block::default().borders(Borders::TOP))
            .alignment(Alignment::Center),
        chunks[2],
    );

    if app.show_popup {
        draw_popup(f, app)
    }
}

fn draw_popup(f: &mut Frame<'_>, app: &App) {
    // let popup_layout = Layout::default()
    //     .direction(Direction::Vertical)
    //     .constraints([
    //         Constraint::Percentage(10),
    //         Constraint::Percentage(80),
    //         Constraint::Percentage(10),
    //     ])
    //     .split(f.size());

    let block = Block::default().title("Popup").borders(Borders::ALL).blue();
    let paragraph = Paragraph::new(format!(
        "Selected station: {} ({})",
        app.selected_station.as_ref().unwrap().name,
        app.selected_station.as_ref().unwrap().tariff_zones
    ))
    .block(block);

    let area = static_widgets::centered_rect(60, 20, f.size());
    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(paragraph, area);
}

async fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('q') => app.quit(),
                    Char('p') => app.show_popup = !app.show_popup,
                    KeyCode::Down => app.increment_station(),
                    KeyCode::Up => app.decrement_station(),
                    KeyCode::Enter => app.select_station().await,
                    _ => {}
                }
            }
        }
    }
    return Ok(());
}

// async fn update_stations(app: &mut App) {
//     if let Ok(station_ids) = api::fetch_station_ids().await {
//         // println!("Fetched station ids {}", station_ids.len());

//         let mut counter = 0;
//         let station_count = station_ids.len();

//         for station_id in station_ids {
//             match api::fetch_station_info(&station_id).await {
//                 Ok(station_info) => {
//                     counter += 1;
//                     // println!("{:#?}", station_info);
//                     if station_info.len() > 0 && station_info[0].name.len() > 0 {
//                         // app.stations.push(station_info[0].clone())
//                         // continue;
//                     } else {
//                         // println!("No station info found for {}", station_id);
//                     }
//                 }
//                 Err(e) => {
//                     counter += 1;
//                     println!("Error fetching station info for {}", e);
//                 }
//             }

//             // let p = (counter / station_count) * 100;
//             app.progress = counter as u16;
//             // println!("Progress: {} {}", p, app.progress);

//             if counter == 10 {
//                 app.fetching = false;
//                 break;
//             }
//         }
//     }
// }

pub fn initialize_panic_handler() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen).unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}
