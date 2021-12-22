use crate::{App, SelectedScreen};
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Modifier, Style};
use tui::widgets::Paragraph;
use tui::widgets::{Block, BorderType, Borders, List, ListItem};
use tui::Frame;

pub fn ui<B>(f: &mut Frame<B>, area: Rect, app: &mut App)
where
    B: Backend,
{
    let border_type = if app.screen == SelectedScreen::RaceList {
        BorderType::Thick
    } else {
        BorderType::Plain
    };

    let block = Block::default()
        .title("Race Weekends")
        .borders(Borders::ALL)
        .border_type(border_type);

    if app.weekends.contains_key(&app.current_season) {
        let weekends: Vec<ListItem> = app.weekends[&app.current_season]
            .items
            .iter()
            .map(|i| ListItem::new(i.name.clone()))
            .collect();

        let weekend_list = List::new(weekends)
            .block(block)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        f.render_stateful_widget(
            weekend_list,
            area,
            &mut app.weekends.get_mut(&app.current_season).unwrap().state,
        );
    } else {
        let loading = Paragraph::new("Loading race weekends...").block(block);
        f.render_widget(loading, area);
    }
}
