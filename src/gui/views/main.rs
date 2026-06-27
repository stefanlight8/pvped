use {
    crate::{
        frags::Frag,
        gui::{
            element::Element,
            views::{
                kills::KillsState,
                players::PlayersState,
                plot::{PlotMessage, PlotState},
                statistics::StatisticsState,
            },
        },
        journals::{get_journals, scan_journals},
        settings::Settings,
    },
    iced::{
        Length, Task,
        widget::{button, column, container, row, scrollable},
    },
    std::path::PathBuf,
};

pub struct MainState {
    kills: KillsState,
    players: PlayersState,
    statistics: StatisticsState,
    plot: PlotState,
}

#[derive(Debug, Clone)]
pub enum MainMessage {
    Scan,
    Settings,
    Journals(Vec<PathBuf>),
    Frags(Vec<Frag>),
    Plot(PlotMessage),
    Error,
}

impl MainState {
    pub fn new() -> MainState {
        MainState {
            kills: KillsState::default(),
            players: PlayersState::default(),
            statistics: StatisticsState::new(),
            plot: PlotState::new(),
        }
    }

    pub fn update(&mut self, settings: &Settings, message: MainMessage) -> Task<MainMessage> {
        match message {
            MainMessage::Scan => {
                return Task::perform(get_journals(settings.journals_path()), |res| {
                    match res {
                        Ok(mut journals) => {
                            journals.sort_by(|a, b| b.file_name().cmp(&a.file_name()));

                            MainMessage::Journals(journals)
                        }
                        Err(err) => {
                            tracing::error!("failed to get journals: {}", err);

                            MainMessage::Error
                        } // TODO: error
                    }
                });
            }
            MainMessage::Journals(journals) => {
                return Task::stream(scan_journals(journals)).map(|res| match res {
                    Ok(frag) => MainMessage::Frags(frag),
                    Err(err) => {
                        tracing::error!("failed to scan journals: {}", err);

                        MainMessage::Error
                    } // TODO: error
                });
            }
            MainMessage::Frags(frags) => {
                tracing::debug!("received frags: {:?}", frags);

                self.kills.extend(&frags);
                self.plot.extend(&frags);
                self.players.extend(&frags);
                self.statistics.extend(&frags);
            }
            MainMessage::Plot(message) => self.plot.update(message),
            _ => (),
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, MainMessage> {
        row![
            self.kills.view().width(Length::Fill),
            self.players.view().width(Length::Fill),
            column![
                container(self.plot.view().map(MainMessage::Plot))
                    .padding(6)
                    .height(Length::Fill),
                scrollable(
                    container(self.statistics.view())
                        .padding(6)
                        .height(Length::Fill)
                ),
                container(
                    row![
                        button("Scan").on_press(MainMessage::Scan),
                        button("Settings").on_press(MainMessage::Settings)
                    ]
                    .spacing(6)
                )
                .padding(6)
                .width(Length::Fill)
            ]
            .width(Length::Fill)
        ]
        .padding(6)
        .into()
    }
}
