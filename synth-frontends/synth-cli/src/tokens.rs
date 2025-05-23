use serde::{Deserialize, Serialize};
use std::{fmt::Display, sync::Arc};
use strum::{Display, EnumIter, IntoEnumIterator};

// use crate::{CanEnumIter, CmdToken};

pub type Node = Box<dyn CmdToken>;
pub type Cmd = Vec<Node>;

// pub trait CanEnumIter {
//     fn into_vec() -> Vec<impl CmdToken>
//     where
//         Self: Sized;
// }

// impl<T> CanEnumIter for T
// where
//     T: IntoEnumIterator + CmdToken,
// {
//     fn into_vec() -> Vec<>
//     where
//         Self: Sized,
//     {
//         Self::iter().collect()
//     }
// }

pub trait CmdToken: std::fmt::Debug {
    // fn get_hildren() -> Vec<Self>;
    fn get_desc() -> Vec<(String, String)>
    where
        Self: Sized,
    {
        Self::into_vec()
            .into_iter()
            .map(|token| (token.get_desc_name().join(" | "), token.get_one_desc()))
            .collect()
    }
    fn get_one_desc(&self) -> String;
    fn get_desc_name(&self) -> Arc<[&str]>;

    // fn match_str(against: &str) -> (Vec<(String, String)>, bool);
    // /// semi-recursive function to add the string repr of self to,
    // fn render(&self, cmd: &mut Vec<String>) -> Vec<String>;
    // fn call(&self);

    fn into_vec() -> Vec<Self>
    where
        Self: Sized;

    fn get_sugestions(&self, tokens: &[NodeType]) -> Vec<Node>;
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct Float(f32);

impl CmdToken for Float {
    fn get_sugestions(&self, tokens: &[NodeType]) -> Vec<Node> {
        Vec::new()
    }

    fn get_desc_name(&self) -> Arc<[&str]> {
        ["Float"].into()
    }

    fn get_one_desc(&self) -> String {
        "decimal amount".into()
    }

    fn into_vec() -> Vec<Self>
        where
            Self: Sized {
        vec![Self(0.0)]
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

    fn get_desc_name(&self) -> Arc<[&str]> {
        match *self {
            Self::Wurlitzer => ["wurlitzer", "wurli"].into(),
            Self::Synth => ["synth"].into(),
            Self::Effect => ["effects", "effect", "ef"].into(),
            Self::LFO => ["lfo"].into(),
            Self::ModMatrix => ["mod", "matrix", "mod-matrix"].into(),
            Self::Sequencer => ["sequencer", "seq"].into(),
        }
    }

    fn into_vec() -> Vec<Self>
        where
            Self: Sized {
        Self::iter().collect()
    }

    fn get_sugestions(&self, tokens: &[NodeType]) -> Vec<Node> {
        Vec::new()
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

    fn get_desc_name(&self) -> Arc<[&str]> {
        match *self {
            Self::G1 => ["g1", "9"],
            Self::G2 => ["g2", "10"],
            Self::G3 => ["g3", "11"],
            Self::G4 => ["g4", "12"],
            Self::G5 => ["g5", "13"],
            Self::G6 => ["g6", "14"],
            Self::G7 => ["g7", "15"],
            Self::G8 => ["g8", "16"],
        }.into()
    }

    fn into_vec() -> Vec<Self>
        where
            Self: Sized {
        Self::iter().collect()
    }

    fn get_sugestions(&self, tokens: &[NodeType]) -> Vec<Node> {
        Float::into_vec().into_iter().map(|f| Box::new(f).into()).collect()
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

    fn get_desc_name(&self) -> Arc<[&str]> {
        match *self {
            Self::Knob(knob) => knob.get_desc_name(),
            Self::Gui(gui) => gui.get_desc_name(),
        }
    }

    fn into_vec() -> Vec<Self>
        where
            Self: Sized {
        Self::iter()
    }

    fn get_sugestions(&self, tokens: &[NodeType]) -> Vec<Node> {
        Vec::new()
    }
}

// impl CanEnumIter for GenericParam {
//     fn into_vec() -> Vec<Self> {
//         Self::iter()
//     }
// }

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
            Self::On => "Makes the thing \"On\"".into(),
            Self::Off => "Makes the thing \"off\"".into(),
        }
    }

    fn get_desc_name(&self) -> Arc<[&str]> {
        match *self {
            Self::On => ["on", "1"],
            Self::Off => ["off", "0"],
        }.into()
    }

    fn into_vec() -> Vec<Self>
        where
            Self: Sized {
        Self::iter().collect()
    }

    fn get_sugestions(&self, tokens: &[NodeType]) -> Vec<Node> {
        // let ctx = tokens[0];
        //
        // match (ctx, *self) {
        //     (NodeType::Known(CmdContext::Reverb), Self::On) =>
        // }
        Vec::new()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter, Display)]
pub enum ModSrc {}

impl CmdToken for ModSrc {
    fn get_one_desc(&self) -> String {
        match *self {
            _ => String::new()
        }
    }

    fn get_desc_name(&self) -> Arc<[&str]> {
        match *self {
            _ => []
        }
    }

    fn into_vec() -> Vec<Self>
        where
            Self: Sized {
        Self::iter().collect()
    }

    fn get_sugestions(&self, tokens: &[NodeType]) -> Vec<Node> {
        Vec::new()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter, Display)]
pub enum ModDest {}

impl CmdToken for ModDest {
    fn get_one_desc(&self) -> String {
        match *self {
            _ => String::new()
        }
    }

    fn get_desc_name(&self) -> Arc<[&str]> {
        match *self {
            _ => []
        }
    }

    fn into_vec() -> Vec<Self>
        where
            Self: Sized {
        Self::iter().collect()
    }

    fn get_sugestions(&self, tokens: &[NodeType]) -> Vec<Node> {
        Vec::new()
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

    fn get_desc_name(&self) -> Arc<[&str]> {
        match *self {
            Self::Connect => ["con", "connect"],
            Self::Disconnect => ["dcon", "disconnect", "discon"],
        }
    }

    fn into_vec() -> Vec<Self>
        where
            Self: Sized {
        Self::iter().collect()
    }

    fn get_sugestions(&self, tokens: &[NodeType]) -> Vec<Node> {
        ModSrc::into_vec()
    }
}

// #[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
// pub struct ModAmt(f32);
//
// impl Display for ModAmt {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "ModAmt")
//     }
// }
//
// impl CmdToken for ModAmt {
//     fn get_one_desc(&self) -> String {
//         "Modulation amount".into()
//     }
// }

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct KnobValue(f32);

impl Display for KnobValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KnobValue")
    }
}

impl CmdToken for KnobValue {
    fn get_one_desc(&self) -> String {
        "The knob's position".into()
    }

    fn get_desc_name(&self) -> Arc<[&str]> {
        ["number"]
    }

    fn into_vec() -> Vec<Self>
        where
            Self: Sized {
        vec![Self(0.0)]
    }

    fn get_sugestions(&self, tokens: &[NodeType]) -> Vec<Node> {
        Vec::new()
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct LfoNum(usize);

impl Display for LfoNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

// impl CmdToken for LfoNum {
//     fn get_one_desc(&self) -> String {
//         "Which LFO?".into()
//     }
// }

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct SeqNum(usize);

impl Display for SeqNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SeqNum")
    }
}

impl CmdToken for SeqNum {
    fn get_one_desc(&self) -> String {
        "Which MIDI sequence".into()
    }

    fn get_desc_name(&self) -> Arc<[&str]> {
        ["sequence"]
    }

    fn into_vec() -> Vec<Self>
        where
            Self: Sized {
        vec![Self(0)]
    }

    fn get_sugestions(&self, tokens: &[NodeType]) -> Vec<Node> {
        Vec::new()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter, Display)]
pub enum OrganDrawBarParam {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
}

impl CmdToken for OrganDrawBarParam {
    fn get_one_desc(&self) -> String {
        match *self {
            Self::Level1 => "Draw bar value of 1",
            Self::Level2 => "Draw bar value of 2",
            Self::Level3 => "Draw bar value of 3",
            Self::Level4 => "Draw bar value of 4",
            Self::Level5 => "Draw bar value of 5",
            Self::Level6 => "Draw bar value of 6",
            Self::Level7 => "Draw bar value of 7",
            Self::Level8 => "Draw bar value of 8",
        }.into()
    }

    fn get_desc_name(&self) -> Arc<[&str]> {
        match *self {
            Self::Level1 => ["1"],
            Self::Level2 => ["2"],
            Self::Level3 => ["3"],
            Self::Level4 => ["4"],
            Self::Level5 => ["5"],
            Self::Level6 => ["6"],
            Self::Level7 => ["7"],
            Self::Level8 => ["8"],
        }
    }

    fn into_vec() -> Vec<Self>
        where
            Self: Sized {
        Self::iter().collect()
    }

    fn get_sugestions(&self, tokens: &[NodeType]) -> Vec<Node> {
        Vec::new()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter, Display)]
pub enum OrganParam {
    Db1,
    Db2,
    Db3,
    Db4,
    Db5,
    Db6,
    Db7,
    Db8,
    SpeakerSpeed,
}

impl CmdToken for OrganParam {
    fn get_one_desc(&self) -> String {
        match *self {
            Self::Db1 => "Draw bar 1".into(),
            Self::Db2 => "Draw bar 2".into(),
            Self::Db3 => "Draw bar 3".into(),
            Self::Db4 => "Draw bar 4".into(),
            Self::Db5 => "Draw bar 5".into(),
            Self::Db6 => "Draw bar 6".into(),
            Self::Db7 => "Draw bar 7".into(),
            Self::Db8 => "Draw bar 8".into(),
            Self::SpeakerSpeed => "spead of the leslie speaker".into(),
        }
    }

    fn get_desc_name(&self) -> Arc<[&str]> {
        match *self {
            Self::SpeakerSpeed => ["speed", "rpm"],
            Self::Db1 => ["1"],
            Self::Db2 => ["2"],
            Self::Db3 => ["3"],
            Self::Db4 => ["4"],
            Self::Db5 => ["5"],
            Self::Db6 => ["6"],
            Self::Db7 => ["7"],
            Self::Db8 => ["8"],
        }
    }

    fn into_vec() -> Vec<Self>
        where
            Self: Sized {
        Self::iter().collect()
    }

    fn get_sugestions(&self, tokens: &[NodeType]) -> Vec<Node> {
        match *self {
            Self::SpeakerSpeed => Float::into_vec(),
            Self::Db1 => OrganDrawBarParam::into_vec(),
            Self::Db2 => OrganDrawBarParam::into_vec(),
            Self::Db3 => OrganDrawBarParam::into_vec(),
            Self::Db4 => OrganDrawBarParam::into_vec(),
            Self::Db5 => OrganDrawBarParam::into_vec(),
            Self::Db6 => OrganDrawBarParam::into_vec(),
            Self::Db7 => OrganDrawBarParam::into_vec(),
            Self::Db8 => OrganDrawBarParam::into_vec(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter, Display)]
pub enum SemitoneDetune {
    Semi1,
    Semi2,
    Semi3,
    Semi4,
    Semi5,
    Semi6,
    Semi7,
    Semi8,
    Semi9,
    Semi10,
    Semi11,
    Semi12,
}

impl CmdToken for SemitoneDetune {
    fn get_one_desc(&self) -> String {
        match *self {
            Self::Semi1 => "1 semi-tone",
            Self::Semi2 => "2 semi-tone",
            Self::Semi3 => "3 semi-tone",
            Self::Semi4 => "4 semi-tone",
            Self::Semi5 => "5 semi-tone",
            Self::Semi6 => "6 semi-tone",
            Self::Semi7 => "7 semi-tone",
            Self::Semi8 => "8 semi-tone",
            Self::Semi9 => "9 semi-tone",
            Self::Semi10 => "10 semi-tone",
            Self::Semi11 => "11 semi-tone",
            Self::Semi12 => "12 semi-tone",
        }.into()
    }

    fn get_desc_name(&self) -> Arc<[&str]> {
        match *self {
            Self::Semi1 => ["1"],
            Self::Semi2 => ["2"],
            Self::Semi3 => ["3"],
            Self::Semi4 => ["4"],
            Self::Semi5 => ["5"],
            Self::Semi6 => ["6"],
            Self::Semi7 => ["7"],
            Self::Semi8 => ["8"],
            Self::Semi9 => ["9"],
            Self::Semi10 => ["10"],
            Self::Semi11 => ["11"],
            Self::Semi12 => ["12"],
        }
    }

    fn into_vec() -> Vec<Self>
        where
            Self: Sized {
        Self::iter().collect()
    }

    fn get_sugestions(&self, tokens: &[NodeType]) -> Vec<Node> {

    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter, Display)]
pub enum WaveformType {
    Sine,
    Saw,
}

impl CmdToken for WaveformType {
    fn get_desc_name(&self) -> Arc<[&str]> {
        match *self {
            Self::Sine => ["sine", "sin"],
            Self::Saw => ["saw", "saw-tooth"],
        }
    }

    fn get_one_desc(&self) -> String {
        match *self {
            Self::Sine => "Sine waveform",
            Self::Saw => "SawTooth waveform"
        }.into()
    }

    fn into_vec() -> Vec<Self>
        where
            Self: Sized {
        Self::iter().collect()
    }

    fn get_sugestions(&self, tokens: &[NodeType]) -> Vec<Node> {
        Vec::new()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter, Display)]
pub enum SubSynthParam {
    /// the waveform type for osc one
    Osc1Type,
    /// how much of osc 1 vs osc 2 gets heard
    Mix,
    /// the waveform type for osc two
    Osc2Type,
    /// how many steps out of tune is osc 2
    Detune,
    /// what percentage of a note is osc 2 out of tune
    DetuneFine,
    /// the attack of the adsr filter
    Atk,
    /// the decay of the adsr
    Dcy,
    /// sustain of the adsr
    Sus,
    /// release of the adsr
    Rel,
    /// lowpass CutOff
    CutOff,
    /// lowpass resonance
    Res,
}

impl CmdToken for SubSynthParam {
    fn get_one_desc(&self) -> String {
        match *self {
            Self::Osc1Type => "Waveform type for main osc".into(),
            Self::Mix => "How much of osc 1 vs osc 2 gets heard".into(),
            Self::Osc2Type => "Waveform type for secondary osc".into(),
            Self::Detune => "How many steps out of tune is osc 2".into(),
            Self::DetuneFine => "What percentage of a note is osc 2 out of tune".into(),
            Self::Atk => "ADSR Attack".into(),
            Self::Dcy => "ADSR Decay".into(),
            Self::Sus => "ADSR Sustain".into(),
            Self::Rel => "ADSR Release".into(),
            Self::CutOff => "Low-pass filter cutoff".into(),
            Self::Res => "Low-pass filter resonance".into(),
        }
    }

    fn get_desc_name(&self) -> Arc<[&str]> {
        match *self {
            Self::Osc1Type => ["osc1-type", "main-wf"],
            Self::Mix => ["Mix"],
            Self::Osc2Type => ["osc2-type", "second-wf"],
            Self::Detune => ["detune"],
            Self::DetuneFine => ["detune-fine", "fine-tune"],
            Self::Atk => ["attack", "atk"],
            Self::Dcy => ["decay", "dcy"],
            Self::Sus => ["sustain", "sus"],
            Self::Rel => ["release", "rel"],
            Self::CutOff => ["cutoff"],
            Self::Res => ["resonance", "res"],
        }
    }

    fn into_vec() -> Vec<Self>
        where
            Self: Sized {
        Self::iter().collect()
    }

    fn get_sugestions(&self, tokens: &[NodeType]) -> Vec<Node> {
       match *self {
            Self::Osc1Type => WaveformType::into_vec(),
            Self::Mix => Float::into_vec(),
            Self::Osc2Type => WaveformType::into_vec(),
            Self::Detune => SemitoneDetune::into_vec(),
            Self::DetuneFine => Float::into_vec(),  // FinetuneDetune::into_vec(),
            Self::Atk => Float::into_vec(),
            Self::Dcy => Float::into_vec(),
            Self::Sus => Float::into_vec(),
            Self::Rel => Float::into_vec(),
            Self::CutOff => Float::into_vec(),
            Self::Res => Float::into_vec(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter, Display)]
pub enum ReverbParams {
    Gain,
    Decay,
    Cutoff,
    Damping,
    PowerState,
}

impl CmdToken for ReverbParams {
    fn get_one_desc(&self) -> String {
        match *self {
            Self::Gain => "Reverb gain",
            Self::Decay => "Reverb decay",
            Self::Cutoff => "Cutoff of the reverb's internal lowpass filter",
            Self::Damping => "Tone change of the reverb",
            Self::PowerState => "Is the reverb on or off"
        }.into()
    }

    fn get_desc_name(&self) -> Arc<[&str]> {
        match *self {
            Self::Gain => ["gain"],
            Self::Decay => ["decay"],
            Self::Cutoff => ["cutoff"],
            Self::Damping => ["tone", "damping"],
            Self::PowerState => ["power"],
        }.into()
    }

    fn into_vec() -> Vec<Self>
        where
            Self: Sized {
        Self::iter().collect()
    }

    fn get_sugestions(&self, tokens: &[NodeType]) -> Vec<Node> {
        match *self {
            Self::Gain => Float::into_vec(),
            Self::Decay => Float::into_vec(),
            Self::Cutoff => Float::into_vec(),
            Self::Damping => Float::into_vec(),
            Self::PowerState => PowerState::iter(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, EnumIter, Display)]
pub enum CmdContext {
    /// set organ params
    Organ,
    /// set params for the subtractive synth,
    SubSynth,
    /// set reverb effect params
    Reverb,
    /// set chourus effect params
    Chorus,
    /// chagne LFO parameters
    Lfo,
    /// make-new/break-old connections in the mod matrix
    Matrix, // (MatrixCmdArgs),
    /// Goto display
    GoTo, // (Screen),
}

impl CmdToken for CmdContext {
    fn get_one_desc(&self) -> String {
        match *self {
            // Self::Engine => "Synth engine & their params".into(),
            // Self::Effect => "Audio effect & their params".into(),
            Self::Organ => "Organ synth engine params".into(),
            Self::SubSynth => "Subtractive synth engine params".into(),
            Self::Reverb => "Reverb effect params".into(),
            Self::Chorus => "Chorus effect params".into(),
            Self::Lfo => "Controls over the LFOs".into(),
            Self::Matrix => "Edit the mod-matrix".into(),
            Self::GoTo => "Change the active screen".into(),
        }
    }

    fn get_desc_name(&self) -> Arc<[&str]> {
        match *self {
            Self::Organ => ["organ", "org"].into(),
            Self::SubSynth => ["synth"].into(),
            Self::Reverb => ["reverb", "verb"].into(),
            Self::Chorus => ["chorus"].into(),
            Self::Lfo => ["lfo"].into(),
            Self::Matrix => ["mod-matrix", "patch", "mod-m"].into(),
            Self::GoTo => ["goto", "screen", "view"].into(),
        }
    }

    fn into_vec() -> Vec<Self>
    where
        Self: Sized,
    {
        Self::iter().collect()
    }

    fn get_sugestions(&self, tokens: &[NodeType]) -> Vec<Node> {
        match tokens {
            [] => Self::into_vec(),
            [NodeType::Known(Self::Organ)] => OrganParam::into_vec()
            [NodeType::Known(Self::SubSynth)] => SubSynthParam::into_vec(),
            [NodeType::Known(Self::Lfo)] => [].to_vec(),
            [NodeType::Known(Self::Reverb)] => ReverbParams::into_vec(),
            [NodeType::Known(Self::Chorus)] => [].to_vec(),
            [NodeType::Known(Self::Matrix)] => MatrixCmdArgs::into_vec(),  // [].to_vec(),
            [NodeType::Known(Self::GoTo)] => GoToParams::into_vec(),  // [].to_vec(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeType {
    Known(Node),
    Unknown(String),
}

#[derive(Debug)]
pub struct Command {
    pub tokens: Vec<NodeType>,
}

impl Command {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    /// returns an api path
    pub fn to_api(&self) -> String {
        todo!();
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub enum NodeType<T> {
//     SuperPos(Vec<T>),
//     Collapsed(T),
// }
//
// impl<T> NodeType<T>
// where
//     T: std::fmt::Display + CmdToken,
// {
//     fn get_desc(&self) -> Vec<(String, String)> {
//         match self.clone() {
//             Self::Collapsed(node) => vec![(format!("{node}"), node.get_one_desc())],
//             Self::SuperPos(nodes) => nodes
//                 .iter()
//                 .map(|node| (format!("{node}"), node.get_one_desc()))
//                 .collect(),
//         }
//     }
// }
//
// impl<T> NodeType<Option<T>>
// where
//     T: std::fmt::Display + CmdToken,
// {
//     fn get_desc(&self) -> Vec<(String, String)> {
//         match self.clone() {
//             Self::Collapsed(Some(node)) => vec![(
//                 format!("{node}"),
//                 format!("Optional: {}", node.get_one_desc()),
//             )],
//             Self::Collapsed(None) => Vec::new(),
//             Self::SuperPos(nodes) => nodes
//                 .into_iter()
//                 .filter_map(|node| {
//                     node.map(|node| {
//                         (
//                             format!("{node}"),
//                             format!("Optional: {}", node.get_one_desc()),
//                         )
//                     })
//                 })
//                 .collect(),
//         }
//     }
// }
//
// impl<T> NodeType<Option<T>>
// where
//     T: IntoEnumIterator + CmdToken,
// {
//     fn op_default() -> Self {
//         let mut v: Vec<Option<T>> = T::iter().map(Some).collect();
//         v.push(None);
//
//         Self::SuperPos(v)
//     }
// }
//
// impl<T> Default for NodeType<T>
// where
//     T: IntoEnumIterator + CmdToken,
// {
//     fn default() -> Self {
//         Self::SuperPos(T::iter().collect())
//     }
// }
//
// impl Default for NodeType<GenericParam> {
//     fn default() -> Self {
//         Self::SuperPos(GenericParam::iter())
//     }
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub enum Context {
//     // command scope
//     Organ,
//     Synth,
//     Lfo,
//     Reverb,
//     Chorus,
//     Connect,
//     Disconnect,
//     GoTo,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub enum Node {
//     // command scope
//     Organ,
//     Synth,
//     Lfo,
//     Reverb,
//     Chorus,
//     Connect,
//     Disconnect,
//     GoTo,
//
//     // organ entity
//     Db1,
//     Db2,
//     Db3,
//     Db4,
//     Db5,
//     Db6,
//     Db7,
//     Db8,
//     SpeakerSpeed,
//
//     // synth entity
//     /// the waveform type for osc one
//     Osc1Type,
//     /// how much of osc 1 vs osc 2 gets heard
//     Mix,
//     /// the waveform type for osc two
//     Osc2Type,
//     /// how many steps out of tune is osc 2
//     Detune,
//     /// what percentage of a note is osc 2 out of tune
//     DetuneFine,
//     /// the attack of the adsr filter
//     Atk,
//     /// the decay of the adsr
//     Dcy,
//     /// sustain of the adsr
//     Sus,
//     /// release of the adsr
//     Rel,
//     /// lowpass CutOff
//     CutOff,
//     /// lowpass resonance
//     Res,
//
//     // lfo entities
//
//     // reverb entities
//     Gain,
//     Decay,
//     ReverbCutoff,
//     Damping,
//     ReverbOn,
//
//     Done,
// }

// impl Default for Node {
//     fn default() -> Self {
//         Self::Cmd(NodeType::SuperPos(MainCmd::iter().collect()))
//     }
// }

// impl Node {
//     pub fn get_sugestions(tokens: &[NodeType]) -> Vec<Node> {
//         match tokens {
//             [] => [
//                 Self::Organ,
//                 Self::Synth,
//                 Self::Lfo,
//                 Self::Reverb,
//                 Self::Chorus,
//                 Self::Connect,
//                 Self::Disconnect,
//                 Self::GoTo,
//             ]
//             .to_vec(),
//             [NodeType::Known(Self::Organ)] => [
//                 Self::Db1,
//                 Self::Db2,
//                 Self::Db3,
//                 Self::Db4,
//                 Self::Db5,
//                 Self::Db6,
//                 Self::Db7,
//                 Self::Db8,
//                 Self::SpeakerSpeed,
//                 Self::Done,
//             ]
//             .to_vec(),
//             [NodeType::Known(Self::Synth)] => [
//                 Self::Osc1Type,
//                 Self::Mix,
//                 Self::Osc2Type,
//                 Self::Detune,
//                 Self::DetuneFine,
//                 Self::Atk,
//                 Self::Dcy,
//                 Self::Sus,
//                 Self::Rel,
//                 Self::CutOff,
//                 Self::Res,
//             ]
//             .to_vec(),
//             [NodeType::Known(Self::Lfo)] => [].to_vec(),
//             [NodeType::Known(Self::Reverb)] => [
//                 Self::Gain,
//                 Self::Decay,
//                 Self::ReverbCutoff,
//                 Self::Damping,
//                 Self::ReverbOn,
//             ]
//             .to_vec(),
//             [NodeType::Known(Self::Chorus)] => [].to_vec(),
//             [NodeType::Known(Self::Connect)] => [].to_vec(),
//             [NodeType::Known(Self::Disconnect)] => [].to_vec(),
//             [NodeType::Known(Self::GoTo)] => [].to_vec(),
//             _ => Vec::new(),
//         }
//     }
//
//     fn parse_from_str(tokens: &[NodeType], from: &str) -> NodeType {
//         let from = from.to_lowercase();
//         let from = from.as_str();
//         let context = tokens[0];
//
//         match context {
//             [] => match from {
//                 "organ" | "org" => NodeType::Known(Self::Organ),
//                 "synth" => NodeType::Known(Self::Synth),
//                 "lfo" => NodeType::Known(Self::Lfo),
//                 "reverb" | "verb" => NodeType::Known(Self::Reverb),
//                 "chorus" => NodeType::Known(Self::Chorus),
//                 "connect" | "con" => NodeType::Known(Self::Connect),
//                 "disconnect" | "discon" | "dcon" => NodeType::Known(Self::Disconnect),
//                 "goto" | "view" | "screen" => NodeType::Known(Self::GoTo),
//             },
//         }
//     }
// }

// impl Nodes {
//     pub fn transition(&self) -> Self {
//         match self {
//             Self::Cmd(NodeType::Collapsed(cmd)) => match cmd {
//                 // MainCmd::Engine => Self::EngineArgs(NodeType::default()),
//                 // MainCmd::Effect => Self::EffectArgs(NodeType::default()),
//                 MainCmd::Organ => Self::OrganParam(NodeType::default()),
//                 MainCmd::SubSynth => Self::SubSynthParam(NodeType::default()),
//                 MainCmd::Lfo => todo!("design lfo cmd interface"),
//                 MainCmd::Matrix => Self::MatrixArgs(NodeType::default()),
//                 MainCmd::GoTo => Self::Screen(NodeType::default()),
//             },
//             Self::MatrixArgs(NodeType::Collapsed(sub_cmd)) => match sub_cmd {
//                 MatrixCmdArgs::Connect => Self::ModSrc(NodeType::default()),
//                 MatrixCmdArgs::Disconnect => {
//                     // Self::(NodeType::SuperPos(ModSrc::iter().collect()))
//                     todo!("write this");
//                 }
//             },
//             Self::ModSrc(NodeType::Collapsed(_)) => {
//                 Self::ModDest(NodeType::SuperPos(ModDest::iter().collect()))
//             }
//             Self::ModDest(NodeType::Collapsed(_)) => {
//                 Self::ModAmt(NodeType::SuperPos(vec![ModAmt::default()]))
//             }
//             Self::ModAmt(NodeType::Collapsed(_)) => Self::Done,
//             Self::EffectArgs(NodeType::Collapsed(sub_cmd)) => match sub_cmd {
//                 EffectCmdArgs::Set => Self::GenericParam(NodeType::default()),
//                 EffectCmdArgs::SwitchTo => Self::EffectType(NodeType::default()),
//             },
//             // Self::EngineArgs(NodeType::Collapsed(sub_cmd)) => match sub_cmd {
//             //     EngineCmdArgs::Set => Self:: // Self::GenericParam(NodeType::default()),
//             //     EngineCmdArgs::SwitchTo => Self::EngineType(NodeType::default()),
//             // },
//             Self::PowerState(NodeType::Collapsed(_)) => Self::Done,
//             Self::GenericParam(NodeType::Collapsed(_)) => {
//                 Self::KnobValue(NodeType::SuperPos(vec![KnobValue::default()]))
//             }
//             Self::KnobValue(NodeType::Collapsed(_)) => Self::Done,
//             Self::Screen(NodeType::Collapsed(screen)) => match screen {
//                 Screen::Wurlitzer => Self::Done,
//                 Screen::Synth => Self::ScreenSynthArgs(NodeType::op_default()),
//                 Screen::Effect => Self::ScreenEffectArgs(NodeType::op_default()),
//                 Screen::LFO => {
//                     Self::ScreenLfoNum(NodeType::SuperPos(vec![Some(LfoNum::default()), None]))
//                 } // todo!("write lfo cmd"),
//                 Screen::ModMatrix => Self::Done,
//                 Screen::Sequencer => {
//                     Self::ScreenSeqNum(NodeType::SuperPos(vec![Some(SeqNum::default()), None]))
//                 }
//             },
//             Self::EngineType(NodeType::Collapsed(_)) => Self::Done,
//             Self::EffectType(NodeType::Collapsed(_)) => Self::Done,
//             Self::ScreenSynthArgs(NodeType::Collapsed(_)) => Self::Done,
//             Self::ScreenEffectArgs(NodeType::Collapsed(_)) => Self::Done,
//             Self::ScreenLfoNum(NodeType::Collapsed(_)) => Self::Done,
//             Self::ScreenSeqNum(NodeType::Collapsed(_)) => Self::Done,
//             Self::Done => Self::Done,
//             Self::OrganParam(NodeType::Collapsed(param)) => match *param {
//                 OrganParam::SpeakerSpeed => Self::SpeakerSpeed(NodeType::default()),
//                 _ => Self::DrawBarValue(NodeType::default()),
//             },
//             _ => self.clone(),
//         }
//     }
//
//     pub fn get_desc(&self) -> Vec<(String, String)> {
//         match self {
//             Self::Cmd(node) => node.get_desc(),
//             Self::MatrixArgs(node) => node.get_desc(),
//             Self::ModSrc(node) => node.get_desc(),
//             Self::ModDest(node) => node.get_desc(),
//             Self::ModAmt(node) => node.get_desc(),
//             Self::EffectArgs(node) => node.get_desc(),
//             // Self::EngineArgs(node) => node.get_desc(),
//             Self::PowerState(node) => node.get_desc(),
//             Self::GenericParam(node) => node.get_desc(),
//             Self::KnobValue(node) => node.get_desc(),
//             Self::Screen(node) => node.get_desc(),
//             Self::EngineType(node) => node.get_desc(),
//             Self::EffectType(node) => node.get_desc(),
//             Self::ScreenSynthArgs(node) => node.get_desc(),
//             Self::ScreenEffectArgs(node) => node.get_desc(),
//             Self::ScreenLfoNum(node) => node.get_desc(),
//             Self::ScreenSeqNum(node) => node.get_desc(),
//             Self::OrganParam(node) => node.get_desc(),
//             Self::SubSynthParam(node) => node.get_desc(),
//             Self::Done => Vec::new(),
//         }
//     }
//
//     pub fn collapse(&mut self, selection: &str) {
//         // let children = self.valid_children();
//         let selection = selection.to_lowercase();
//
//         match *self {
//             // Self::Cmd(NodeType::Collapsed(cmd)) => match cmd {
//             //
//             // },
//             Self::Cmd(ref mut node) => match selection.as_str() {
//                 // "engine" => *node = NodeType::Collapsed(MainCmd::Engine),
//                 // "effect" => *node = NodeType::Collapsed(MainCmd::Effect),
//                 "organ" => *node = NodeType::Collapsed(MainCmd::Organ),
//                 "synth" => *node = NodeType::Collapsed(MainCmd::SubSynth),
//                 "lfo" => {
//                     *node = NodeType::Collapsed(MainCmd::Lfo);
//                     todo!("design LFO command interface")
//                 }
//                 "matrix" | "mod" | "mod-matrix" => *node = NodeType::Collapsed(MainCmd::Matrix),
//                 "goto" | "go-to" => *node = NodeType::Collapsed(MainCmd::GoTo),
//                 _ => {}
//             },
//             Self::MatrixArgs(ref mut node) => match selection.as_str() {
//                 "connect" | "con" => *node = NodeType::Collapsed(MatrixCmdArgs::Connect),
//                 "disconnect" | "dcon" => *node = NodeType::Collapsed(MatrixCmdArgs::Disconnect),
//                 _ => {}
//             },
//             Self::ModSrc(ref mut _node) => match selection.as_str() {
//                 _ => todo!("generate a list of modulation sources"),
//             },
//             Self::ModDest(ref mut _node) => match selection.as_str() {
//                 _ => todo!("Generate a list opf modulation destinations"),
//             },
//             Self::ModAmt(ref mut node) => match selection.as_str() {
//                 num => {
//                     if let Ok(n) = num.parse::<f32>() {
//                         *node = NodeType::Collapsed(ModAmt(n))
//                     } else {
//                         // Error Ocurred
//                     }
//                 }
//             },
//             Self::EffectArgs(ref mut node) => match selection.as_str() {
//                 "set" => *node = NodeType::Collapsed(EffectCmdArgs::Set),
//                 "switch" | "switch-to" | "to" => {
//                     *node = NodeType::Collapsed(EffectCmdArgs::SwitchTo)
//                 }
//                 _ => {}
//             },
//             // Self::EngineArgs(ref mut node) => match selection.as_str() {
//             //     "set" => *node = NodeType::Collapsed(EngineCmdArgs::Set),
//             //     "switch" | "switch-to" | "to" => {
//             //         *node = NodeType::Collapsed(EngineCmdArgs::SwitchTo)
//             //     }
//             //     _ => {}
//             // },
//             Self::PowerState(ref mut node) => match selection.as_str() {
//                 "on" | "true" | "1" => *node = NodeType::Collapsed(PowerState::On),
//                 "off" | "false" | "0" => *node = NodeType::Collapsed(PowerState::On),
//                 _ => {}
//             },
//             Self::GenericParam(ref mut node) => match selection.as_str() {
//                 "k1" | "1" => *node = NodeType::Collapsed(GenericParam::Knob(Knob::K1)),
//                 "k2" | "2" => *node = NodeType::Collapsed(GenericParam::Knob(Knob::K2)),
//                 "k3" | "3" => *node = NodeType::Collapsed(GenericParam::Knob(Knob::K3)),
//                 "k4" | "4" => *node = NodeType::Collapsed(GenericParam::Knob(Knob::K4)),
//                 "k5" | "5" => *node = NodeType::Collapsed(GenericParam::Knob(Knob::K5)),
//                 "k6" | "6" => *node = NodeType::Collapsed(GenericParam::Knob(Knob::K6)),
//                 "k7" | "7" => *node = NodeType::Collapsed(GenericParam::Knob(Knob::K7)),
//                 "k8" | "8" => *node = NodeType::Collapsed(GenericParam::Knob(Knob::K8)),
//                 "g1" | "9" => *node = NodeType::Collapsed(GenericParam::Gui(Gui::G1)),
//                 "g2" | "10" => *node = NodeType::Collapsed(GenericParam::Gui(Gui::G2)),
//                 "g3" | "11" => *node = NodeType::Collapsed(GenericParam::Gui(Gui::G3)),
//                 "g4" | "12" => *node = NodeType::Collapsed(GenericParam::Gui(Gui::G4)),
//                 "g5" | "13" => *node = NodeType::Collapsed(GenericParam::Gui(Gui::G5)),
//                 "g6" | "14" => *node = NodeType::Collapsed(GenericParam::Gui(Gui::G6)),
//                 "g7" | "15" => *node = NodeType::Collapsed(GenericParam::Gui(Gui::G7)),
//                 "g8" | "16" => *node = NodeType::Collapsed(GenericParam::Gui(Gui::G8)),
//                 _ => {}
//             },
//             Self::KnobValue(ref mut node) => {
//                 if let Ok(n) = selection.parse::<f32>() {
//                     *node = NodeType::Collapsed(KnobValue(n))
//                 } else {
//                     // Error Ocurred
//                 }
//             }
//             Self::Screen(ref mut node) => match selection.as_str() {
//                 "wurlitzer" | "wurli" => *node = NodeType::Collapsed(Screen::Wurlitzer),
//                 "synth" | "synths" => *node = NodeType::Collapsed(Screen::Synth),
//                 "lfo" => *node = NodeType::Collapsed(Screen::LFO),
//                 "matrix" | "mod" | "mod-matrix" => *node = NodeType::Collapsed(Screen::ModMatrix),
//                 "effect" | "effects" => *node = NodeType::Collapsed(Screen::Effect),
//                 "midi" | "seq" | "sequencer" => *node = NodeType::Collapsed(Screen::Sequencer),
//                 _ => {}
//             },
//             Self::EngineType(ref mut node) => match selection.as_str() {
//                 "organ" => *node = NodeType::Collapsed(EngineType::Organ),
//                 "synth" | "subtractive-synth" => {
//                     *node = NodeType::Collapsed(EngineType::SubtractiveSynth)
//                 }
//                 _ => {}
//             },
//             Self::EffectType(ref mut node) => match selection.as_str() {
//                 "reverb" | "verb" => *node = NodeType::Collapsed(EffectType::Reverb),
//                 "delay" | "chrous" => *node = NodeType::Collapsed(EffectType::Chorus),
//                 _ => {}
//             },
//             Self::ScreenSynthArgs(ref mut node) => match selection.as_str() {
//                 "organ" => *node = NodeType::Collapsed(Some(EngineType::Organ)),
//                 "synth" | "subtractive-synth" => {
//                     *node = NodeType::Collapsed(Some(EngineType::SubtractiveSynth))
//                 }
//                 _ => {}
//             },
//             Self::ScreenEffectArgs(ref mut node) => match selection.as_str() {
//                 "reverb" | "verb" => *node = NodeType::Collapsed(Some(EffectType::Reverb)),
//                 "delay" | "chrous" => *node = NodeType::Collapsed(Some(EffectType::Chorus)),
//                 _ => {}
//             },
//             Self::ScreenLfoNum(ref mut node) => {
//                 if let Ok(n) = selection.parse::<usize>() {
//                     *node = NodeType::Collapsed(Some(LfoNum(n)))
//                 } else {
//                     // Error Ocurred
//                 }
//             }
//             Self::ScreenSeqNum(ref mut node) => {
//                 if let Ok(n) = selection.parse::<usize>() {
//                     *node = NodeType::Collapsed(Some(SeqNum(n)))
//                 } else {
//                     // Error Ocurred
//                 }
//             }
//             Self::Done => {}
//         }
//     }
// }
