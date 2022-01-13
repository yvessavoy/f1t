use crate::util::{StatefulList, TabsState};
use crate::{App, SelectedScreen};
use f1::Standing;
use std::fmt::Display;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Color;
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders};
use tui::widgets::{List, ListItem};
use tui::widgets::{Paragraph, Tabs};
use tui::Frame;

pub fn ui<B>(f: &mut Frame<B>, area: Rect, app: &mut App)
where
    B: Backend,
{
    let block = Block::default()
        .title("Circuit Information")
        .borders(Borders::ALL);

    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(area);

    if let Some(weekend) = app.weekends.get(&app.current_season) {
        let circuit = weekend
            .items
            .get(weekend.state.selected().unwrap_or_default())
            .unwrap()
            .clone()
            .circuit;

        let p = Paragraph::new(format!("Name: {}", circuit.name));
        f.render_widget(p, chunks[0]);

        let p = Paragraph::new(format!("Country: {}", circuit.country));
        f.render_widget(p, chunks[1]);

        let p = Paragraph::new(format!("Locality: {}", circuit.locality));
        f.render_widget(p, chunks[2]);

        let p = Paragraph::new(format!("Latitude: {}", circuit.latitude));
        f.render_widget(p, chunks[3]);

        let p = Paragraph::new(format!("Longitude: {}", circuit.longitude));
        f.render_widget(p, chunks[4]);
    }
}
