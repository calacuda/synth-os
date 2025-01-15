use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::{CanEnumIter, CmdToken};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter, Display)]
pub enum EngineType {
    Organ,
    SubtractiveSynth,
}

impl CmdToken for EngineType {
    fn get_one_desc(&self) -> String {
        match *self {
            Self::Organ => "Organ sound, (loosely) moddeled after a Hamanond B3".into(),
            Self::SubtractiveSynth => "A fairly standard subtractive synth.".into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter, Display)]
pub enum EffectType {
    Reverb,
    Chorus,
}

impl CmdToken for EffectType {
    fn get_one_desc(&self) -> String {
        match *self {
            Self::Reverb => "Reverb Effect".into(),
            Self::Chorus => "Chorus Effect".into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter, Display)]
pub enum Screen {
    /// Vital
    Wurlitzer,
    Synth,  // (Option<EngineType>),
    Effect, // (Option<EffectType>),
    LFO,    // (Option<usize>),
    ModMatrix,
    Sequencer, // (Option<usize>),
}

impl CmdToken for Screen {
    fn get_one_desc(&self) -> String {
        match *self {
            Self::Wurlitzer => "The Wurlitzer screen (powered by Vital)".into(),
            Self::Synth => "Multiple synth engines".into(),
            Self::Effect => "Effects modifying screen".into(),
            Self::LFO => "LFO modifying screen".into(),
            Self::ModMatrix => "Mod-Matrix screen".into(),
            Self::Sequencer => "Midi Sequencer".into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter, Display)]
pub enum Knob {
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
}

impl CmdToken for Knob {
    fn get_one_desc(&self) -> String {
        format!("{self}")
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter, Display)]
pub enum Gui {
    G1,
    G2,
    G3,
    G4,
    G5,
    G6,
    G7,
    G8,
}

impl CmdToken for Gui {
    fn get_one_desc(&self) -> String {
        format!("{self}")
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum GenericParam {
    Knob(Knob),
    Gui(Gui),
}

impl Display for GenericParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Knob(knob) => write!(f, "{}", knob.get_one_desc()),
            Self::Gui(gui) => write!(f, "{}", gui.get_one_desc()),
        }
    }
}

impl CmdToken for GenericParam {
    fn get_one_desc(&self) -> String {
        match *self {
            Self::Knob(knob) => format!("Midi controlled knob: {}", knob.get_one_desc()),
            Self::Gui(gui) => format!("Gui editable parameter: {}", gui.get_one_desc()),
        }
    }
}

impl CanEnumIter for GenericParam {
    fn into_vec() -> Vec<Self> {
        Self::iter()
    }
}

impl GenericParam {
    // type Iterator = Vec<Self>;

    pub fn iter() -> Vec<Self> {
        let knobs = Knob::iter().map(|knob| GenericParam::Knob(knob));
        let mut guis: Vec<Self> = Gui::iter().map(|param| GenericParam::Gui(param)).collect();

        let mut v: Vec<Self> = knobs.collect();
        v.append(&mut guis);

        v
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter, Display)]
pub enum PowerState {
    On,
    Off,
}

impl CmdToken for PowerState {
    fn get_one_desc(&self) -> String {
        match *self {
            Self::On => "Makse the thing \"On\"".into(),
            Self::Off => "Makes the thing \"off\"".into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter, Display)]
pub enum EngineCmdArgs {
    Set,      // (GenericParam),
    SwitchTo, // (EngineType),
}

impl CmdToken for EngineCmdArgs {
    fn get_one_desc(&self) -> String {
        match *self {
            Self::Set => "Set's a synth engine parameter".into(),
            Self::SwitchTo => "Switch view, and default synth, to an engine".into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter, Display)]
pub enum EffectCmdArgs {
    Set,      // (GenericParam),
    SwitchTo, // (EffectType, Option<PowerState>),
}

impl CmdToken for EffectCmdArgs {
    fn get_one_desc(&self) -> String {
        match *self {
            Self::Set => "Set's an effect param".into(),
            Self::SwitchTo => "Switch view, and optionally power, for an effect".into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter, Display)]
pub enum ModSrc {}

impl CmdToken for ModSrc {
    fn get_one_desc(&self) -> String {
        match *self {}
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter, Display)]
pub enum ModDest {}

impl CmdToken for ModDest {
    fn get_one_desc(&self) -> String {
        match *self {}
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter, Display)]
pub enum MatrixCmdArgs {
    Connect,    // (ModSrc, ModDest, f32),
    Disconnect, // (),
}

impl CmdToken for MatrixCmdArgs {
    fn get_one_desc(&self) -> String {
        match *self {
            Self::Connect => "Make a new connection in the mod-matrix".into(),
            Self::Disconnect => "Remove a connection from the mod-matrix".into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct ModAmt(f32);

impl Display for ModAmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ModAmt")
    }
}

impl CanEnumIter for ModAmt {
    fn into_vec() -> Vec<Self> {
        vec![ModAmt(0.0)]
    }
}

impl CmdToken for ModAmt {
    fn get_one_desc(&self) -> String {
        "Modulation amount".into()
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct KnobValue(f32);

impl Display for KnobValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KnobValue")
    }
}

impl CanEnumIter for KnobValue {
    fn into_vec() -> Vec<Self> {
        vec![KnobValue(0.0)]
    }
}

impl CmdToken for KnobValue {
    fn get_one_desc(&self) -> String {
        "The knob's position".into()
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct LfoNum(usize);

impl Display for LfoNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl CanEnumIter for LfoNum {
    fn into_vec() -> Vec<Self> {
        vec![LfoNum(0)]
    }
}

impl CmdToken for LfoNum {
    fn get_one_desc(&self) -> String {
        "Which LFO?".into()
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct SeqNum(usize);

impl Display for SeqNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SeqNum")
    }
}

impl CanEnumIter for SeqNum {
    fn into_vec() -> Vec<Self> {
        vec![SeqNum(0)]
    }
}

impl CmdToken for SeqNum {
    fn get_one_desc(&self) -> String {
        "Which MIDI sequence".into()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter, Display)]
pub enum MainCmd {
    /// commands to change parameters of one of the multi-engine synths
    Engine, // (EngineCmdArgs),
    /// commands to change parameters of an effect
    Effect, // (EffectCmdArgs),
    /// chagne LFO parameters
    Lfo,
    /// make-new/break-old connections in the mod matrix
    Matrix, // (MatrixCmdArgs),
    /// Goto display
    GoTo, // (Screen),
}

impl CmdToken for MainCmd {
    fn get_one_desc(&self) -> String {
        match *self {
            Self::Engine => "Synth engine & their params".into(),
            Self::Effect => "Audio effect & their params".into(),
            Self::Lfo => "Controls over the LFOs".into(),
            Self::Matrix => "Edit the mod-matrix".into(),
            Self::GoTo => "Change the active screen".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeType<T> {
    SuperPos(Vec<T>),
    Collapsed(T),
}

impl<T> NodeType<T>
where
    T: std::fmt::Display + CmdToken,
{
    fn get_desc(&self) -> Vec<(String, String)> {
        match self.clone() {
            Self::Collapsed(node) => vec![(format!("{node}"), node.get_one_desc())],
            Self::SuperPos(nodes) => nodes
                .iter()
                .map(|node| (format!("{node}"), node.get_one_desc()))
                .collect(),
        }
    }
}

impl<T> NodeType<Option<T>>
where
    T: std::fmt::Display + CmdToken,
{
    fn get_desc(&self) -> Vec<(String, String)> {
        match self.clone() {
            Self::Collapsed(Some(node)) => vec![(
                format!("{node}"),
                format!("Optional: {}", node.get_one_desc()),
            )],
            Self::Collapsed(None) => Vec::new(),
            Self::SuperPos(nodes) => nodes
                .into_iter()
                .filter_map(|node| {
                    node.map(|node| {
                        (
                            format!("{node}"),
                            format!("Optional: {}", node.get_one_desc()),
                        )
                    })
                })
                .collect(),
        }
    }
}

impl<T> NodeType<Option<T>>
where
    T: IntoEnumIterator + CmdToken,
{
    fn op_default() -> Self {
        let mut v: Vec<Option<T>> = T::iter().map(Some).collect();
        v.push(None);

        Self::SuperPos(v)
    }
}

impl<T> Default for NodeType<T>
where
    T: IntoEnumIterator + CmdToken,
{
    fn default() -> Self {
        Self::SuperPos(T::iter().collect())
    }
}

impl Default for NodeType<GenericParam> {
    fn default() -> Self {
        Self::SuperPos(GenericParam::iter())
    }
}

// TODO: for the engine command, set which engine is being modified.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Nodes {
    Cmd(NodeType<MainCmd>),
    MatrixArgs(NodeType<MatrixCmdArgs>),
    ModSrc(NodeType<ModSrc>),
    ModDest(NodeType<ModDest>),
    ModAmt(NodeType<ModAmt>),
    EffectArgs(NodeType<EffectCmdArgs>),
    EngineArgs(NodeType<EngineCmdArgs>),
    PowerState(NodeType<PowerState>),
    GenericParam(NodeType<GenericParam>),
    KnobValue(NodeType<KnobValue>),
    Screen(NodeType<Screen>),
    EngineType(NodeType<EngineType>),
    EffectType(NodeType<EffectType>),
    ScreenSynthArgs(NodeType<Option<EngineType>>),
    ScreenEffectArgs(NodeType<Option<EffectType>>),
    ScreenLfoNum(NodeType<Option<LfoNum>>),
    ScreenSeqNum(NodeType<Option<SeqNum>>),
    Done,
}

impl Default for Nodes {
    fn default() -> Self {
        Self::Cmd(NodeType::SuperPos(MainCmd::iter().collect()))
    }
}

impl Nodes {
    pub fn transition(&self) -> Self {
        match self {
            Self::Cmd(NodeType::Collapsed(cmd)) => match cmd {
                MainCmd::Engine => Self::EngineArgs(NodeType::default()),
                MainCmd::Effect => Self::EffectArgs(NodeType::default()),
                MainCmd::Lfo => todo!("design lfo cmd interface"),
                MainCmd::Matrix => Self::MatrixArgs(NodeType::default()),
                MainCmd::GoTo => Self::Screen(NodeType::default()),
            },
            Self::MatrixArgs(NodeType::Collapsed(sub_cmd)) => match sub_cmd {
                MatrixCmdArgs::Connect => Self::ModSrc(NodeType::default()),
                MatrixCmdArgs::Disconnect => {
                    // Self::(NodeType::SuperPos(ModSrc::iter().collect()))
                    todo!("write this");
                }
            },
            Self::ModSrc(NodeType::Collapsed(_)) => {
                Self::ModDest(NodeType::SuperPos(ModDest::iter().collect()))
            }
            Self::ModDest(NodeType::Collapsed(_)) => {
                Self::ModAmt(NodeType::SuperPos(vec![ModAmt::default()]))
            }
            Self::ModAmt(NodeType::Collapsed(_)) => Self::Done,
            Self::EffectArgs(NodeType::Collapsed(sub_cmd)) => match sub_cmd {
                EffectCmdArgs::Set => Self::GenericParam(NodeType::default()),
                EffectCmdArgs::SwitchTo => Self::EffectType(NodeType::default()),
            },
            Self::EngineArgs(NodeType::Collapsed(sub_cmd)) => match sub_cmd {
                EngineCmdArgs::Set => Self::GenericParam(NodeType::default()),
                EngineCmdArgs::SwitchTo => Self::EngineType(NodeType::default()),
            },
            Self::PowerState(NodeType::Collapsed(_)) => Self::Done,
            Self::GenericParam(NodeType::Collapsed(_)) => {
                Self::KnobValue(NodeType::SuperPos(vec![KnobValue::default()]))
            }
            Self::KnobValue(NodeType::Collapsed(_)) => Self::Done,
            Self::Screen(NodeType::Collapsed(screen)) => match screen {
                Screen::Wurlitzer => Self::Done,
                Screen::Synth => Self::ScreenSynthArgs(NodeType::op_default()),
                Screen::Effect => Self::ScreenEffectArgs(NodeType::op_default()),
                Screen::LFO => {
                    Self::ScreenLfoNum(NodeType::SuperPos(vec![Some(LfoNum::default()), None]))
                } // todo!("write lfo cmd"),
                Screen::ModMatrix => Self::Done,
                Screen::Sequencer => {
                    Self::ScreenSeqNum(NodeType::SuperPos(vec![Some(SeqNum::default()), None]))
                }
            },
            Self::EngineType(NodeType::Collapsed(_)) => Self::Done,
            Self::EffectType(NodeType::Collapsed(_)) => Self::Done,
            Self::ScreenSynthArgs(NodeType::Collapsed(_)) => Self::Done,
            Self::ScreenEffectArgs(NodeType::Collapsed(_)) => Self::Done,
            Self::ScreenLfoNum(NodeType::Collapsed(_)) => Self::Done,
            Self::ScreenSeqNum(NodeType::Collapsed(_)) => Self::Done,
            Self::Done => Self::Done,
            _ => self.clone(),
        }
    }

    pub fn get_desc(&self) -> Vec<(String, String)> {
        match self {
            Self::Cmd(node) => node.get_desc(),
            Self::MatrixArgs(node) => node.get_desc(),
            Self::ModSrc(node) => node.get_desc(),
            Self::ModDest(node) => node.get_desc(),
            Self::ModAmt(node) => node.get_desc(),
            Self::EffectArgs(node) => node.get_desc(),
            Self::EngineArgs(node) => node.get_desc(),
            Self::PowerState(node) => node.get_desc(),
            Self::GenericParam(node) => node.get_desc(),
            Self::KnobValue(node) => node.get_desc(),
            Self::Screen(node) => node.get_desc(),
            Self::EngineType(node) => node.get_desc(),
            Self::EffectType(node) => node.get_desc(),
            Self::ScreenSynthArgs(node) => node.get_desc(),
            Self::ScreenEffectArgs(node) => node.get_desc(),
            Self::ScreenLfoNum(node) => node.get_desc(),
            Self::ScreenSeqNum(node) => node.get_desc(),
            Self::Done => Vec::new(),
        }
    }

    pub fn collapse(&mut self, selection: &str) {
        // let children = self.valid_children();
        let selection = selection.to_lowercase();

        match *self {
            // Self::Cmd(NodeType::Collapsed(cmd)) => match cmd {
            //
            // },
            Self::Cmd(ref mut node) => match selection.as_str() {
                "engine" => *node = NodeType::Collapsed(MainCmd::Engine),
                "effect" => *node = NodeType::Collapsed(MainCmd::Effect),
                "lfo" => {
                    *node = NodeType::Collapsed(MainCmd::Lfo);
                    todo!("design LFO command interface")
                }
                "matrix" | "mod" | "mod-matrix" => *node = NodeType::Collapsed(MainCmd::Matrix),
                "goto" | "go-to" => *node = NodeType::Collapsed(MainCmd::GoTo),
                _ => {}
            },
            Self::MatrixArgs(ref mut node) => match selection.as_str() {
                "connect" | "con" => *node = NodeType::Collapsed(MatrixCmdArgs::Connect),
                "disconnect" | "dcon" => *node = NodeType::Collapsed(MatrixCmdArgs::Disconnect),
                _ => {}
            },
            Self::ModSrc(ref mut _node) => match selection.as_str() {
                _ => todo!("generate a list of modulation sources"),
            },
            Self::ModDest(ref mut _node) => match selection.as_str() {
                _ => todo!("Generate a list opf modulation destinations"),
            },
            Self::ModAmt(ref mut node) => match selection.as_str() {
                num => {
                    if let Ok(n) = num.parse::<f32>() {
                        *node = NodeType::Collapsed(ModAmt(n))
                    } else {
                        // Error Ocurred
                    }
                }
            },
            Self::EffectArgs(ref mut node) => match selection.as_str() {
                "set" => *node = NodeType::Collapsed(EffectCmdArgs::Set),
                "switch" | "switch-to" | "to" => {
                    *node = NodeType::Collapsed(EffectCmdArgs::SwitchTo)
                }
                _ => {}
            },
            Self::EngineArgs(ref mut node) => match selection.as_str() {
                "set" => *node = NodeType::Collapsed(EngineCmdArgs::Set),
                "switch" | "switch-to" | "to" => {
                    *node = NodeType::Collapsed(EngineCmdArgs::SwitchTo)
                }
                _ => {}
            },
            Self::PowerState(ref mut node) => match selection.as_str() {
                "on" | "true" | "1" => *node = NodeType::Collapsed(PowerState::On),
                "off" | "false" | "0" => *node = NodeType::Collapsed(PowerState::On),
                _ => {}
            },
            Self::GenericParam(ref mut node) => match selection.as_str() {
                "k1" | "1" => *node = NodeType::Collapsed(GenericParam::Knob(Knob::K1)),
                "k2" | "2" => *node = NodeType::Collapsed(GenericParam::Knob(Knob::K2)),
                "k3" | "3" => *node = NodeType::Collapsed(GenericParam::Knob(Knob::K3)),
                "k4" | "4" => *node = NodeType::Collapsed(GenericParam::Knob(Knob::K4)),
                "k5" | "5" => *node = NodeType::Collapsed(GenericParam::Knob(Knob::K5)),
                "k6" | "6" => *node = NodeType::Collapsed(GenericParam::Knob(Knob::K6)),
                "k7" | "7" => *node = NodeType::Collapsed(GenericParam::Knob(Knob::K7)),
                "k8" | "8" => *node = NodeType::Collapsed(GenericParam::Knob(Knob::K8)),
                "g1" | "9" => *node = NodeType::Collapsed(GenericParam::Gui(Gui::G1)),
                "g2" | "10" => *node = NodeType::Collapsed(GenericParam::Gui(Gui::G2)),
                "g3" | "11" => *node = NodeType::Collapsed(GenericParam::Gui(Gui::G3)),
                "g4" | "12" => *node = NodeType::Collapsed(GenericParam::Gui(Gui::G4)),
                "g5" | "13" => *node = NodeType::Collapsed(GenericParam::Gui(Gui::G5)),
                "g6" | "14" => *node = NodeType::Collapsed(GenericParam::Gui(Gui::G6)),
                "g7" | "15" => *node = NodeType::Collapsed(GenericParam::Gui(Gui::G7)),
                "g8" | "16" => *node = NodeType::Collapsed(GenericParam::Gui(Gui::G8)),
                _ => {}
            },
            Self::KnobValue(ref mut node) => {
                if let Ok(n) = selection.parse::<f32>() {
                    *node = NodeType::Collapsed(KnobValue(n))
                } else {
                    // Error Ocurred
                }
            }
            Self::Screen(ref mut node) => match selection.as_str() {
                "wurlitzer" | "wurli" => *node = NodeType::Collapsed(Screen::Wurlitzer),
                "synth" | "synths" => *node = NodeType::Collapsed(Screen::Synth),
                "lfo" => *node = NodeType::Collapsed(Screen::LFO),
                "matrix" | "mod" | "mod-matrix" => *node = NodeType::Collapsed(Screen::ModMatrix),
                "effect" | "effects" => *node = NodeType::Collapsed(Screen::Effect),
                "midi" | "seq" | "sequencer" => *node = NodeType::Collapsed(Screen::Sequencer),
                _ => {}
            },
            Self::EngineType(ref mut node) => match selection.as_str() {
                "organ" => *node = NodeType::Collapsed(EngineType::Organ),
                "synth" | "subtractive-synth" => {
                    *node = NodeType::Collapsed(EngineType::SubtractiveSynth)
                }
                _ => {}
            },
            Self::EffectType(ref mut node) => match selection.as_str() {
                "reverb" | "verb" => *node = NodeType::Collapsed(EffectType::Reverb),
                "delay" | "chrous" => *node = NodeType::Collapsed(EffectType::Chorus),
                _ => {}
            },
            Self::ScreenSynthArgs(ref mut node) => match selection.as_str() {
                "organ" => *node = NodeType::Collapsed(Some(EngineType::Organ)),
                "synth" | "subtractive-synth" => {
                    *node = NodeType::Collapsed(Some(EngineType::SubtractiveSynth))
                }
                _ => {}
            },
            Self::ScreenEffectArgs(ref mut node) => match selection.as_str() {
                "reverb" | "verb" => *node = NodeType::Collapsed(Some(EffectType::Reverb)),
                "delay" | "chrous" => *node = NodeType::Collapsed(Some(EffectType::Chorus)),
                _ => {}
            },
            Self::ScreenLfoNum(ref mut node) => {
                if let Ok(n) = selection.parse::<usize>() {
                    *node = NodeType::Collapsed(Some(LfoNum(n)))
                } else {
                    // Error Ocurred
                }
            }
            Self::ScreenSeqNum(ref mut node) => {
                if let Ok(n) = selection.parse::<usize>() {
                    *node = NodeType::Collapsed(Some(SeqNum(n)))
                } else {
                    // Error Ocurred
                }
            }
            Self::Done => {}
        }
    }
}
