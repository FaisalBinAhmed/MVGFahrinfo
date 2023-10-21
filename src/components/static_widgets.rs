use ratatui::{
    prelude::Alignment,
    widgets::{Block, BorderType, Borders},
};

pub fn get_app_border() -> Block<'static> {
    return Block::default()
        .borders(Borders::ALL)
        .title("App")
        .border_type(BorderType::Rounded)
        .title_alignment(Alignment::Center);
}
