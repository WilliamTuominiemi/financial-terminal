use dotenv::dotenv;
mod data;
use color_eyre::Result;
use crossterm::event;
use data::send;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::symbols::Marker;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Axis, Chart, Dataset, GraphType};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

fn main() -> Result<()> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not set");

    let symbol = "AAPL";
    let url = format!(
        "https://www.alphavantage.co/query?function=TIME_SERIES_WEEKLY&symbol={symbol}&apikey={api_key}"
    );

    let mut dataset = match send(&url) {
        Ok(mut summaries) => {
            summaries.sort_by(|a, b| a.date.cmp(&b.date));
            summaries
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            Vec::new()
        }
    };

    color_eyre::install()?;
    ratatui::run(|terminal| {
        loop {
            terminal.draw(render)?;
            if let event::Event::Key(_) = event::read()? {
                break Ok(());
            }
        }
    })
}

fn render(frame: &mut Frame) {
    let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let [top, main] = frame.area().layout(&layout);

    let title = Line::from_iter([
        Span::from("Chart Widget").bold(),
        Span::from(" (Press 'q' to quit)"),
    ]);
    frame.render_widget(title.centered(), top);

    render_chart(frame, main);
}

pub fn render_chart(frame: &mut Frame, area: Rect) {
    let dataset = Dataset::default()
        .name("Stonks")
        .marker(Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Color::Blue)
        .data(&[
            (0.0, 1.0),
            (1.0, 3.0),
            (2.0, 0.5),
            (3.0, 2.0),
            (4.0, 0.8),
            (5.0, 4.0),
            (6.0, 1.0),
            (7.0, 6.0),
            (8.0, 3.0),
            (10.0, 10.0),
        ]);

    let x_axis = Axis::default()
        .title("Hustle".blue())
        .bounds([0.0, 10.0])
        .labels(["0%", "50%", "100%"]);

    let y_axis = Axis::default()
        .title("Profit".blue())
        .bounds([0.0, 10.0])
        .labels(["0", "5", "10"]);

    let chart = Chart::new(vec![dataset]).x_axis(x_axis).y_axis(y_axis);
    frame.render_widget(chart, area);
}
