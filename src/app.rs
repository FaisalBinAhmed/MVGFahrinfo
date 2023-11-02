use ratatui::widgets::ListState;

use crate::api;

#[derive(PartialEq)] // need this to do binary comparison
pub enum AppTabs {
    HomeTab,
    StationTab,
}

#[derive(PartialEq)]
pub enum AppMode {
    Normal,
    Search,
}

pub struct App {
    pub selected_tab: AppTabs,
    pub should_quit: bool,
    pub stations: Vec<api::Station>,
    pub selected_station: Option<api::Station>,
    pub departures: Vec<api::DepartureInfo>,
    pub should_redraw: bool,
    // auto_refresh: bool,
    pub status: String,
    //scroll related
    // pub scroll_position: usize,
    pub scroll_state: ListState,
    //search related
    pub app_mode: AppMode,
    pub query: String,
    pub cursor_position: usize,
    pub suggested_stations: Vec<api::Station>,
    pub search_scroll_state: ListState,
}

impl App {
    pub async fn new() -> Self {
        Self {
            selected_tab: AppTabs::HomeTab,
            should_quit: false,
            stations: api::get_stations().await.unwrap_or_else(|_| vec![]),
            selected_station: None,
            departures: vec![],
            should_redraw: true,
            // auto_refresh: false,
            status: "Loading stations...".to_string(),
            scroll_state: ListState::default(),
            app_mode: AppMode::Normal,
            query: String::new(),
            cursor_position: 0,
            search_scroll_state: ListState::default(),
            suggested_stations: vec![],
        }
    }
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
    pub fn increment_station(&mut self) {
        let i = match self.scroll_state.selected() {
            Some(i) => {
                if i >= self.stations.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.scroll_state.select(Some(i));
    }

    pub fn decrement_station(&mut self) {
        let i = match self.scroll_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.stations.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.scroll_state.select(Some(i));
    }
    pub fn toggle_tabs(&mut self) {
        match self.selected_tab {
            AppTabs::HomeTab => self.selected_tab = AppTabs::StationTab,
            AppTabs::StationTab => self.selected_tab = AppTabs::HomeTab,
        }
    }

    pub async fn update_departures(&mut self) {
        if let Some(station) = &self.selected_station {
            self.departures = match api::get_departures(&station.id).await {
                Ok(departures) => departures,
                Err(e) => {
                    println!("Error fetching departures {}", e);
                    vec![]
                }
            }
        }
    }

    pub async fn select_station(&mut self) {
        self.selected_station = match self.scroll_state.selected() {
            Some(i) => Some(self.stations[i].clone()),
            None => None,
        };
        self.status = format!("Fetching departures");
        self.update_departures().await;
        self.selected_tab = AppTabs::HomeTab; // switch to home tab immidiatelyq
        self.should_redraw = true;
        // self.auto_refresh = true;
        // self.keep_refreshing_departures().await;
    }
}

//second impl block for the search mode and to keep the code clean
//reference: ratatui book https://github.com/ratatui-org/ratatui/blob/main/examples/user_input.rs

impl App {
    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.query.insert(self.cursor_position, new_char);
        self.move_cursor_right();

        //should also commence the search
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.query.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.query.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.query = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }
    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.query.len())
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    //search result related

    pub fn scroll_down(&mut self) {
        let i = match self.search_scroll_state.selected() {
            Some(i) => {
                if i >= self.suggested_stations.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.search_scroll_state.select(Some(i));
    }

    pub fn scroll_up(&mut self) {
        let i = match self.search_scroll_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.suggested_stations.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.search_scroll_state.select(Some(i));
    }

    pub async fn select_searched_station(&mut self) {
        self.selected_station = match self.search_scroll_state.selected() {
            Some(i) => Some(self.suggested_stations[i].clone()),
            None => None,
        };
        self.status = format!("Fetching departures from search");
        self.suggested_stations.clear();
        self.update_departures().await;
        self.selected_tab = AppTabs::HomeTab;
        self.app_mode = AppMode::Normal;
        self.query.clear();
        self.reset_cursor();
    }
}
