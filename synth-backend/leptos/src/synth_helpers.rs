// use actix_web::rt::spawn;
use anyhow::{bail, Result};
use log::*;
use midi_control::{Channel, MidiMessage};
use midir::{os::unix::VirtualOutput, Ignore, MidiInput, MidiOutput, PortInfoError};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use stepper_synth_backend::{pygame_coms::SynthEngineType, MidiControlled};
use stepper_synth_backend::{sequencer::SequencerIntake, synth_engines::Synth, HashMap};

const WURLITZER_CHANNEL: Channel = Channel::Ch3;

pub async fn run_midi(
    seq: actix_web::web::Data<Mutex<SequencerIntake>>,
    synth: actix_web::web::Data<Mutex<Synth>>,
    // updated: Arc<Mutex<bool>>,
    exit: Arc<AtomicBool>,
    // effect_midi: Arc<AtomicBool>,
) -> Result<()> {
    let mut registered_ports = HashMap::default();
    let midi_out = MidiOutput::new("VirtualOutput")?;
    let wurli_port_name = "wurlitzer";
    let Ok(wurlitzer) = midi_out.create_virtual(wurli_port_name) else {
        bail!("failed to create a virtual midi port for the wurlitzer")
    };
    let wurli = Arc::new(Mutex::new(wurlitzer));
    // let mut wurli = midi_out.connect(&out_port, )?;

    while !exit.load(Ordering::Relaxed) {
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

            if port_name.starts_with("VirtualOutput") {
                continue;
            }

            if registered_ports.contains_key(&port_name) {
                continue;
            }

            info!("port {port_name}");
            let mut midi_in = MidiInput::new("midir reading input")?;
            midi_in.ignore(Ignore::None);
            let synth = synth.clone();
            // let tx = tx.clone();
            // let updated = updated.clone();
            // let effect = effect_midi.clone();
            let seq = seq.clone();
            // let wurli = wurli.clone();
            // let name = port_name.clone();

            registered_ports.insert(
                port_name,
                midi_in.connect(
                    in_port,
                    "midir-read-input",
                    move |_stamp, msg, _| {
                        let message = MidiMessage::from(msg);
                        // let wurli_focused =
                        //     synth.lock().unwrap().engine_type == SynthEngineType::Wurlitzer;

                        // match message {
                        //     MidiMessage::NoteOn(channel, _)
                        //     | MidiMessage::NoteOff(channel, _)
                        //     | MidiMessage::PitchBend(channel, _, _)
                        //     | MidiMessage::ControlChange(channel, _)
                        //         // if channel == WURLITZER_CHANNEL || wurli_focused
                        //         =>
                        //     {
                        //         // println!("sending midi message {message:?} on port {name}");
                        //         _ = wurli.lock().unwrap().send(msg);
                        //     }
                        //     _ => {
                        //         synth.lock().unwrap().midi_input(&message);
                        //     }
                        // }

                        synth.lock().unwrap().midi_input(&message);

                        let mut seq = seq.lock().unwrap();

                        if seq.state.recording {
                            seq.midi_input(&message);
                        }
                    },
                    (),
                ),
            );
        }
    }

    Ok(())
}
