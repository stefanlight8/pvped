use std::collections::HashMap;

use iced::{
    Alignment, Length,
    widget::{Container, column, container, row, scrollable, text},
};

use crate::frags::{Frag, Target};

#[derive(Debug, Default)]
struct PlayerStats {
    fights: usize,
    kills: usize,
    deaths: usize,
    kd: f32,
}

impl PlayerStats {
    fn kd(&mut self) -> f32 {
        if self.deaths == 0 {
            self.kills as f32
        } else {
            self.kills as f32 / self.deaths as f32
        }
    }
}

#[derive(Debug, Default)]
pub struct PlayersState {
    players: HashMap<String, PlayerStats>,
}

fn player_view<'a, Message: 'a>(name: &'a str, stats: &PlayerStats) -> Container<'a, Message> {
    container(
        row![
            text(name).size(16).width(Length::FillPortion(2)),
            text(format!("{} fights", stats.fights))
                .size(14)
                .width(Length::FillPortion(1)),
            container(row![text(stats.kills).style(text::success), text("K")].spacing(2))
                .padding([2, 6])
                .width(Length::FillPortion(1)),
            container(row![text(stats.deaths).style(text::danger), text("D")].spacing(2))
                .padding([2, 6])
                .width(Length::FillPortion(1)),
            text(format!("{:.2} KD", stats.kd)).width(Length::FillPortion(1)),
        ]
        .align_y(Alignment::Center)
        .spacing(12),
    )
    .padding([6, 10])
    .width(Length::Fill)
}

impl PlayersState {
    pub fn extend(&mut self, frags: &[Frag]) {
        for frag in frags {
            match (&frag.killer, &frag.victim) {
                (Target::You, Target::Player(name)) => {
                    let stats = self.players.entry(name.clone()).or_default();
                    stats.fights += 1;
                    stats.kills += 1;
                    stats.kd = stats.kd();
                }
                (Target::Player(name), Target::You) => {
                    let stats = self.players.entry(name.clone()).or_default();
                    stats.fights += 1;
                    stats.deaths += 1;
                    stats.kd = stats.kd();
                }
                _ => {}
            }
        }
    }

    pub fn view<'a, Message: 'a>(&'a self) -> Container<'a, Message> {
        let mut players: Vec<_> = self.players.iter().collect();

        players.sort_by(|a, b| b.1.fights.cmp(&a.1.fights));

        container(scrollable(
            column(
                players
                    .iter()
                    .map(|player| player_view(player.0, player.1).into()),
            )
            .spacing(6),
        ))
    }
}
