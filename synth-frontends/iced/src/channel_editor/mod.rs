use crate::Message;
use iced::{
    Length::{Fill, FillPortion},
    widget::{Column, Text, column, row},
};
use stepper_synth::{effects::EffectsModule, sequencer::SequenceChannel, synth_engines::Synth};

pub fn channel_editor<'a>(synth: &Synth) -> Column<'a, Message> {
    let get_effect_name = |effect: &EffectsModule| match effect {
        EffectsModule::Chorus(_) => "Chorus",
        EffectsModule::Reverb(_) => "Reverb",
    };

    let mk_channel = |channel: SequenceChannel| {
        let lable = Text::new(match channel {
            SequenceChannel::A => "A",
            SequenceChannel::B => "B",
            SequenceChannel::C => "C",
            SequenceChannel::D => "D",
        })
        .center();
        let chan = &synth.channels[channel as usize];
        let sound_src = Text::new(chan.engine_type.to_string()).center();
        let effect_1 = if let Some(ref effect) = chan.effects[0] {
            Text::new(get_effect_name(&effect.0))
        } else {
            Text::new("None")
        }
        .center();
        let effect_2 = if let Some(ref effect) = chan.effects[1] {
            Text::new(get_effect_name(&effect.0))
        } else {
            Text::new("None")
        }
        .center();

        row![
            lable
                .width(FillPortion(25))
                .height(FillPortion(25))
                .center(),
            sound_src
                .width(FillPortion(25))
                .height(FillPortion(25))
                .center(),
            effect_1
                .width(FillPortion(25))
                .height(FillPortion(25))
                .center(),
            effect_2
                .width(FillPortion(25))
                .height(FillPortion(25))
                .center()
        ]
    };

    column![
        mk_channel(SequenceChannel::A).height(Fill),
        mk_channel(SequenceChannel::B).height(Fill),
        mk_channel(SequenceChannel::C).height(Fill),
        mk_channel(SequenceChannel::D).height(Fill),
    ]
    .width(Fill)
    .height(Fill)
    // .spacing(80)
}
