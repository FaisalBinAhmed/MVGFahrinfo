use chrono::Utc;
use ratatui::{
    prelude::Constraint,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Cell, List, ListItem, Row, Table},
};

use crate::{
    api::{self, Station},
    constants::{get_sbahn_color, get_ubahn_color},
    App,
};
// this is used in the Station List tab
pub fn get_station_list_widget(app: &App) -> List {
    return List::new(
        app.stations
            .iter()
            .map(|station| {
                ListItem::new(vec![
                    Line::from(vec![
                        Span::styled(format!("{}", station.name), Style::default()),
                        Span::styled(
                            format!(" ({})", station.tariff_zones),
                            Style::default().fg(Color::LightCyan),
                        ),
                    ]),
                    Line::from(get_product_icon_spans(&station.products)),
                ])
            })
            .collect::<Vec<ListItem>>(),
    )
    .highlight_style(
        Style::default()
            .bg(Color::Rgb(38, 35, 38))
            .add_modifier(Modifier::BOLD),
    );
    // .highlight_symbol(">> ");
}

fn get_type_icon(product: &str) -> Span {
    let icon = match product {
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
    icon
}

fn get_product_icon_spans(products: &Vec<String>) -> Vec<Span> {
    let mut spans = vec![];
    for product in products {
        let icon = get_type_icon(product);
        spans.push(icon);
        spans.push(Span::raw(" ")); // add a space between the icons
    }
    spans
}

pub fn display_departures_table(departures: &[api::DepartureInfo]) -> Table {
    let header_cells = ["Vehicle", "Direction", "Platform", "ETA"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Gray)));

    let header = Row::new(header_cells)
        .style(
            Style::default(), // .bg(Color::White)
        )
        .height(2)
        .bottom_margin(1);

    let rows = departures.iter().enumerate().map(|(index, item)| {
        let cells = vec![
            Cell::from(get_vehicle_label(&item.label, &item.transport_type)),
            Cell::from(format!("{}", item.destination)),
            Cell::from(get_platform_number(item.platform, index)),
            Cell::from(match get_minutes(item.realtime_departure_time) {
                ETA::Minutes(minutes) => format!("{} min", minutes),
                ETA::Now => "now".to_string(),
            }),
        ];
        return Row::new(cells).height(1);
    });

    let t = Table::new(rows)
        .header(header)
        .style(Style::default().fg(Color::White))
        .widths(&[
            Constraint::Percentage(20),
            Constraint::Max(50),
            Constraint::Percentage(20),
            Constraint::Min(10),
        ]);
    t
}

fn get_platform_number<'a>(platform: Option<i64>, index: usize) -> Span<'a> {
    let bg = if index % 2 == 0 {
        Color::White
    } else {
        Color::Gray
    };
    return match platform {
        Some(p) => Span::styled(format!(" {} ", p), Style::default().bg(bg).fg(Color::Black)),
        None => Span::styled(" ", Style::default().fg(Color::White)),
    };
}

fn get_vehicle_label<'a>(label: &'a str, transport_type: &str) -> Line<'a> {
    let icon = match transport_type {
        "UBAHN" => vec![
            Span::styled(
                format!(" U "),
                Style::default().bg(Color::Rgb(29, 43, 83)).fg(Color::White),
            ),
            Span::raw(" "),
            Span::styled(
                format!(" {} ", label),
                Style::default().bg(get_ubahn_color(label)).fg(Color::White),
            ),
        ],
        "BUS" => vec![
            Span::styled(
                format!(" B "),
                Style::default()
                    .bg(Color::Rgb(17, 93, 111))
                    .fg(Color::White),
            ),
            Span::raw(" "),
            Span::styled(
                format!(" {} ", label),
                Style::default()
                    .bg(Color::Rgb(17, 93, 111))
                    .fg(Color::White),
            ),
        ],
        "TRAM" => vec![
            Span::styled(
                format!(" T "),
                Style::default()
                    .bg(Color::Rgb(231, 27, 30))
                    .fg(Color::White),
            ),
            Span::raw(" "),
            Span::styled(
                format!(" {} ", label),
                Style::default()
                    .bg(Color::Rgb(231, 27, 30))
                    .fg(Color::White),
            ),
        ],
        "SBAHN" => vec![
            Span::styled(
                format!(" S "),
                Style::default()
                    .bg(Color::Rgb(84, 253, 84))
                    .fg(Color::Black),
            ),
            Span::raw(" "),
            Span::styled(
                format!(" {} ", label),
                Style::default().bg(get_sbahn_color(label)).fg(Color::White),
            ),
        ],
        // .bold(),
        _ => vec![Span::styled(
            label,
            Style::default().bg(Color::LightYellow).fg(Color::Black),
        )],
    };
    Line::from(icon)
}

// sometimes, the departure time is negative
// in that case, we return a string instead of a number. This is a temporary fix though
enum ETA {
    Minutes(i64),
    Now, // it might also be depaurtures that are late and come in any moment. Needs more investigation
}

fn get_minutes(time: i64) -> ETA {
    let now = Utc::now();
    let timestamp_in_seconds = time / 1000;
    let future_time = chrono::DateTime::from_timestamp(timestamp_in_seconds, 0).unwrap();
    let diff = future_time.signed_duration_since(now); //now.signed_duration_since(future_time);

    let minutes = diff.num_minutes();

    if minutes < 1 {
        return ETA::Now;
    } else {
        return ETA::Minutes(minutes);
    }
}

// search suggestions

pub fn get_suggested_station_list(app: &mut App) -> List {
    let mut suggested_stations: Vec<Station> = vec![];

    let suggested_stations_list = app
        .stations
        .iter()
        .filter(|station| {
            // todo: move to filter_map
            station
                .name
                .to_ascii_lowercase()
                .contains(&app.query.to_ascii_lowercase())
        })
        .map(|station| {
            suggested_stations.push(station.clone());
            return ListItem::new(vec![Line::from(vec![
                Span::styled(format!("{}", station.name), Style::default()),
                Span::styled(
                    format!(" ({})", station.tariff_zones),
                    Style::default().fg(Color::LightCyan),
                ),
            ])]);
        })
        .collect::<Vec<ListItem>>();

    app.suggested_stations = suggested_stations;

    return List::new(suggested_stations_list);
}
