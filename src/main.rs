mod screens;
mod util;

use crate::util::StatefulList;
use crossterm::{
    event::{Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use f1::historical::get_season;
use f1::historical::Weekend as F1Weekend;
use std::collections::HashMap;
use std::io;
use std::time::Duration;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Terminal;

#[derive(PartialEq)]
enum SelectedScreen {
    SeasonList,
    RaceList,
}

pub struct App {
    screen: SelectedScreen,
    current_season: i32,
    seasons: StatefulList<i32>,
    weekends: HashMap<i32, StatefulList<F1Weekend>>,
}

impl App {
    fn new(tx: std::sync::mpsc::Sender<i32>) -> Self {
        let mut seasons = StatefulList::with_items(f1::get_available_seasons());
        seasons.state.select(Option::Some(0));

        tx.send(seasons.items[0]).unwrap();

        let app = Self {
            current_season: seasons.items[0],
            screen: SelectedScreen::SeasonList,
            seasons,
            weekends: HashMap::new(),
        };

        app
    }
}

fn load_weekends_for_season(app: &mut App, tx: std::sync::mpsc::Sender<i32>) {
    let selected_season = app.seasons.state.selected().unwrap_or(0);
    app.current_season = app.seasons.items[selected_season];
    if !app.weekends.contains_key(&app.current_season) {
        tx.send(app.current_season).unwrap();
    }
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = std::sync::mpsc::channel::<(i32, Vec<F1Weekend>)>();
    let (tx_trigger, rx_trigger) = std::sync::mpsc::channel::<i32>();

    std::thread::spawn(move || loop {
        if let Ok(year) = rx_trigger.recv() {
            let weekends = get_season(year).unwrap_or_default();
            tx.send((year, weekends)).unwrap();
        }
    });

    let mut app = App::new(tx_trigger.clone());

    loop {
        if let Ok((year, weekends)) = rx.try_recv() {
            app.weekends
                .insert(year, StatefulList::with_items(weekends));
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

            screens::season_list::ui(f, chunks[0], &mut app);
            screens::weekend_list::ui(f, chunks[1], &mut app);
            screens::footer::ui(f, root[1]);
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
                            app.weekends.get_mut(&app.current_season).unwrap().next();
                        }
                    }
                    KeyCode::Char('k') => {
                        if app.screen == SelectedScreen::SeasonList {
                            app.seasons.previous();
                            load_weekends_for_season(&mut app, tx_trigger.clone());
                        } else {
                            app.weekends
                                .get_mut(&app.current_season)
                                .unwrap()
                                .previous();
                        }
                    }
                    KeyCode::Char('h') => {
                        if app.screen == SelectedScreen::SeasonList {
                            app.screen = SelectedScreen::RaceList;
                            app.weekends
                                .get_mut(&app.current_season)
                                .unwrap()
                                .state
                                .select(Option::Some(0));
                        } else {
                            app.screen = SelectedScreen::SeasonList;
                        }
                    }
                    KeyCode::Char('l') => {
                        if app.screen == SelectedScreen::SeasonList {
                            app.screen = SelectedScreen::RaceList;
                            app.weekends
                                .get_mut(&app.current_season)
                                .unwrap()
                                .state
                                .select(Option::Some(0));
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
