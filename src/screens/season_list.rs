use crate::{App, SelectedScreen};
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Modifier, Style};
use tui::widgets::{Block, BorderType, Borders};
use tui::widgets::{List, ListItem};
use tui::Frame;

pub fn ui<B>(f: &mut Frame<B>, area: Rect, app: &mut App)
where
    B: Backend,
{
    let seasons: Vec<ListItem> = app
        .seasons
        .items
        .iter()
        .map(|i| ListItem::new(i.to_string()))
        .collect();

    let border_type = if app.screen == SelectedScreen::SeasonList {
        BorderType::Thick
    } else {
        BorderType::Plain
    };

    let season_list = List::new(seasons)
        .block(
            Block::default()
                .title("Seasons")
                .borders(Borders::ALL)
                .border_type(border_type),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(season_list, area, &mut app.seasons.state);
}
