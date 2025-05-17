use crate::{Message, Screen};
use iced::{
    Alignment::Center,
    widget::{Column, button, column, text},
};

pub fn side_bar<'a>(screen: Screen) -> Column<'a, Message> {
    let b_size = 75;

    let button = |label| {
        button(text(label).align_x(Center))
            .padding(20)
            .width(b_size)
            .height(b_size)
    };

    column![
        button("Step."),
        button("Seq."),
        button("Chan"),
        button("A"),
        button("B"),
        button("C"),
        button("D"),
        button("Set."),
    ]
}
