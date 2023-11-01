use ratatui::{
    prelude::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Padding, Paragraph, Tabs},
};

use crate::{
    app::{App, AppTabs},
    components::{self, static_widgets, station_list::display_departures_table},
    tui::Frame,
};

pub fn render(app: &mut App, f: &mut Frame) {
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
        .highlight_style(Style::default().fg(Color::Green));

    f.render_widget(tabs, chunks[0]);

    let list_state = &mut app.scroll_state.clone(); //we can clone this value, because it is cheap and the function is called only once per frame

    match app.selected_tab {
        AppTabs::HomeTab => draw_departures(f, app),
        AppTabs::StationTab => f.render_stateful_widget(itemlist, chunks[1], list_state),
    };

    let bottom_line_text = Line::from(vec![
        Span::styled(
            format!("Q: close app, tab: switch tabs, up/down: scroll stations, enter: select station, r: reload departures"),
            Style::default().fg(Color::LightYellow),
        ),
        Span::styled(
            format!(" ({})", &app.status),
            Style::default().fg(Color::LightCyan),
        ),
    ]);

    f.render_widget(
        Paragraph::new(bottom_line_text),
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
        .padding(Padding::new(2, 2, 1, 1))
        .style(Style::default().fg(Color::LightCyan));

    // let list = display_departures(&app.departures).block(block);
    let table = display_departures_table(&app.departures).block(block);

    let area = static_widgets::centered_rect(69, 50, f.size());
    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(table, area);
}
