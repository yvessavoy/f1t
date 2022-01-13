use crate::{
    load_weekends_for_season,
    util::{StatefulList, TabsState},
    App, SelectedScreen,
};
use crossterm::event::{Event, KeyCode};
use std::{sync::mpsc::Sender, time::Duration};

pub fn handle(app: &mut App, tx: &Sender<i32>) -> Result<bool, std::io::Error> {
    if crossterm::event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = crossterm::event::read()? {
            match key.code {
                KeyCode::Char('q') => match app.screen {
                    SelectedScreen::WeekendResults => app.screen = SelectedScreen::RaceList,
                    _ => return Ok(true),
                },
                KeyCode::Char('j') => match app.screen {
                    SelectedScreen::SeasonList => {
                        app.seasons.next();
                        load_weekends_for_season(app, tx.clone());
                    }
                    SelectedScreen::RaceList => {
                        app.weekends.get_mut(&app.current_season).unwrap().next();
                    }
                    SelectedScreen::WeekendResults => {
                        app.ranking.next();
                    }
                },
                KeyCode::Char('k') => match app.screen {
                    SelectedScreen::SeasonList => {
                        app.seasons.previous();
                        load_weekends_for_season(app, tx.clone());
                    }
                    SelectedScreen::RaceList => {
                        app.weekends
                            .get_mut(&app.current_season)
                            .unwrap()
                            .previous();
                    }
                    SelectedScreen::WeekendResults => {
                        app.ranking.previous();
                    }
                },
                KeyCode::Char('h') => match app.screen {
                    SelectedScreen::SeasonList => {
                        if let Some(weekends) = app.weekends.get_mut(&app.current_season) {
                            app.screen = SelectedScreen::RaceList;
                            weekends.state.select(Option::Some(0));
                        }
                    }
                    SelectedScreen::RaceList => {
                        app.screen = SelectedScreen::SeasonList;
                    }
                    SelectedScreen::WeekendResults => {
                        app.detail_tabs.previous();

                        let hm = app.weekends.get(&app.current_season).unwrap();
                        let weekend = hm.items.get(hm.state.selected().unwrap()).unwrap();
                        let standings = weekend.sessions[app.detail_tabs.index].clone().standings;
                        app.ranking = StatefulList::with_items(standings);
                        app.ranking.state.select(Some(0));
                    }
                },
                KeyCode::Char('l') => {
                    match app.screen {
                        SelectedScreen::SeasonList => {
                            if let Some(weekends) = app.weekends.get_mut(&app.current_season) {
                                app.screen = SelectedScreen::RaceList;
                                weekends.state.select(Option::Some(0));
                            }
                        }
                        SelectedScreen::RaceList => {
                            app.screen = SelectedScreen::SeasonList;
                        }
                        SelectedScreen::WeekendResults => {
                            app.detail_tabs.next();

                            let hm = app.weekends.get(&app.current_season).unwrap();
                            let weekend = hm.items.get(hm.state.selected().unwrap()).unwrap();
                            let standings =
                                weekend.sessions[app.detail_tabs.index].clone().standings;
                            app.ranking = StatefulList::with_items(standings);
                            app.ranking.state.select(Some(0));
                        }
                    };
                }
                KeyCode::Enter => {
                    if app.screen == SelectedScreen::RaceList {
                        app.screen = SelectedScreen::WeekendResults;

                        let hm = app.weekends.get(&app.current_season).unwrap();
                        let weekend = hm.items.get(hm.state.selected().unwrap()).unwrap();

                        app.detail_tabs = TabsState::new(
                            weekend
                                .sessions
                                .iter()
                                .map(|s| s.r#type.to_string())
                                .collect(),
                        );

                        let standings = weekend.sessions[app.detail_tabs.index].clone().standings;
                        app.ranking = StatefulList::with_items(standings);
                        app.ranking.state.select(Some(0));
                    }
                }
                _ => (),
            }
        }
    }

    Ok(false)
}
