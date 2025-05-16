use leptos::prelude::*;

pub mod channel_editor;
// pub mod organ;
// pub mod reverb;
// pub mod sub_synth;
// pub mod wurlitzer;

// struct SynthScreen;
//
// impl ApiPage for SynthScreen {
//     fn show(b64_state: ReadSignal<String>) -> impl IntoView {
//         let synth_state = move || -> SynthEngineState {
//             bincode::serde::decode_from_slice(
//                 &BASE64_STANDARD_NO_PAD
//                     .decode(b64_state.get().as_bytes())
//                     .unwrap(),
//                 bincode::config::standard(),
//             )
//             .unwrap()
//             .0
//         };
//
//         // let display_state = move || format!("{}", synth_state().engine);
//
//         view! {
//             { move ||
//                 match synth_state().engine {
//                     // SynthEngineType::B3Organ => view! { <organ::OrganDisplay get_state=synth_state/> }.into_any(),
//                     // SynthEngineType::SubSynth => view! { <sub_synth::SubSynthDisplay get_state=synth_state/> }.into_any(),
//                     // SynthEngineType::Wurlitzer => view! { <wurlitzer::WurlitzerDisplay get_state=synth_state/> }.into_any(),
//                     // SynthEngineType::WaveTable => view! { <div> "TODO" </div> }.into_any(),
//                     // SynthEngineType::MidiOut => view! { <div> "TODO" </div> }.into_any(),
//                     _ => view! { <div> "TODO" </div> }.into_any(),
//                 }
//             }
//         }
//     }
// }

// struct EffectScreen;
//
// impl ApiPage for EffectScreen {
//     fn show(b64_state: ReadSignal<String>) -> impl IntoView {
//         let synth_state = move || -> crate::SynthEffectState {
//             bincode::serde::decode_from_slice(
//                 &BASE64_STANDARD_NO_PAD
//                     .decode(b64_state.get().as_bytes())
//                     .unwrap(),
//                 bincode::config::standard(),
//             )
//             .unwrap()
//             .0
//         };
//
//         // let display_state = move || format!("{}", synth_state().engine);
//
//         view! {
//             { move ||
//                 match synth_state().effect {
//                     // EffectType::Reverb => view! { <reverb::ReverbDisplay get_state=synth_state/> }.into_any(),
//                     // EffectType::Chorus => view! { <UnderConstruction/> }.into_any(),
//                     _ => view! { <UnderConstruction/> }.into_any(),
//                 }
//             }
//         }
//     }
// }

#[component]
pub fn UnderConstruction() -> impl IntoView {
    view! {
        <p> "its a synth bro..." </p>
        <div> "under construction check back later" </div>
    }
}

// #[component]
// pub fn SynthPage() -> impl IntoView {
//     let UseEventSourceReturn { data, .. } =
//         // use_event_source::<String, FromToStringCodec>("http://127.0.0.1:3000/synth-state/engine");
//         use_event_source::<String, FromToStringCodec>("/synth-state/engine");
//
//     view! {
//         {
//             SynthScreen::display(data)
//         }
//     }
// }
//
// #[component]
// pub fn EffectPage() -> impl IntoView {
//     let UseEventSourceReturn { data, .. } =
//         use_event_source::<String, FromToStringCodec>("/synth-state/effect");
//
//     view! {
//         {
//             EffectScreen::display(data)
//         }
//     }
// }
