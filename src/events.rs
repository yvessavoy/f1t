use crate::{load_weekends_for_season, App, SelectedScreen};
use crossterm::event::{Event, KeyCode};
use std::{sync::mpsc::Sender, time::Duration};

pub fn handle(app: &mut App, tx: &Sender<i32>) -> Result<bool, std::io::Error> {
    if crossterm::event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = crossterm::event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Char('j') => {
                    if app.screen == SelectedScreen::SeasonList {
                        app.seasons.next();
                        load_weekends_for_season(app, tx.clone());
                    } else {
                        app.weekends.get_mut(&app.current_season).unwrap().next();
                    }
                }
                KeyCode::Char('k') => {
                    if app.screen == SelectedScreen::SeasonList {
                        app.seasons.previous();
                        load_weekends_for_season(app, tx.clone());
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

    Ok(false)
}
