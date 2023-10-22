#[allow(unused, dead_code)]
use anyhow::Result; //to avoid writing the error type

use api::get_departures;
use crossterm::{
    event::{self, Event::Key, KeyCode::Char, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{
        Alignment, Constraint, CrosstermBackend, Direction, Layout, Rect, Stylize, Terminal,
    },
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Gauge, List, ListItem, Paragraph},
};
// use tokio::{runtime::Handle, task};

use std::io::stderr;

mod components; //to import the components module
use components::static_widgets; // to avoid typing components::static_widgets:: every time
mod api;

pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>; // alias for the frame type

struct App {
    counter: i64,
    should_quit: bool,
    stations: Result<Vec<api::Station>>,
    show_popup: bool,
    progress: u16,
    fetching: bool,
}

impl App {
    async fn new() -> Self {
        Self {
            counter: 0,
            should_quit: false,
            stations: api::get_stations().await,
            show_popup: false,
            progress: 0,
            fetching: true,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // startup()?;

    // let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    // terminal.clear()?;

    let mut app = App::new().await;

    if let stations = app.stations? {
        println!("Stations: {:#?}", stations[0]);
    }

    // get_departures("de:09162:6").await?;

    // tokio::spawn(async {
    //     update_stations(&mut app).await;
    // });
    // update_stations(&mut app).await;

    // loop {
    //     if app.fetching {
    //         terminal.draw(|f| {
    //             draw_progress_bar(&app, f);
    //         })?;
    //     } else {
    //         // application render
    //         terminal.draw(|f| {
    //             ui(&app, f);
    //         })?;
    //     }
    //     // application update
    //     update(&mut app)?;

    //     // application exit
    //     if app.should_quit {
    //         break;
    //     }
    // }
    // shutdown()?;

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
    let paragraph = Paragraph::new(format!("Counter: {}", app.progress))
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

    // if app.station_names.len() > 0 {
    //     let station_name = Paragraph::new(format!("Name: {}", app.station_names[0].name))
    //         .block(static_widgets::get_app_border())
    //         .style(Style::default().fg(Color::Yellow))
    //         .alignment(Alignment::Center);

    //     // f.render_widget(station_name, chunks[1]);
    // }

    f.render_widget(
        Paragraph::new(format!("This is a line")).light_red(),
        chunks[2],
    );

    if app.show_popup {
        let block = Block::default().title("Popup").borders(Borders::ALL).blue();
        let area = static_widgets::centered_rect(60, 20, f.size());
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(block, area);
    }

    if app.fetching {
        draw_progress_bar(app, f)
    }
}

fn draw_progress_bar(app: &App, f: &mut Frame<'_>) {
    let area = f.size();
    println!("Progress: {}", app.progress);
    let gauge = Gauge::default()
        // .block(Block::default().title("Progress").borders(Borders::ALL))
        .gauge_style(Style::new().light_red())
        .percent(app.progress);
    f.render_widget(gauge, area);
}

fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('j') => app.counter += 1,
                    Char('k') => app.counter -= 1,
                    Char('q') => app.should_quit = true,
                    Char('p') => app.show_popup = !app.show_popup,
                    _ => {}
                }
            }
        }
    }
    return Ok(());
}

async fn update_stations(app: &mut App) {
    if let Ok(station_ids) = api::fetch_station_ids().await {
        // println!("Fetched station ids {}", station_ids.len());

        let mut counter = 0;
        let station_count = station_ids.len();

        for station_id in station_ids {
            match api::fetch_station_info(&station_id).await {
                Ok(station_info) => {
                    counter += 1;
                    // println!("{:#?}", station_info);
                    if station_info.len() > 0 && station_info[0].name.len() > 0 {
                        // app.stations.push(station_info[0].clone())
                        // continue;
                    } else {
                        // println!("No station info found for {}", station_id);
                    }
                }
                Err(e) => {
                    counter += 1;
                    println!("Error fetching station info for {}", e);
                }
            }

            // let p = (counter / station_count) * 100;
            app.progress = counter as u16;
            // println!("Progress: {} {}", p, app.progress);

            if counter == 10 {
                app.fetching = false;
                break;
            }
        }
    }
}
