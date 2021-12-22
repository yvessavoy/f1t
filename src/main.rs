mod util;

use crate::util::StatefulList;
use chrono::prelude::*;
use crossterm::{
    event::{Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use f1_rs::historical::get_season;
use f1_rs::historical::Weekend as F1Weekend;
use std::io;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem, Widget};
use tui::Terminal;
use tui::{backend::CrosstermBackend, widgets::Paragraph};

const EARLIEST_SEASON: i32 = 1950;

#[derive(PartialEq)]
enum SelectedScreen {
    SeasonList,
    RaceList,
}

struct App {
    screen: SelectedScreen,
    seasons: StatefulList<i32>,
    weekends: StatefulList<F1Weekend>,
}

impl App {
    fn new() -> Self {
        let mut seasons = StatefulList::with_items(get_available_seasons());
        seasons.state.select(Option::Some(0));

        let mut app = Self {
            screen: SelectedScreen::SeasonList,
            seasons,
            weekends: StatefulList::<F1Weekend>::new(),
        };

        load_weekends_for_season(&mut app);
        app
    }
}

fn get_available_seasons() -> Vec<i32> {
    (EARLIEST_SEASON..Utc::now().year() + 1).rev().collect()
}

fn load_weekends_for_season(app: &mut App) {
    let selected_season = app.seasons.state.selected().unwrap_or(0);
    app.weekends =
        StatefulList::with_items(get_season(app.seasons.items[selected_season]).unwrap());
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let root = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
                .split(f.size());

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(root[0]);

            let seasons: Vec<ListItem> = app
                .seasons
                .items
                .iter()
                .map(|i| ListItem::new(i.to_string()))
                .collect();

            let block = Block::default().title("Seasons").borders(Borders::ALL);
            let season_list = List::new(seasons)
                .block(block)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("> ");

            f.render_stateful_widget(season_list, chunks[0], &mut app.seasons.state);

            let weekends: Vec<ListItem> = app
                .weekends
                .items
                .iter()
                .map(|i| ListItem::new(i.name.clone()))
                .collect();

            let season_list = List::new(weekends)
                .block(
                    Block::default()
                        .title("Race Weekends")
                        .borders(Borders::ALL),
                )
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("> ");

            f.render_stateful_widget(season_list, chunks[1], &mut app.weekends.state);

            let block = Block::default().title("Controls").borders(Borders::ALL);
            let controls =
                Paragraph::new("q = Quit, h = Left, j = Down, k = Up, l = Right").block(block);
            f.render_widget(controls, root[1]);
        })?;

        if let Event::Key(key) = crossterm::event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('j') => {
                    if app.screen == SelectedScreen::SeasonList {
                        app.seasons.next();
                        load_weekends_for_season(&mut app);
                    } else {
                        app.weekends.next();
                    }
                }
                KeyCode::Char('k') => {
                    if app.screen == SelectedScreen::SeasonList {
                        app.seasons.previous();
                        load_weekends_for_season(&mut app);
                    } else {
                        app.weekends.previous();
                    }
                }
                KeyCode::Char('h') => {
                    if app.screen == SelectedScreen::SeasonList {
                        app.screen = SelectedScreen::RaceList;
                        app.weekends.state.select(Option::Some(0));
                    } else {
                        app.screen = SelectedScreen::SeasonList;
                    }
                }
                KeyCode::Char('l') => {
                    if app.screen == SelectedScreen::SeasonList {
                        app.screen = SelectedScreen::RaceList;
                        app.weekends.state.select(Option::Some(0));
                    } else {
                        app.screen = SelectedScreen::SeasonList;
                    }
                }
                _ => (),
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
