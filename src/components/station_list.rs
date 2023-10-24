use chrono::Utc;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem},
};

use crate::{api, App};

pub fn get_station_list_widget(app: &App) -> List {
    return List::new(
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
                            Style::default().fg(Color::LightCyan),
                        ),
                        Span::styled(
                            if index == app.counter as usize {
                                format!(" <<",)
                            } else {
                                format!("  ")
                            },
                            Style::default().fg(Color::LightYellow),
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
}

fn get_product_icon_spans(products: &Vec<String>) -> Vec<Span> {
    let mut spans = vec![];
    for product in products {
        let icon = match product.as_str() {
            "UBAHN" => Span::styled(
                " U ",
                Style::default().bg(Color::Rgb(29, 43, 83)).fg(Color::White), // .bold(),
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
            ),
            // .bold(),
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

pub fn display_departures(departures: &Vec<api::DepartureInfo>) -> List {
    return List::new(
        departures
            .iter()
            .enumerate()
            .map(|(index, departure)| {
                ListItem::new(vec![
                    Line::from(vec![
                        Span::styled(
                            format!("{}", departure.label),
                            // Style::default().fg(if index == app.counter as usize {
                            //     Color::Blue
                            // } else {
                            //     Color::White
                            // }),
                            Style::default().fg(Color::Blue),
                        ),
                        Span::styled(
                            format!(" ({})", departure.destination),
                            Style::default().fg(Color::LightCyan),
                        ),
                        Span::styled(
                            format!(
                                " ({})",
                                get_minutes(departure.realtime_departure_time.clone())
                            ),
                            Style::default().fg(Color::LightYellow),
                        ),
                    ]),
                    // Line::from(get_product_icon_spans(&station.products)),
                ])
            })
            .collect::<Vec<ListItem>>(),
    );
}

fn get_minutes(time: i64) -> i64 {
    let now = Utc::now();
    let timestamp_in_seconds = time / 1000;
    let future_time = chrono::DateTime::from_timestamp(timestamp_in_seconds, 0).unwrap();
    let diff = now.signed_duration_since(future_time);

    return diff.num_minutes();
}
