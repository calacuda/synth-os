#![feature(let_chains)]

use color_eyre::Result;
use ratatui::{
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::{error::Error, fmt::Display, io};
use strum::IntoEnumIterator;
use tokens::Nodes;
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

pub mod tokens;

// pub type Cmd = Vec<Box<dyn CmdToken>>;

// pub trait CanEnumIter: Clone {
//     fn into_vec() -> Vec<Self>;
// }
//
// impl<T> CanEnumIter for T
// where
//     T: IntoEnumIterator + Clone,
// {
//     fn into_vec() -> Vec<Self> {
//         Self::iter().collect()
//     }
// }
//
// pub trait CmdToken: std::fmt::Debug + Clone + CanEnumIter + Display {
//     // fn get_hildren() -> Vec<Self>;
//     fn get_desc() -> Vec<(String, String)> {
//         Self::into_vec()
//             .into_iter()
//             .map(|token| (format!("{token}"), token.get_one_desc()))
//             .collect()
//     }
//     fn get_one_desc(&self) -> String;
//
//     // fn match_str(against: &str) -> (Vec<(String, String)>, bool);
//     // /// semi-recursive function to add the string repr of self to,
//     // fn render(&self, cmd: &mut Vec<String>) -> Vec<String>;
//     // fn call(&self);
// }

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: Input,
    token: String,
    // /// History of recorded messages
    // history: Vec<String>,
    valid_tokens: Nodes,
}

impl Default for App {
    fn default() -> App {
        App {
            input: Input::default(),
            // history: Vec::new(),
            token: String::new(),
            valid_tokens: Nodes::default(),
        }
    }
}

impl App {
    fn get_tokens(&self) -> Vec<(String, String)> {
        // let tokens = self.valid_tokens;
        self.valid_tokens
            .get_desc()
            .into_iter()
            // .zip(self.valid_tokens.valid_children())
            .filter_map(|item| {
                if item
                    .0
                    // .0
                    .to_lowercase()
                    .starts_with(&self.token.to_lowercase())
                {
                    Some(item)
                } else {
                    None
                }
            })
            .collect()
    }

    fn step(&mut self) {
        if let Some(token) = self.input.to_string().split_whitespace().clone().last()
            && !self.input.to_string().ends_with(" ")
        {
            self.token = token.to_string();
        } else if let Some(token) = self.input.to_string().split_whitespace().clone().last()
            && self.input.to_string().ends_with(" ")
        {
            self.valid_tokens.collapse(token);
            self.valid_tokens = self.valid_tokens.transition();
            self.token.clear();
        } else {
            self.token.clear();
        }

        if self.input.to_string().is_empty() {
            self.valid_tokens = Nodes::default();
        }
    }

    fn reset(&mut self) {
        self.input.reset();
        self.token.clear();
        self.valid_tokens = Nodes::default();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    match res {
        Ok(Some(path)) => println!("{}", path.replace(" ", "/")),
        Ok(None) => {
            println!("QUIT")
        }
        Err(err) => {
            println!("{:?}", err)
        }
    }

    Ok(())
}

// fn main() {
//     let mut app = App::default();
//
//     app.input.handle_event(&Event::Key(KeyEvent::new(
//         KeyCode::Char('e'),
//         KeyModifiers::empty(),
//     )));
//
//     println!("{:?}", app.get_tokens());
//
//     app.valid_tokens.collapse("engine");
//     app.valid_tokens = app.valid_tokens.transition();
//
//     println!("{:?}", app.get_tokens());
// }

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<Option<String>> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Enter => {
                    let cmd: String = app.input.value().into();
                    // app.history.push(cmd.clone());

                    if cmd.to_lowercase() == "quit" {
                        break Ok(None);
                    } else {
                        // Send Command to backend over *unix-domain-socket*

                        break Ok(Some(app.input.to_string()));
                    }

                    // app.input.reset();
                }
                _ => {
                    app.input.handle_event(&Event::Key(key));
                    app.step();
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.area());

    let (msg, style) = (
        vec![
            Span::raw("Press "),
            // Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            // Span::raw(" to stop editing, "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to run the command"),
        ],
        Style::default(),
    );
    let text = Text::from(Line::from(msg)).style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let width = chunks[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor

    let scroll = app.input.visual_scroll(width as usize);
    let input = Paragraph::new(app.input.value())
        .style(Style::default().fg(Color::Yellow))
        .scroll((0, scroll as u16))
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);

    // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
    f.set_cursor_position((
        // Put cursor past the end of the input text
        chunks[1].x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
        // Move one line down, from the border to the input line
        chunks[1].y + 1,
    ));

    // filter based on current token
    let tokens = app.get_tokens();

    let messages: Vec<ListItem> = tokens
        .into_iter()
        .enumerate()
        .map(|(i, (token, desc))| {
            let content = vec![
                Line::from(Span::styled(
                    format!("{i}: {token}"),
                    Style::default().add_modifier(Modifier::BOLD),
                )),
                Line::from(Span::raw(format!("    {desc}"))),
            ];
            ListItem::new(content)
        })
        .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Commands"));
    f.render_widget(messages, chunks[2]);
}
