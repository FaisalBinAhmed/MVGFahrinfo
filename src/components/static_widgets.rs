use ratatui::{
    prelude::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders},
};

pub fn get_app_border() -> Block<'static> {
    return Block::default()
        .borders(Borders::ALL)
        .title("MVG Departures")
        .border_type(BorderType::Rounded)
        .title_alignment(Alignment::Center);
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
