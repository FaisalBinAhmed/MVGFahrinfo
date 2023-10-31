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
            .map(|(_index, departure)| {
                ListItem::new(vec![
                    Line::from(vec![
                        get_vehicle_label(&departure.label, &departure.transport_type),
                        Span::styled(
                            format!(" {}", departure.destination),
                            Style::default().fg(Color::LightCyan),
                        ),
                        Span::styled(
                            format!(
                                " ({}) min",
                                get_minutes(departure.realtime_departure_time.clone())
                            ),
                            Style::default().fg(Color::White),
                        ),
                    ]),
                    // Line::from(get_product_icon_spans(&station.products)),
                ])
            })
            .collect::<Vec<ListItem>>(),
    );
}

fn get_vehicle_label<'a>(label: &'a str, transport_type: &'a str) -> Span<'a> {
    let icon = match transport_type {
        "UBAHN" => Span::styled(
            format!(" {} ", label),
            Style::default().bg(Color::Rgb(29, 43, 83)).fg(Color::White), // .bold(),
        ),
        "BUS" => Span::styled(
            format!(" {} ", label),
            Style::default()
                .bg(Color::Rgb(17, 93, 111))
                .fg(Color::White),
        ),
        "TRAM" => Span::styled(
            format!(" {} ", label),
            Style::default()
                .bg(Color::Rgb(231, 27, 30))
                .fg(Color::White),
        ),
        "SBAHN" => Span::styled(
            format!(" {} ", label),
            Style::default()
                .bg(Color::Rgb(84, 253, 84))
                .fg(Color::Black),
        ),
        // .bold(),
        _ => Span::styled(
            label,
            Style::default().bg(Color::LightYellow).fg(Color::Black),
        ),
    };
    return icon;
}

fn get_minutes(time: i64) -> i64 {
    let now = Utc::now();
    let timestamp_in_seconds = time / 1000;
    let future_time = chrono::DateTime::from_timestamp(timestamp_in_seconds, 0).unwrap();
    let diff = future_time.signed_duration_since(now); //now.signed_duration_since(future_time);

    return diff.num_minutes();
}
