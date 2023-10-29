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
    prelude::{Alignment, Constraint, CrosstermBackend, Direction, Layout, Stylize, Terminal},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, ListState, Paragraph, Tabs},
};
// use tokio::{runtime::Handle, task};

use std::io::stderr;

mod components; //to import the components module
use components::{static_widgets, station_list::display_departures}; // to avoid typing components::static_widgets:: every time
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

enum AppTabs {
    HomeTab,
    StationTab,
}
pub struct App {
    selected_tab: AppTabs,
    counter: i64,
    should_quit: bool,
    stations: Vec<api::Station>,
    // station_list: StatefulList,
    show_popup: bool,

    selected_station: Option<api::Station>,
    departures: Vec<api::DepartureInfo>,
    should_redraw: bool,
}

pub struct Deprtures {
    current_station_id: String,
    departures: Vec<api::DepartureInfo>,
    is_loading: bool,
}

impl Deprtures {
    fn new() -> Self {
        Self {
            current_station_id: String::from(""),
            departures: vec![],
            is_loading: false,
        }
    }
}

async fn refresh_departures(departures: &mut Deprtures, app: &App) {
    loop {
        departures.is_loading = true;
        let current_station_id: &str = match &app.selected_station {
            Some(station) => station.id.as_str(),
            None => "",
        };

        departures.departures = get_departures(current_station_id)
            .await
            .unwrap_or_else(|_| vec![]);
        //wait a minute
        departures.is_loading = false;
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}

impl App {
    async fn new() -> Self {
        Self {
            selected_tab: AppTabs::HomeTab,
            counter: 0,
            should_quit: false,
            stations: api::get_stations().await.unwrap_or_else(|_| vec![]),
            // station_list: StatefulList::new().await,
            show_popup: false,
            selected_station: None,
            departures: vec![],
            should_redraw: true,
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
        self.update_departures().await;
        self.show_popup = !self.show_popup; //temp
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // initialize_panic_handler();
    startup()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    println!("initializing app...");
    println!("fetching stations...");
    let mut app = App::new().await;
    terminal.clear()?;
    // let mut departures = Deprtures::new();

    // refresh_departures(&mut departures, &app).await;

    // println!("current_station_id: {:#?}", app.stations[0]);

    // if let stations = app.stations? {
    //     println!("Stations: {:#?}", stations[0]);
    // }

    loop {
        // application render

        if app.should_redraw {
            terminal.draw(|f| {
                ui(&app, f);
            })?;
            app.should_redraw = false;
        }

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
    // let paragraph = Paragraph::new(format!("Counter: {}", app.counter))
    //     .block(static_widgets::get_app_border())
    //     .style(Style::default().fg(Color::Yellow))
    //     .alignment(Alignment::Center);

    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(size);

    let block = Block::default();
    f.render_widget(block, size);

    let tab_names = vec!["Home", "Station"];
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
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(index)
        .style(Style::default())
        .highlight_style(Style::default().bold());

    f.render_widget(tabs, chunks[0]);
    match app.selected_tab {
        AppTabs::HomeTab => draw_popup(f, app),
        AppTabs::StationTab => f.render_widget(itemlist, chunks[1]),
    };

    // f.render_widget(itemlist, chunks[1]);

    // f.render_widget(
    //     Paragraph::new(format!("Press p to toggle departures, enter to select station, q to quit app"))
    //         .light_red()
    //         .block(Block::default().borders(Borders::TOP))
    //         .alignment(Alignment::Center),
    //     chunks[2],
    // );

    // if app.show_popup {
    //     draw_popup(f, app)
    // }
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

    let popup_title = match &app.selected_station {
        Some(station) => format!("{}", station.name),
        None => "Unknown Station".to_string(),
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
                    Char('p') => {
                        app.show_popup = !app.show_popup;
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
                    _ => {}
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
