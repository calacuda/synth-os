use actix::prelude::*;
use serde::{Deserialize, Serialize};

pub type MidiNote = u8;
pub type Velocity = u8;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SynthType {
    Organ,
    Subtractive,
    VitalWurlitzer,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Screen {
    Synths,
    Effects,
    Lfos,
    ModMatrix,
    Sequencer,
    Vital,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Message)]
#[rtype(result = "()")]
pub enum MidiToBackend {
    NodeOn {
        note: MidiNote,
        vel: Velocity,
        channel: u8,
    },
    NodeOff {
        note: MidiNote,
        channel: u8,
    },
    CC {
        code: u8,
        data: u8,
        channel: u8,
    },
    PitchBend {
        amt: i16,
        channel: u8,
    },
}
