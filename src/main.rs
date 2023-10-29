#[allow(unused, dead_code)]
use anyhow::Result; //to avoid writing the error type

use api::get_departures;
use crossterm::{
    event::{
        self,
        Event::Key,
        KeyCode::{self, Char},
    },
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{Constraint, CrosstermBackend, Direction, Layout, Stylize, Terminal},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Tabs},
};
// use tokio::{runtime::Handle, task};

use std::io::stderr;

mod components; //to import the components module
use components::{static_widgets, station_list::display_departures}; // to avoid typing components::static_widgets:: every time
mod api;

pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>; // alias for the frame type

#[derive(PartialEq)] // need this to do binary comparison
enum AppTabs {
    HomeTab,
    StationTab,
}
pub struct App {
    selected_tab: AppTabs,
    counter: i64,
    should_quit: bool,
    stations: Vec<api::Station>,
    selected_station: Option<api::Station>,
    departures: Vec<api::DepartureInfo>,
    should_redraw: bool,
    // auto_refresh: bool,
    status: String,
}

impl App {
    async fn new() -> Self {
        Self {
            selected_tab: AppTabs::HomeTab,
            counter: 0,
            should_quit: false,
            stations: api::get_stations().await.unwrap_or_else(|_| vec![]),
            selected_station: None,
            departures: vec![],
            should_redraw: true,
            // auto_refresh: false,
            status: "Loading stations...".to_string(),
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
    fn toggle_tabs(&mut self) {
        match self.selected_tab {
            AppTabs::HomeTab => self.selected_tab = AppTabs::StationTab,
            AppTabs::StationTab => self.selected_tab = AppTabs::HomeTab,
        }
    }

    async fn update_departures(&mut self) {
        if let Some(station) = &self.selected_station {
            self.departures = get_departures(&station.id).await.unwrap_or_else(|_| vec![]);
        }
    }

    async fn select_station(&mut self) {
        self.selected_station = Some(self.stations[self.counter as usize].clone());
        self.status = format!("Fetching departures");
        self.update_departures().await;
        self.selected_tab = AppTabs::HomeTab; // switch to home tab immidiatelyq
        self.should_redraw = true;
        // self.auto_refresh = true;
        // self.keep_refreshing_departures().await;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // initialize_panic_handler();
    println!("initializing app...");
    startup()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    terminal.clear()?;
    println!("fetching stations...");
    let mut app = App::new().await;
    terminal.clear()?;

    loop {
        if app.should_redraw {
            // only redraw if explicitly told to
            terminal.draw(|f| {
                ui(&app, f);
            })?;
            app.should_redraw = false;
        }

        // if app.auto_refresh {
        //     app.keep_refreshing_departures().await
        // }
        // application update
        update(&mut app).await?;

        // if app.selected_tab == AppTabs::HomeTab {
        //     app.update_departures().await;
        //     app.should_redraw = true;
        // }

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
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(size);

    let block = Block::default();
    f.render_widget(block, size);

    let tab_names = vec!["Departures", "Station List"];
    let titles = tab_names
        .iter()
        .map(|t| {
            Line::from(Span::styled(
                format!("{}", t),
                Style::default().fg(Color::LightCyan),
            ))
        })
        .collect();

    let index: usize = match app.selected_tab {
        AppTabs::HomeTab => 0,
        AppTabs::StationTab => 1,
    };

    let itemlist = components::station_list::get_station_list_widget(app);

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("MVG FahrInfo"))
        .select(index)
        .style(Style::default())
        .highlight_style(Style::default().bold());

    f.render_widget(tabs, chunks[0]);
    match app.selected_tab {
        AppTabs::HomeTab => draw_departures(f, app),
        AppTabs::StationTab => f.render_widget(itemlist, chunks[1]),
    };

    let bottom_line_text = Line::from(vec![
        Span::styled(
            format!("Press q to close app, tab to switch tabs and enter to select station"),
            Style::default(),
        ),
        Span::styled(
            format!(" ({})", &app.status),
            Style::default().fg(Color::LightCyan),
        ),
    ]);

    f.render_widget(
        Paragraph::new(bottom_line_text).light_red(),
        // .block(Block::default().borders(Borders::TOP))
        // .alignment(Alignment::Center),
        chunks[2],
    );
}

fn draw_departures(f: &mut Frame<'_>, app: &App) {
    let popup_title = match &app.selected_station {
        Some(station) => format!("{}", station.name),
        None => "No station selected".to_string(),
    };

    let block = Block::default()
        .title(popup_title)
        .borders(Borders::ALL)
        .blue();

    let list = display_departures(&app.departures).block(block);

    let area = static_widgets::centered_rect(60, 40, f.size());
    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(list, area);
}

async fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('q') => app.quit(),
                    Char('r') => {
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
                    _ => {
                        // todo: pass the key event
                    }
                }
            }
        }
    }
    return Ok(());
}

pub fn initialize_panic_handler() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen).unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}
