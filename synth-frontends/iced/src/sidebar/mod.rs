use crate::{Message, Screen};
use iced::{
    Alignment::Center,
    widget::{Column, button, column, text},
};

pub fn side_bar<'a>(screen: Screen) -> Column<'a, Message> {
    let button = |label| {
        button(text(label).align_x(Center))
            .padding(10)
            .width(80)
            .height(80)
    };

    column![
        button("Midi Step"),
        button("Midi Seq"),
        button("Channels"),
        button("Channel A"),
        button("Channel B"),
        button("Channel C"),
        button("Channel D"),
        button("Settings"),
    ]
}
