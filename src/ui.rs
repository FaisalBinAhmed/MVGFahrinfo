use ratatui::{
    prelude::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Padding, Paragraph, Tabs},
};

use crate::{
    app::{App, AppTabs},
    components::{
        self, static_widgets,
        station_list::{display_departures_table, get_suggested_station_list},
    },
    tui::Frame,
};

pub fn render(app: &mut App, f: &mut Frame) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(size);

    let block = Block::default();
    f.render_widget(block, size);

    let titles = ["Departures", "Station List"]
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
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" MVG FahrInfo "),
        )
        .select(index)
        .style(Style::default())
        .highlight_style(Style::default().fg(Color::Green));

    f.render_widget(tabs, chunks[0]);

    let list_state = &mut app.scroll_state.clone(); //we can clone this value, because it is cheap and the function is called only once per frame

    match app.selected_tab {
        AppTabs::HomeTab => draw_departures(f, app),
        AppTabs::StationTab => f.render_stateful_widget(itemlist, chunks[1], list_state),
    };

    //Status bar

    let app_mode_indicator: Vec<Span> = match app.app_mode {
        crate::app::AppMode::Normal => {
            vec![
            Span::styled(format!(" NORMAL "), Style::default().bg(Color::Blue).bold()),
            Span::styled(
            format!(" Q: close app. Tab: switch tabs. Enter: select station. R: reload departures. S: search. "),
            Style::default()),
            Span::styled(
            format!("Last refreshed: {}", &app.last_refreshed),
            Style::default().fg(Color::LightCyan))]
        }
        crate::app::AppMode::Search => {
            vec![
                Span::styled(format!(" SEARCH "), Style::default().bg(Color::Red).bold()),
                Span::styled(
                    format!(
                        " Esc: back to normal mode. Up/Down: navigate. Enter: select station. "
                    ),
                    Style::default(),
                ),
            ]
        }
    };

    let status_bar = Line::from(app_mode_indicator);

    f.render_widget(Paragraph::new(status_bar), chunks[2]);

    //SEARCH MODAL
    //todo: move to its own component

    if app.app_mode == crate::app::AppMode::Search {
        let popup_title = " ⌕ Search for a station ";

        let mut text = Text::from(Line::from(app.query.clone()));
        text.patch_style(Style::default().add_modifier(Modifier::RAPID_BLINK));

        let input_field = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title(popup_title))
            .style(Style::default().fg(Color::LightCyan))
            .alignment(ratatui::prelude::Alignment::Left);

        let area = static_widgets::centered_rect(69, 50, f.size()); //size of the MODAL

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(input_field, chunks[0]);
        f.set_cursor(
            // Draw the cursor at the current position in the input field.
            // This position is can be controlled via the left and right arrow key
            chunks[0].x + app.cursor_position as u16 + 1,
            // Move one line down, from the border to the input line
            chunks[0].y + 1,
        );

        //search suggestion section

        let search_scroll_state = &mut app.search_scroll_state.clone();
        let suggested_stations = get_suggested_station_list(app).highlight_style(
            Style::default()
                .bg(Color::Rgb(38, 35, 53))
                .add_modifier(Modifier::BOLD),
        );

        f.render_stateful_widget(suggested_stations, chunks[1], search_scroll_state);
    }
}

fn draw_departures(f: &mut Frame<'_>, app: &App) {
    let popup_title = match &app.selected_station {
        Some(station) => format!(" {} ", station.name),
        None => " No station selected ".to_string(),
    };

    let block = Block::default()
        .title(popup_title)
        .borders(Borders::ALL)
        .padding(Padding::new(2, 2, 1, 1))
        .style(Style::default());

    let table = display_departures_table(&app.departures).block(block);

    let area = static_widgets::centered_rect(80, 69, f.size());
    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(table, area);
}
