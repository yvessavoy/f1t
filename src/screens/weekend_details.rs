use crate::util::{StatefulList, TabsState};
use crate::{App, SelectedScreen};
use f1::Standing;
use std::fmt::Display;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Color;
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::Tabs;
use tui::widgets::{Block, BorderType, Borders};
use tui::widgets::{List, ListItem};
use tui::Frame;

pub fn ui<B>(f: &mut Frame<B>, area: Rect, app: &mut App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(area);

    let hm = app.weekends.get(&app.current_season).unwrap();
    let weekend = hm.items.get(hm.state.selected().unwrap()).unwrap();
    let titles = weekend
        .sessions
        .iter()
        .map(|s| Spans::from(vec![Span::raw(s.r#type.to_string())]))
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(weekend.name.clone()),
        )
        .select(app.detail_tabs.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );

    f.render_widget(tabs, chunks[0]);

    let ranking: Vec<ListItem> = app
        .ranking
        .items
        .iter()
        .map(|s| {
            ListItem::new(format!(
                "{}. {} Time: {}",
                s.position, s.driver.screen_name, s.lap_time
            ))
        })
        .collect();

    let list = List::new(ranking)
        .block(Block::default().title("Ranking").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunks[1], &mut app.ranking.state);
}
