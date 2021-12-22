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
use std::time::Duration;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Modifier, Style};
use tui::widgets::{Block, BorderType, Borders, List, ListItem};
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
    fn new(tx: std::sync::mpsc::Sender<i32>) -> Self {
        let mut seasons = StatefulList::with_items(get_available_seasons());
        seasons.state.select(Option::Some(0));

        tx.send(seasons.items[0]).unwrap();

        let app = Self {
            screen: SelectedScreen::SeasonList,
            seasons,
            weekends: StatefulList::<F1Weekend>::new(),
        };

        //load_weekends_for_season(&mut app);
        app
    }
}

fn get_available_seasons() -> Vec<i32> {
    (EARLIEST_SEASON..Utc::now().year() + 1).rev().collect()
}

fn load_weekends_for_season(app: &mut App, tx: std::sync::mpsc::Sender<i32>) {
    app.weekends = StatefulList::<F1Weekend>::new();
    let selected_season = app.seasons.state.selected().unwrap_or(0);
    tx.send(app.seasons.items[selected_season]).unwrap();
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = std::sync::mpsc::channel::<Vec<F1Weekend>>();
    let (tx_trigger, rx_trigger) = std::sync::mpsc::channel::<i32>();

    std::thread::spawn(move || loop {
        if let Ok(year) = rx_trigger.recv() {
            let weekends = get_season(year).unwrap_or_default();
            tx.send(weekends).unwrap();
        }
    });

    let mut app = App::new(tx_trigger.clone());

    loop {
        if let Ok(weekends) = rx.try_recv() {
            app.weekends = StatefulList::with_items(weekends);
        }

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

            f.render_stateful_widget(season_list, chunks[0], &mut app.seasons.state);

            let border_type = if app.screen == SelectedScreen::RaceList {
                BorderType::Thick
            } else {
                BorderType::Plain
            };

            let block = Block::default()
                .title("Race Weekends")
                .borders(Borders::ALL)
                .border_type(border_type);

            if app.weekends.items.len() > 0 {
                if app.weekends.items[0].year
                    == app.seasons.items[app.seasons.state.selected().unwrap_or(0)]
                {
                    let weekends: Vec<ListItem> = app
                        .weekends
                        .items
                        .iter()
                        .map(|i| ListItem::new(i.name.clone()))
                        .collect();

                    let weekend_list = List::new(weekends)
                        .block(block)
                        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                        .highlight_symbol("> ");

                    f.render_stateful_widget(weekend_list, chunks[1], &mut app.weekends.state);
                } else {
                    let loading = Paragraph::new("Loading race weekends...").block(block);
                    f.render_widget(loading, chunks[1]);
                }
            } else {
                let loading = Paragraph::new("Loading race weekends...").block(block);
                f.render_widget(loading, chunks[1]);
            }

            let block = Block::default().title("Controls").borders(Borders::ALL);
            let controls =
                Paragraph::new("q = Quit, h = Left, j = Down, k = Up, l = Right").block(block);
            f.render_widget(controls, root[1]);
        })?;

        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = crossterm::event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('j') => {
                        if app.screen == SelectedScreen::SeasonList {
                            app.seasons.next();
                            load_weekends_for_season(&mut app, tx_trigger.clone());
                        } else {
                            app.weekends.next();
                        }
                    }
                    KeyCode::Char('k') => {
                        if app.screen == SelectedScreen::SeasonList {
                            app.seasons.previous();
                            load_weekends_for_season(&mut app, tx_trigger.clone());
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
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
