#![allow(unused)]
use anyhow::Result; //to avoid writing the error type
use serde::Deserialize;

use crossterm::{
    event::{self, Event::Key, KeyCode::Char, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{Alignment, CrosstermBackend, Stylize, Terminal},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use std::io::stderr;

mod components;
use components::static_widgets;

pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>;

struct App {
    counter: i64,
    should_quit: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    startup()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    terminal.clear()?;

    let mut app = App {
        counter: 0,
        should_quit: false,
    };

    loop {
        // application render
        terminal.draw(|f| {
            ui(&app, f);
        })?;

        // application update
        update(&mut app)?;

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
    return f.render_widget(
        Paragraph::new(format!("Counter: {}", app.counter))
            .block(static_widgets::get_app_border())
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center),
        f.size(),
    );
}

fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('j') => app.counter += 1,
                    Char('k') => app.counter -= 1,
                    Char('q') => app.should_quit = true,
                    _ => {}
                }
            }
        }
    }
    return Ok(());
}

// let mvg_fib = "https://www.mvg.de/api/fib/v2";
// let mvg_zdm = "https://www.mvg.de/.rest/zdm/";

// const LOCATION_URL: &'static str = "/location";

// struct EndPoint {
//     url: &'static str,
//     args: Vec<&'static str>,
// }

// let location_endpoint = EndPoint {
//     url: LOCATION_URL,
//     args: vec!["query"],
// };

// let station_ids_endpoint = EndPoint {
//     url: "mvgStationGlobalIds",
//     args: vec![],
// };

// let resp = fetch_url(mvg_zdm, station_ids_endpoint.url).await?;

// let location_url = format!("{}{}", mvg_fib, LOCATION_URL);

// let resp = fetch_station_info(&location_url, "de:09162:6").await?;
// println!("{:#?}", resp);

async fn fetch_url(base_url: &str, url: &str) -> Result<()> {
    let full_url = format!("{}{}", base_url, url);

    let resp = reqwest::get(full_url).await?.json::<Vec<String>>().await?;
    println!("{:#?}", resp);
    return Ok(());
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")] //to avoid renaming all the fields to snake_case
struct StationInfo {
    house_number: String,
    latitude: f32,
    longitude: f32,
    name: String,
    place: String,
    post_code: String,
    street: String,
    r#type: String, //type is a reserved keyword in Rust
}

async fn fetch_station_info(url: &str, query: &str) -> Result<()> {
    let full_url = format!("{}?query={}", url, query);

    let resp = reqwest::get(full_url)
        .await?
        .json::<Vec<StationInfo>>()
        .await?;
    println!("{:#?}", resp);
    return Ok(());
}
