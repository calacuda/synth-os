use iced::{
    Task, Theme,
    widget::{Row, row},
};
use sidebar::side_bar;
use strum::{Display, EnumIter, EnumString};
use tracing::debug;

pub mod sidebar;

#[derive(Debug, Clone, Copy, Default, EnumIter, EnumString, Display)]
pub enum Screen {
    // #[default]
    // Loading,
    #[default]
    MidiStepper,
    MidiSequenser,
    ChannelEditor,
    ChannelA,
    ChannelB,
    ChannelC,
    ChannelD,
    Settings,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    /// changes wht screen the UI is set to.
    ScreenChange(Screen),
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

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ScreenChange(screen) => {
                debug!("screen set to {screen}");
                self.screen = screen
            }
        }

        Task::none()
    }

    fn view(&self) -> Row<Message> {
        row![side_bar(self.screen)]
    }
}

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application("Synth OS", App::update, App::view)
        .theme(|_| Theme::CatppuccinMocha)
        // .subscription(f)
        .run()
}
