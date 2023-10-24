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
    stations: Vec<api::Station>,
    show_popup: bool,
    progress: u16,
    fetching: bool,
}

impl App {
    async fn new() -> Self {
        Self {
            counter: 0,
            should_quit: false,
            stations: api::get_stations().await.unwrap_or_else(|_| vec![]),
            show_popup: false,
            progress: 0,
            fetching: true,
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
}

#[tokio::main]
async fn main() -> Result<()> {
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

    let itemlist = List::new(
        app.stations
            // .as_ref()
            // .unwrap() //TODO: handle result later
            .iter()
            .enumerate()
            .map(|(index, station)| {
                ListItem::new(vec![
                    Line::from(vec![
                        Span::styled(
                            format!("{}", station.name),
                            Style::default().fg(if index == app.counter as usize {
                                Color::Blue
                            } else {
                                Color::White
                            }),
                        ),
                        Span::styled(
                            format!(" ({})", station.tariff_zones),
                            Style::default().fg(Color::LightRed),
                        ),
                    ]),
                    // Line::from(vec![
                    // Span::styled(
                    //     format!("ID: {}", station.id),
                    //     Style::default().fg(Color::Blue),
                    // ),
                    // get_product_icon_spans(&station.products),
                    // Span::styled(
                    //     format!(
                    //         " ({})",
                    //         station.abbreviation.as_ref().unwrap_or(&"".to_string())
                    //     ),
                    //     Style::default().fg(Color::DarkGray),
                    // ),
                    // ]),
                    Line::from(get_product_icon_spans(&station.products)),
                ])
            })
            .collect::<Vec<ListItem>>(),
    );

    f.render_widget(itemlist, chunks[1]);

    f.render_widget(
        Paragraph::new(format!("Press p to select station, q to quit app"))
            .light_red()
            .block(Block::default().borders(Borders::TOP))
            .alignment(Alignment::Center),
        chunks[2],
    );

    if app.show_popup {
        draw_popup(f)
    }
}

fn draw_popup(f: &mut Frame<'_>) {
    let block = Block::default().title("Popup").borders(Borders::ALL).blue();
    let area = static_widgets::centered_rect(60, 20, f.size());
    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(block, area);
}

fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('q') => app.quit(),
                    Char('p') => app.show_popup = !app.show_popup,
                    KeyCode::Down => app.increment_station(),
                    KeyCode::Up => app.decrement_station(),
                    _ => {}
                }
            }
        }
    }
    return Ok(());
}

fn get_product_icon_spans(products: &Vec<String>) -> Vec<Span> {
    let mut spans = vec![];
    for product in products {
        let icon = match product.as_str() {
            "UBAHN" => Span::styled(
                " U ",
                Style::default()
                    .bg(Color::Rgb(29, 43, 83))
                    .fg(Color::White)
                    .bold(),
            ),
            "BUS" => Span::styled(
                " BUS ",
                Style::default()
                    .bg(Color::Rgb(17, 93, 111))
                    .fg(Color::White),
            ),
            "TRAM" => Span::styled(
                " Tram ",
                Style::default()
                    .bg(Color::Rgb(231, 27, 30))
                    .fg(Color::White),
            ),
            "SBAHN" => Span::styled(
                " S ",
                Style::default()
                    .bg(Color::Rgb(84, 253, 84))
                    .fg(Color::Black),
            )
            .bold(),
            _ => Span::styled(
                product,
                Style::default().bg(Color::LightYellow).fg(Color::Black),
            ),
        };
        spans.push(icon);
        spans.push(Span::raw(" ")); // add a space between the icons
    }
    return spans;
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
