use iced::{
    Task, Theme,
    widget::{Row, row},
};

#[derive(Debug, Clone, Copy, Default)]
pub enum Screen {
    #[default]
    Loading,
    MidiStepper,
    MidiSequenser,
    ChannelEditor,
    ChannelA,
    ChannelB,
    ChannelC,
    ChannelD,
    Settings,
}

pub struct App {
    /// describes what screen the user is on and holds screen specific data.
    screen: Screen,
    // /// the websocket connection that comunicates with the synth backend
    // socket:
}

impl Default for App {
    fn default() -> Self {
        Self::new(Screen::default())
    }
}

impl App {
    fn new(screen: Screen) -> Self {
        // (
        Self { screen: screen }
        //     Task::batch([
        //         Task::perform(echo::server::run(), |_| Message::Server),
        //         widget::focus_next(),
        //     ]),
        // )
    }

    fn update(&mut self, message: Screen) -> Task<Screen> {
        Task::none()
    }

    fn view(&self) -> Row<Screen> {
        row!["foobar"]
    }
}

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application("Synth OS", App::update, App::view)
        .theme(|_| Theme::CatppuccinMocha)
        // .subscription(f)
        .run()
}
