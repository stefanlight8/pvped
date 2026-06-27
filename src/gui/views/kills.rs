use iced::Length;
use iced::widget::{row, text};
use iced::{
    Alignment, Padding,
    alignment::Horizontal,
    widget::{column, container, scrollable, text::Wrapping},
};

use crate::{
    frags::{Frag, Target},
    gui::element::{Column, Container},
};

#[derive(Debug, Default)]
pub struct KillsState {
    frags: Vec<Frag>,
}

impl KillsState {
    pub fn extend(&mut self, frags: &[Frag]) {
        self.frags.extend(frags.iter().cloned());
        self.frags.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    }

    pub fn frag<'a, Message: 'a>(&self, frag: &'a Frag) -> Container<'a, Message> {
        let killer: Column<'a, Message> = match &frag.killer {
            Target::You => column![
                text("You"),
                text(
                    frag.ship
                        .as_ref()
                        .map(|s| s.to_string())
                        .unwrap_or_default()
                )
                .wrapping(Wrapping::WordOrGlyph)
            ]
            .into(), // TODO: replace with one variable
            Target::Player(name) => column![text(name).wrapping(Wrapping::WordOrGlyph)], // TODO: replace with one variable
        };
        let victim: Column<'a, Message> = match &frag.victim {
            Target::You => column![
                text("You").align_x(Alignment::End),
                text(
                    frag.ship
                        .as_ref()
                        .map(|s| s.to_string())
                        .unwrap_or_default()
                )
                .align_x(Alignment::End)
                .wrapping(Wrapping::WordOrGlyph)
            ]
            .align_x(Horizontal::Right)
            .into(), // TODO: replace with one variable
            Target::Player(name) => column![text(name).wrapping(Wrapping::WordOrGlyph)], // TODO: replace with one variable
        };

        container(
            column![
                row![
                    text(frag.timestamp.to_string())
                        .width(Length::Fill)
                        .wrapping(Wrapping::WordOrGlyph),
                    text(frag.star_system.clone().unwrap_or("Unknown".to_string()))
                        .width(Length::Fill)
                        .align_x(Alignment::End)
                        .wrapping(Wrapping::WordOrGlyph)
                ],
                row![
                    container(killer)
                        .width(Length::FillPortion(1))
                        .align_x(Horizontal::Left),
                    container(text("killed"))
                        .width(Length::Shrink)
                        .align_x(Horizontal::Center),
                    container(victim)
                        .width(Length::FillPortion(1))
                        .align_x(Horizontal::Right),
                ]
                .width(Length::Fill)
            ]
            .spacing(2)
            .padding(2),
        )
        .style(|theme| {
            if frag.is_kill() {
                container::success(theme)
            } else {
                container::danger(theme)
            }
        })
        .width(Length::Fill)
        .into()
    }

    pub fn view<'a, Message: 'a>(&'a self) -> Container<'a, Message> {
        container(scrollable(
            column(self.frags.iter().map(|entry| self.frag(entry).into()))
                .padding(Padding {
                    top: 6.,
                    bottom: 6.,
                    left: 6.,
                    right: 18., // because of scrollbar
                })
                .spacing(4),
        ))
        .into()
    }
}
