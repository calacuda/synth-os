use std::{
    sync::{Arc, RwLock},
    thread::{JoinHandle, spawn},
};

use channel_editor::channel_editor;
use helpers::IndexLessThan;
use iced::{
    Task, Theme,
    widget::{Column, Row, Text, row},
};
use midi_control::MidiMessage;
use midir::{Ignore, MidiInput, PortInfoError};
use sidebar::side_bar;
use stepper_synth::{
    CHANNEL_SIZE, HashMap, MidiControlled, SAMPLE_RATE, SampleGen,
    pygame_coms::SynthEngineType,
    sequencer::SequenceChannel,
    synth_engines::{
        Synth, SynthModule,
        wave_table::wavetable_synth::config::{N_ENV, N_LFO, N_OSC},
    },
};
use strum::EnumIter;
use tinyaudio::{OutputDevice, OutputDeviceParameters, run_output_device};
use tracing::*;

pub mod channel_editor;
pub mod helpers;
pub mod sidebar;

#[derive(Debug, Clone, Copy, Default, EnumIter)]
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

impl ToString for Screen {
    fn to_string(&self) -> String {
        match *self {
            Self::MidiStepper => "Step.",
            Self::MidiSequenser => "Seq.",
            Self::ChannelEditor => "Chan.",
            Self::ChannelA => "A",
            Self::ChannelB => "B",
            Self::ChannelC => "C",
            Self::ChannelD => "D",
            Self::Settings => "Set",
        }
        .into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WaveTableOscMessage {
    SetLevel(f32),
    SetOffset(i16),
    SetDetune(f32),
    // TODO: Write wavetabe settigns in backend
    // SetWaveTable(),
    /// a true value will turn on the oscilator
    SetPower(bool),
}

#[derive(Debug, Clone, Copy)]
pub enum WaveTableEnvMessage {
    SetAtk(f32),
    SetDcy(f32),
    SetSus(f32),
    SetRel(f32),
}

#[derive(Debug, Clone, Copy)]
pub enum WaveTableLfoMessage {
    SetSpeed(f32),
}

#[derive(Debug, Clone, Copy)]
pub enum WaveTableLPFilterMessage {
    SetCutoff(f32),
    SetResonance(f32),
    SetMix(f32),
    SetKeytrack(bool),
}

#[derive(Debug, Clone, Copy)]
pub enum WaveTableMessage {
    Osc {
        osc: IndexLessThan<{ N_OSC }>,
        msg: WaveTableOscMessage,
    },
    Env {
        env: IndexLessThan<{ N_ENV }>,
        msg: WaveTableEnvMessage,
    },
    Lfo {
        lfo: IndexLessThan<{ N_LFO }>,
        msg: WaveTableLfoMessage,
    },
    LPFilter {
        filter: IndexLessThan<2>,
        msg: WaveTableLPFilterMessage,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum ChannelMessage {
    ChangeInstrument(SynthEngineType),
    WaveTableMessage(WaveTableMessage),
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    /// changes wht screen the UI is set to.
    ScreenChange(Screen),
    ChannelMsg {
        channel: SequenceChannel,
        message: ChannelMessage,
    },
}

pub struct App {
    /// describes what screen the user is on and holds screen specific data.
    screen: Screen,
    // /// the websocket connection that comunicates with the synth backend
    // socket:
    /// the state of the synth
    synth: Arc<RwLock<Synth>>,
    /// audio device
    _device: Result<OutputDevice, Box<dyn std::error::Error>>,
    _midi_jh: JoinHandle<()>,
}

impl Default for App {
    fn default() -> Self {
        Self::new(Screen::default())
    }
}

impl App {
    fn new(screen: Screen) -> Self {
        let synth = Arc::new(RwLock::new(Synth::new()));
        let params = OutputDeviceParameters {
            channels_count: 1,
            sample_rate: SAMPLE_RATE as usize,
            channel_sample_count: CHANNEL_SIZE,
        };
        // NOTE: must stay in this thread so that it stays in scope
        let _device = run_output_device(params, {
            let synth = synth.clone();

            move |data| {
                for samples in data.chunks_mut(params.channels_count) {
                    if let Ok(mut synth) = synth.write() {
                        let value = synth.get_sample();

                        for sample in samples {
                            *sample = value;
                        }
                    } else {
                        error!("failled to lock synth as mutable")
                    }
                }
            }
        });

        if let Err(ref e) = _device {
            error!("starting audio playback caused error: {e}");
        }

        let _midi_jh = spawn({
            let synth = synth.clone();

            move || {
                if let Err(e) = run_midi(synth) {
                    error!("{e}");
                }
            }
        });

        // (
        Self {
            screen: screen,
            synth,
            _device,
            _midi_jh,
        }
        //     Task::batch([
        //         Task::perform(echo::server::run(), |_| Message::Server),
        //         widget::focus_next(),
        //     ]),
        // )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ScreenChange(screen) => {
                debug!("screen set to {}", screen.to_string());
                self.screen = screen
            }
            Message::ChannelMsg {
                channel,
                message: channel_msg,
            } => {
                if let Ok(ref mut synth) = self.synth.write() {
                    match channel_msg {
                        ChannelMessage::ChangeInstrument(instrument) => {
                            synth.set_channel_engine(channel as usize, instrument);
                        }
                        ChannelMessage::WaveTableMessage(wt_msg) => {
                            if let SynthModule::WaveTable(ref mut wt) =
                                synth.get_channel_engine(channel as usize).engine
                            {
                                match wt_msg {
                                    WaveTableMessage::Osc { osc, msg: osc_msg } => match osc_msg {
                                        WaveTableOscMessage::SetLevel(level) => wt
                                            .synth
                                            .voices
                                            .iter_mut()
                                            .for_each(|v| v.oscs[*osc].0.level = level),
                                        WaveTableOscMessage::SetOffset(offset) => wt
                                            .synth
                                            .voices
                                            .iter_mut()
                                            .for_each(|v| v.oscs[*osc].0.offset = offset),
                                        WaveTableOscMessage::SetDetune(detune) => wt
                                            .synth
                                            .voices
                                            .iter_mut()
                                            .for_each(|v| v.oscs[*osc].0.detune = detune),
                                        WaveTableOscMessage::SetPower(power) => wt
                                            .synth
                                            .voices
                                            .iter_mut()
                                            .for_each(|v| v.oscs[*osc].1 = power),
                                    },
                                    WaveTableMessage::Env { env, msg: env_msg } => match env_msg {
                                        WaveTableEnvMessage::SetAtk(val) => wt
                                            .synth
                                            .voices
                                            .iter_mut()
                                            .for_each(|v| v.envs[*env].set_atk(val)),
                                        WaveTableEnvMessage::SetDcy(val) => wt
                                            .synth
                                            .voices
                                            .iter_mut()
                                            .for_each(|v| v.envs[*env].set_decay(val)),
                                        WaveTableEnvMessage::SetSus(val) => wt
                                            .synth
                                            .voices
                                            .iter_mut()
                                            .for_each(|v| v.envs[*env].set_sus(val)),
                                        WaveTableEnvMessage::SetRel(val) => wt
                                            .synth
                                            .voices
                                            .iter_mut()
                                            .for_each(|v| v.envs[*env].set_release(val)),
                                    },
                                    WaveTableMessage::Lfo { lfo, msg: lfo_msg } => match lfo_msg {
                                        WaveTableLfoMessage::SetSpeed(speed) => {
                                            wt.synth.lfos[*lfo].set_frequency(1.0 / speed)
                                        }
                                    },
                                    WaveTableMessage::LPFilter {
                                        filter,
                                        msg: lp_fitler_msg,
                                    } => match lp_fitler_msg {
                                        WaveTableLPFilterMessage::SetCutoff(cutoff) => wt
                                            .synth
                                            .voices
                                            .iter_mut()
                                            .for_each(|v| v.filters[*filter].set_cutoff(cutoff)),
                                        WaveTableLPFilterMessage::SetResonance(res) => wt
                                            .synth
                                            .voices
                                            .iter_mut()
                                            .for_each(|v| v.filters[*filter].set_resonace(res)),
                                        WaveTableLPFilterMessage::SetMix(mix) => wt
                                            .synth
                                            .voices
                                            .iter_mut()
                                            .for_each(|v| v.filters[*filter].mix = mix),
                                        WaveTableLPFilterMessage::SetKeytrack(track) => wt
                                            .synth
                                            .voices
                                            .iter_mut()
                                            .for_each(|v| v.filters[*filter].key_track = track),
                                    },
                                }
                            } else {
                                error!(
                                    "the channel \"{channel:?}\" is a \"{}\". it was expected to be a \"{}\".",
                                    synth.get_channel_engine(channel as usize).engine_type,
                                    SynthEngineType::WaveTable
                                )
                            }
                        }
                    }
                } else {
                    error!("failed to lock synth with write access.")
                }
            }
        }

        Task::none()
    }

    fn view(&self) -> Row<Message> {
        let mut dis = row![side_bar(self.screen)];

        // if let Some(screem) = self.screen {
        // if let Some(screen) = match state {
        //     // TODO: make this a per screen state that can be requested
        //     StepperSynthState::MidiStepper {
        //         name,
        //         playing,
        //         recording,
        //         cursor,
        //         tempo,
        //         step,
        //         sequence,
        //         seq_n,
        //         channel,
        //     } => None::<Column<Message>>,
        //     StepperSynthState::Synth {
        //         engine,
        //         knob_params,
        //         gui_params,
        //     } => None::<Column<Message>>,
        //     StepperSynthState::WaveTable {
        //         osc,
        //         filter,
        //         adsr,
        //         lfo,
        //         mod_matrix,
        //     } => None::<Column<Message>>,
        //     StepperSynthState::Effect {
        //         effect: _,
        //         effect_on: _,
        //         params: _,
        //     } => None::<Column<Message>>,
        // } {
        //     dis = dis.push(screen)
        // }
        if let Ok(synth) = self.synth.read() {
            if let Some(screen) = match self.screen {
                Screen::MidiStepper => None::<Column<Message>>,
                Screen::MidiSequenser => None::<Column<Message>>,
                Screen::ChannelEditor => Some(channel_editor(&synth)),
                Screen::ChannelA => None::<Column<Message>>,
                Screen::ChannelB => None::<Column<Message>>,
                Screen::ChannelD => None::<Column<Message>>,
                Screen::ChannelC => None::<Column<Message>>,
                Screen::Settings => None::<Column<Message>>,
            } {
                dis = dis.push(screen);
            } else {
                dis = dis.push(Text::new("not implemented").center());
            }
        } else {
            dis = dis.push(Text::new("").center());
        }

        dis
    }
}

fn run_midi(synth: Arc<RwLock<Synth>>) -> anyhow::Result<()> {
    let mut registered_ports = HashMap::default();

    loop {
        let mut midi_in = MidiInput::new("midir reading input")?;
        midi_in.ignore(Ignore::None);

        // Get an input port (read from console if multiple are available)
        let in_ports = midi_in.ports();
        let port_names: Vec<std::result::Result<String, PortInfoError>> = in_ports
            .iter()
            .map(|port| midi_in.port_name(port))
            .collect();
        registered_ports.retain(|k: &String, _| port_names.contains(&Ok(k.clone())));

        for in_port in in_ports.iter() {
            let Ok(port_name) = midi_in.port_name(in_port) else {
                continue;
            };

            if registered_ports.contains_key(&port_name) {
                continue;
            }

            info!("port {port_name}");
            let mut midi_in = MidiInput::new("midir reading input")?;
            midi_in.ignore(Ignore::None);

            registered_ports.insert(
                port_name,
                midi_in.connect(
                    in_port,
                    "midir-read-input",
                    {
                        let synth = synth.clone();
                        move |_stamp, message, _| {
                            let message = MidiMessage::from(message);

                            // do midi stuff
                            synth.write().unwrap().midi_input(&message);
                        }
                    },
                    (),
                ),
            );
        }
    }
}

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application("Synth OS", App::update, App::view)
        .theme(|_| Theme::CatppuccinMocha)
        // .subscription(f)
        .run()
}
