use crate::{Message, Screen};
use iced::{
    Alignment::Center,
    widget::{Column, button, column, text},
};

pub fn side_bar<'a>(_focused_screen: Screen) -> Column<'a, Message> {
    let b_size = 75;

    let button = |screen: Screen| {
        button(text(screen.to_string()).align_x(Center).align_y(Center))
            .padding(5)
            .width(b_size)
            .height(b_size)
            .on_press(Message::ScreenChange(screen))
    };

    column![
        button(Screen::MidiStepper),
        button(Screen::MidiSequenser),
        button(Screen::ChannelEditor),
        button(Screen::ChannelA),
        button(Screen::ChannelB),
        button(Screen::ChannelC),
        button(Screen::ChannelD),
        button(Screen::Settings),
    ]
}
